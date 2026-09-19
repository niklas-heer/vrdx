//! A read-only loopback view. Every API request rebuilds from the source files.

use super::{Error, Graph};
use serde_json::{Value, json};
use std::{
    io::{self, Write},
    path::Path,
};
use tiny_http::{Header, Method, Request, Response, Server, StatusCode};

fn envelope(result: Result<Value, Error>) -> (u16, String) {
    match result {
        Ok(data) => (
            200,
            json!({"schema_version":1,"ok":true,"data":data}).to_string(),
        ),
        Err(error) => {
            let status = match error.code {
                "not_found" => 404,
                "usage" | "ambiguous_id" => 400,
                "invalid_collection" => 422,
                _ => 500,
            };
            (status, json!({"schema_version":1,"ok":false,"error":{"code":error.code,"message":error.message,"hint":error.hint}}).to_string())
        }
    }
}

fn api(directory: &Path, url: &str) -> Result<Value, Error> {
    let graph = Graph::load(directory)?;
    if url == "/api/graph" {
        return Ok(json!({"valid":graph.findings.is_empty(),"graph":graph}));
    }
    let id = url
        .strip_prefix("/api/suggest?id=")
        .filter(|id| !id.is_empty() && id.bytes().all(|byte| byte.is_ascii_alphanumeric()))
        .ok_or_else(|| Error::new("usage", "Expected /api/suggest?id=ULID"))?;
    graph.require_valid()?;
    super::ai::suggest(&graph, id, 10)
}

fn trusted(request: &Request, port: u16) -> bool {
    let hosts = [format!("127.0.0.1:{port}"), format!("localhost:{port}")];
    let host_ok = request.headers().iter().any(|header| {
        header.field.equiv("Host") && hosts.iter().any(|host| host == header.value.as_str())
    });
    host_ok
        && request.headers().iter().all(|header| {
            if header.field.equiv("Origin") {
                hosts
                    .iter()
                    .any(|host| header.value.as_str() == format!("http://{host}"))
            } else if header.field.equiv("Sec-Fetch-Site") {
                matches!(header.value.as_str(), "same-origin" | "none")
            } else {
                true
            }
        })
}

fn respond(request: Request, status: u16, content_type: &str, body: String) {
    let mut response = Response::from_string(body).with_status_code(StatusCode(status));
    for (name, value) in [
        ("Content-Type", content_type),
        ("Cache-Control", "no-store"),
        ("X-Content-Type-Options", "nosniff"),
        ("Referrer-Policy", "no-referrer"),
        (
            "Content-Security-Policy",
            "default-src 'none'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; connect-src 'self'; base-uri 'none'; frame-ancestors 'none'; form-action 'none'",
        ),
    ] {
        if let Ok(header) = Header::from_bytes(name, value) {
            response.add_header(header);
        }
    }
    // A browser closing its tab must not stop the dashboard.
    let _ = request.respond(response);
}

fn handle(request: Request, directory: &Path, port: u16) {
    if !trusted(&request, port) {
        respond(
            request,
            403,
            "text/plain; charset=utf-8",
            "Local same-origin requests only.\n".into(),
        );
        return;
    }
    if request.method() != &Method::Get {
        respond(
            request,
            405,
            "text/plain; charset=utf-8",
            "Read-only dashboard; use GET.\n".into(),
        );
        return;
    }
    let (status, content_type, body) = match request.url() {
        "/" => (
            200,
            "text/html; charset=utf-8",
            include_str!("../../web/index.html").to_owned(),
        ),
        "/app.js" => (
            200,
            "text/javascript; charset=utf-8",
            include_str!("../../web/app.js").to_owned(),
        ),
        "/style.css" => (
            200,
            "text/css; charset=utf-8",
            include_str!("../../web/style.css").to_owned(),
        ),
        "/logo.svg" | "/favicon.ico" => (
            200,
            "image/svg+xml",
            include_str!("../../assets/logo.svg").to_owned(),
        ),
        url if url == "/api/graph" || url.starts_with("/api/suggest?") => {
            let (status, body) = envelope(api(directory, url));
            (status, "application/json; charset=utf-8", body)
        }
        _ => (404, "text/plain; charset=utf-8", "Not found.\n".into()),
    };
    respond(request, status, content_type, body);
}

pub(super) fn serve(directory: &Path, port: u16, json_output: bool) -> Result<(), Error> {
    let directory = directory.canonicalize()?;
    // Fail before announcing a URL if the directory cannot be read. Invalid records
    // are still browsable with visible findings.
    Graph::load(&directory)?;
    let server =
        Server::http(("127.0.0.1", port)).map_err(|error| Error::new("io", error.to_string()))?;
    let address = server
        .server_addr()
        .to_ip()
        .ok_or_else(|| Error::new("io", "Dashboard did not bind a TCP socket"))?;
    let url = format!("http://{address}");
    let startup = if json_output {
        json!({"schema_version":1,"ok":true,"data":{"url":url,"read_only":true,"directory":directory}}).to_string()
    } else {
        format!(
            "vrdx dashboard → {url}\nReading {} · read-only · Ctrl+C to stop",
            directory.display()
        )
    };
    {
        let mut stdout = io::stdout().lock();
        writeln!(stdout, "{startup}")?;
        stdout.flush()?;
    }
    loop {
        let request = server.recv()?;
        handle(request, &directory, address.port());
    }
}
