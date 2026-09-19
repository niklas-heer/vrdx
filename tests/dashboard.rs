//! Real HTTP journeys against the bundled dashboard, including installed binaries.
#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "Fixture failures should fail tests immediately"
)]

use serde_json::Value;
use std::{
    fs,
    io::{BufRead, BufReader, Read, Write},
    net::TcpStream,
    path::Path,
    process::{Child, Command, Stdio},
    sync::mpsc,
    time::Duration,
};
use tempfile::TempDir;

struct Dashboard {
    child: Child,
    address: String,
}
impl Dashboard {
    fn start(directory: &Path) -> Self {
        let binary = std::env::var_os("VRDX_TEST_BINARY")
            .unwrap_or_else(|| env!("CARGO_BIN_EXE_vrdx").into());
        let child = Command::new(binary)
            .arg("--dir")
            .arg(directory)
            .args(["dashboard", "--port", "0", "--json"])
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        let mut server = Self {
            child,
            address: String::new(),
        };
        let stdout = server.child.stdout.take().unwrap();
        let (sender, receiver) = mpsc::channel();
        std::thread::spawn(move || {
            let mut line = String::new();
            let result = BufReader::new(stdout).read_line(&mut line);
            let _ = sender.send((result, line));
        });
        let (result, line) = receiver.recv_timeout(Duration::from_secs(10)).unwrap();
        result.unwrap();
        let value: Value = serde_json::from_str(&line).unwrap();
        assert_eq!(value["ok"], true, "{value}");
        assert_eq!(value["data"]["read_only"], true);
        value["data"]["url"]
            .as_str()
            .unwrap()
            .strip_prefix("http://")
            .unwrap()
            .clone_into(&mut server.address);
        assert!(server.address.starts_with("127.0.0.1:"));
        server
    }
    fn request(&self, method: &str, path: &str, host: &str, headers: &str) -> String {
        let mut stream = TcpStream::connect(&self.address).unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        write!(
            stream,
            "{method} {path} HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n{headers}\r\n"
        )
        .unwrap();
        let mut response = String::new();
        stream.read_to_string(&mut response).unwrap();
        response
    }
    fn get(&self, path: &str) -> String {
        self.request("GET", path, &self.address, "")
    }
    fn graph(&self) -> Value {
        let response = self.get("/api/graph");
        assert!(response.starts_with("HTTP/1.1 200"), "{response}");
        serde_json::from_str(response.split_once("\r\n\r\n").unwrap().1).unwrap()
    }
}
impl Drop for Dashboard {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

const ID: &str = "01ARZ3NDEKTSV4RRFFQ69G5FAV";
fn record(title: &str) -> String {
    format!(
        "+++\nschema_version = 1\nid = \"{ID}\"\ntitle = \"{title}\"\ndate = \"2026-09-19\"\nstatus = \"accepted\"\ntags = [\"storage\"]\n+++\n## Decision\n\n<script>alert('evidence')</script>\n"
    )
}

#[test]
fn dashboard_rebuilds_current_markdown_and_reports_invalid_edits() {
    let directory = TempDir::new().unwrap();
    let original = record("Use Markdown");
    fs::write(directory.path().join("decision.md"), &original).unwrap();
    let server = Dashboard::start(directory.path());
    let graph = server.graph();
    assert_eq!(graph["schema_version"], 1);
    assert_eq!(graph["data"]["valid"], true);
    assert_eq!(
        graph["data"]["graph"]["decisions"][ID]["title"],
        "Use Markdown"
    );
    assert!(
        graph["data"]["graph"]["decisions"][ID]["body"]
            .as_str()
            .unwrap()
            .contains("<script>")
    );
    assert_eq!(
        fs::read_to_string(directory.path().join("decision.md")).unwrap(),
        original
    );
    fs::rename(
        directory.path().join("decision.md"),
        directory.path().join("renamed.md"),
    )
    .unwrap();
    fs::write(
        directory.path().join("renamed.md"),
        record("A revised title"),
    )
    .unwrap();
    let graph = server.graph();
    assert_eq!(
        graph["data"]["graph"]["decisions"][ID]["title"],
        "A revised title"
    );
    assert_eq!(
        graph["data"]["graph"]["decisions"][ID]["file"],
        "renamed.md"
    );
    fs::write(directory.path().join("broken.md"), "missing metadata").unwrap();
    let graph = server.graph();
    assert_eq!(graph["ok"], true);
    assert_eq!(graph["data"]["valid"], false);
    assert_eq!(graph["data"]["graph"]["findings"][0]["file"], "broken.md");
    assert!(
        server
            .get(&format!("/api/suggest?id={ID}"))
            .starts_with("HTTP/1.1 422")
    );
    fs::remove_file(directory.path().join("broken.md")).unwrap();
    assert_eq!(server.graph()["data"]["valid"], true);
    let response = server.get(&format!("/api/suggest?id={ID}"));
    assert!(response.starts_with("HTTP/1.1 200"));
    let value: Value = serde_json::from_str(response.split_once("\r\n\r\n").unwrap().1).unwrap();
    assert_eq!(value["data"]["advisory"], true);
    assert_eq!(value["data"]["decision"]["id"], ID);
}

#[test]
fn dashboard_bundles_assets_and_restricts_network_and_filesystem_access() {
    let directory = TempDir::new().unwrap();
    let server = Dashboard::start(directory.path());
    assert!(
        server.graph()["data"]["graph"]["decisions"]
            .as_object()
            .unwrap()
            .is_empty()
    );
    for path in ["/", "/app.js", "/style.css", "/logo.svg"] {
        let response = server.get(path);
        assert!(response.starts_with("HTTP/1.1 200"), "{path}: {response}");
        assert!(response.contains("Content-Security-Policy:"));
        assert!(response.contains("X-Content-Type-Options: nosniff"));
    }
    for path in [
        "/../Cargo.toml",
        "/%2e%2e/Cargo.toml",
        "/decision.md",
        "/unknown",
    ] {
        assert!(server.get(path).starts_with("HTTP/1.1 404"));
    }
    assert!(
        server
            .request(
                "POST",
                "/api/graph",
                &server.address,
                "Content-Length: 0\r\n"
            )
            .starts_with("HTTP/1.1 405")
    );
    assert!(
        server
            .request("GET", "/api/graph", "evil.example", "")
            .starts_with("HTTP/1.1 403")
    );
    assert!(
        server
            .request(
                "GET",
                "/api/graph",
                &server.address,
                "Origin: https://evil.example\r\n"
            )
            .starts_with("HTTP/1.1 403")
    );
    assert!(
        server
            .request(
                "GET",
                "/api/graph",
                &server.address,
                "Sec-Fetch-Site: cross-site\r\n"
            )
            .starts_with("HTTP/1.1 403")
    );
    assert!(
        server
            .get("/api/suggest?id=%2fetc")
            .starts_with("HTTP/1.1 400")
    );
    assert!(
        server
            .get(&format!("/api/suggest?id={ID}"))
            .starts_with("HTTP/1.1 404")
    );
    assert_eq!(fs::read_dir(directory.path()).unwrap().count(), 0);
}
