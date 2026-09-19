//! Replay a realistic, stateful project history through the shipped CLI and HTTP API.
#![allow(
    clippy::arithmetic_side_effects,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::option_if_let_else,
    clippy::too_many_lines,
    clippy::unwrap_used,
    reason = "Bounded fixture arithmetic and direct failures keep replay traces readable"
)]

use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet},
    ffi::OsString,
    fs,
    io::{BufRead, BufReader, Read, Write},
    net::TcpStream,
    path::{Path, PathBuf},
    process::{Child, Command, Output, Stdio},
    sync::mpsc,
    time::Duration,
};
use tempfile::TempDir;
use vrdx::records::{Metadata, Status};

const SCENARIO: &str = "release-collection-v1-seed-20260919";
const BACKGROUND_RECORDS: usize = 216;
const EXPECTED_RECORDS: usize = BACKGROUND_RECORDS + 5;
const MISSING_ID: &str = "7ZZZZZZZZZZZZZZZZZZZZZZZZZ";

fn binary() -> OsString {
    std::env::var_os("VRDX_TEST_BINARY").unwrap_or_else(|| env!("CARGO_BIN_EXE_vrdx").into())
}

fn raw(root: &Path, args: &[&str]) -> Output {
    Command::new(binary())
        .args(args)
        .current_dir(root)
        .output()
        .unwrap()
}

fn invoke(root: &Path, args: &[&str]) -> (i32, Value) {
    let mut values = vec!["--json", "--dir", "."];
    values.extend(args);
    let output = raw(root, &values);
    assert!(
        output.stderr.is_empty(),
        "{SCENARIO}: stderr for {args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let value: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["schema_version"], 1, "{SCENARIO}: {args:?}");
    (output.status.code().unwrap(), value)
}

fn success(root: &Path, args: &[&str]) -> Value {
    let (code, response) = invoke(root, args);
    assert_eq!(code, 0, "{SCENARIO}: {args:?}: {response}");
    assert_eq!(response["ok"], true, "{SCENARIO}: {args:?}");
    response["data"].clone()
}

fn write(root: &Path, name: &str, metadata: &Metadata, body: &str) {
    fs::write(
        root.join(name),
        format!("+++\n{}+++\n{body}", toml::to_string(metadata).unwrap()),
    )
    .unwrap();
}

fn edit(root: &Path, file: &str, change: impl FnOnce(&mut Metadata)) {
    let path = root.join(file);
    let source = fs::read_to_string(&path).unwrap();
    let source = source.strip_prefix("+++\n").unwrap();
    let (header, body) = source.split_once("+++\n").unwrap();
    let mut metadata: Metadata = toml::from_str(header).unwrap();
    change(&mut metadata);
    write(root, file, &metadata, body);
}

fn snapshot(root: &Path) -> BTreeMap<String, Vec<u8>> {
    fs::read_dir(root)
        .unwrap()
        .map(|entry| {
            let entry = entry.unwrap();
            assert!(
                entry.file_type().unwrap().is_file(),
                "{SCENARIO}: unexpected non-file {}",
                entry.path().display()
            );
            let path = entry.path();
            (path.to_string_lossy().into_owned(), fs::read(path).unwrap())
        })
        .collect()
}

#[derive(Clone)]
struct ModelRecord {
    id: String,
    file: String,
    status: Status,
    tags: Vec<String>,
}

fn populate_background(root: &Path, count: usize) -> Vec<ModelRecord> {
    let topics = [
        "storage",
        "security",
        "observability",
        "delivery",
        "networking",
        "frontend",
        "analytics",
        "testing",
        "identity",
        "billing",
        "documentation",
        "runtime",
    ];
    let ids: Vec<_> = (0..count)
        .map(|index| {
            ulid::Ulid::from(u128::try_from(index).unwrap().saturating_add(10_000)).to_string()
        })
        .collect();
    let mut records = Vec::new();
    for (index, id) in ids.iter().enumerate() {
        let topic = topics[index % topics.len()];
        let status = match index % 4 {
            0 => Status::Accepted,
            1 => Status::Proposed,
            2 => Status::Rejected,
            _ => Status::Deprecated,
        };
        let mut metadata = Metadata {
            schema_version: 1,
            id: id.clone(),
            title: format!("{topic} operating choice {index:03}"),
            date: format!("2026-08-{:02}", index % 28 + 1),
            status,
            tags: vec![topic.into(), format!("workstream-{}", index % 6)],
            supersedes: vec![],
            superseded_by: vec![],
            depends_on: vec![],
            related_to: vec![],
        };
        if index > 0 && index % 17 == 0 {
            metadata.depends_on.push(ids[index - 1].clone());
        }
        if index > 0 && index % 19 == 0 {
            metadata.related_to.push(ids[index - 1].clone());
        }
        let file = format!("background-{index:03}.md");
        write(
            root,
            &file,
            &metadata,
            &format!(
                "## Decision\n\nUse bounded {topic} batch {index}.\n\n## Context\n\nThe {topic} subsystem needs traceable rollouts.\n\n## Consequences\n\nOperators trade throughput for predictable recovery marker {index}.\n"
            ),
        );
        records.push(ModelRecord {
            id: id.clone(),
            file,
            status,
            tags: metadata.tags,
        });
    }
    records
}

struct Created {
    id: String,
    file: String,
}

fn create(root: &Path, title: &str, tags: &[&str], body: &str) -> Created {
    let body_file = root.join("simulation-body.txt");
    fs::write(&body_file, body).unwrap();
    let mut args = vec!["new", title, "--date", "2026-09-19", "--status", "accepted"];
    for tag in tags {
        args.extend(["--tag", tag]);
    }
    args.extend(["--body-file", "simulation-body.txt"]);
    let result = success(root, &args);
    fs::remove_file(body_file).unwrap();
    Created {
        id: result["decision"]["id"].as_str().unwrap().into(),
        file: result["decision"]["file"].as_str().unwrap().into(),
    }
}

fn workspace() -> (PathBuf, Option<TempDir>) {
    if let Some(path) = std::env::var_os("VRDX_SIMULATION_KEEP_DIR") {
        let path = PathBuf::from(path);
        if path.exists() {
            assert_eq!(
                fs::read_dir(&path).unwrap().count(),
                0,
                "{SCENARIO}: VRDX_SIMULATION_KEEP_DIR must be empty"
            );
        } else {
            fs::create_dir_all(&path).unwrap();
        }
        (path, None)
    } else {
        let temporary = TempDir::new().unwrap();
        (temporary.path().to_owned(), Some(temporary))
    }
}

struct Dashboard {
    child: Child,
    address: String,
}

impl Dashboard {
    fn start(directory: &Path) -> Self {
        let child = Command::new(binary())
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
        assert_eq!(value["ok"], true, "{SCENARIO}: {value}");
        value["data"]["url"]
            .as_str()
            .unwrap()
            .strip_prefix("http://")
            .unwrap()
            .clone_into(&mut server.address);
        server
    }

    fn get(&self, path: &str) -> (u16, Value) {
        let mut stream = TcpStream::connect(&self.address).unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        write!(
            stream,
            "GET {path} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
            self.address
        )
        .unwrap();
        let mut response = Vec::new();
        stream.read_to_end(&mut response).unwrap();
        let header_end = response
            .windows(4)
            .position(|window| window == b"\r\n\r\n")
            .unwrap();
        let headers = std::str::from_utf8(&response[..header_end]).unwrap();
        let body = &response[header_end.saturating_add(4)..];
        let status = headers
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<u16>()
            .unwrap();
        let body = if headers
            .lines()
            .any(|line| line.eq_ignore_ascii_case("Transfer-Encoding: chunked"))
        {
            decode_chunked(body)
        } else {
            body.to_vec()
        };
        (status, serde_json::from_slice(&body).unwrap())
    }
}

// Large graph responses use chunked framing. Decode bytes before JSON so a
// chunk boundary cannot split a UTF-8 code point in a future fixture.
fn decode_chunked(body: &[u8]) -> Vec<u8> {
    let mut decoded = Vec::new();
    let mut position = 0;
    loop {
        let line_end = body[position..]
            .windows(2)
            .position(|window| window == b"\r\n")
            .unwrap()
            .saturating_add(position);
        let size =
            usize::from_str_radix(std::str::from_utf8(&body[position..line_end]).unwrap(), 16)
                .unwrap();
        position = line_end.saturating_add(2);
        if size == 0 {
            return decoded;
        }
        let chunk_end = position.saturating_add(size);
        decoded.extend_from_slice(&body[position..chunk_end]);
        position = chunk_end.saturating_add(2);
    }
}

impl Drop for Dashboard {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn finding_codes(response: &Value) -> Vec<&str> {
    response["data"]["findings"]
        .as_array()
        .unwrap()
        .iter()
        .map(|finding| finding["code"].as_str().unwrap())
        .collect()
}

struct ReplayRng(u64);

impl ReplayRng {
    const fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        self.0
    }

    fn index(&mut self, length: usize) -> usize {
        usize::try_from(self.next() % u64::try_from(length).unwrap()).unwrap()
    }
}

#[test]
fn mixed_project_history_remains_useful_through_mistakes_repairs_and_renames() {
    let (root, _temporary) = workspace();
    let _background = populate_background(&root, BACKGROUND_RECORDS);
    assert_eq!(
        success(&root, &["validate"])["records"],
        BACKGROUND_RECORDS,
        "{SCENARIO}: background workload"
    );

    let original = create(
        &root,
        "Cache external API reads",
        &["performance", "edge-cache"],
        "## Decision\n\nCache external API reads for five minutes.\n\n## Context\n\nOrigin requests raise p95 latency.\n\n## Consequences\n\nResponses may be stale.\n",
    );
    let middle = create(
        &root,
        "Bound cache entries with fixed TTL",
        &["performance", "edge-cache"],
        "## Decision\n\nUse a fixed TTL and generation token.\n\n## Consequences\n\nFreshness improves while misses cost latency.\n",
    );
    let current = create(
        &root,
        "Adopt adaptive cache freshness budget",
        &["performance", "edge-cache", "freshness"],
        "## Decision\n\nAdapt cache lifetime to the freshness budget.\n\n## Consequences\n\nLatency stays bounded while stale responses remain observable.\n",
    );
    let dependency = create(
        &root,
        "Issue monotonic freshness generations",
        &["data-integrity", "generation-token"],
        "## Decision\n\nIssue a monotonic generation token for every origin change.\n",
    );
    let candidate = create(
        &root,
        "Monitor adaptive cache freshness budget",
        &["performance", "edge-cache", "freshness", "observability"],
        "## Decision\n\nMonitor adaptive cache freshness latency and stale response budgets.\n",
    );
    edit(&root, &candidate.file, |metadata| {
        metadata.status = Status::Proposed;
    });
    assert_eq!(
        success(&root, &["validate"])["records"],
        EXPECTED_RECORDS,
        "{SCENARIO}: CLI creation count"
    );

    let dashboard = Dashboard::start(&root);
    let (status, graph) = dashboard.get("/api/graph");
    assert_eq!(status, 200, "{SCENARIO}: {graph}");
    assert_eq!(graph["data"]["valid"], true, "{SCENARIO}: {graph}");
    assert_eq!(
        graph["data"]["graph"]["decisions"]
            .as_object()
            .unwrap()
            .len(),
        EXPECTED_RECORDS,
        "{SCENARIO}: HTTP graph count"
    );

    // A normal two-file lifecycle edit is briefly inconsistent. Authoritative
    // retrieval must stop until the user finishes the edit.
    edit(&root, &middle.file, |metadata| {
        metadata.supersedes.push(original.id.clone());
    });
    let (code, invalid) = invoke(&root, &["validate"]);
    assert_eq!(code, 1, "{SCENARIO}: incomplete first replacement");
    assert!(
        finding_codes(&invalid).contains(&"status_mismatch"),
        "{SCENARIO}: {invalid}"
    );
    let (_, blocked) = invoke(&root, &["context", "external API cache"]);
    assert_eq!(
        blocked["error"]["code"], "invalid_collection",
        "{SCENARIO}: {blocked}"
    );
    let (status, invalid_graph) = dashboard.get("/api/graph");
    assert_eq!(status, 200, "{SCENARIO}: {invalid_graph}");
    assert_eq!(
        invalid_graph["data"]["valid"], false,
        "{SCENARIO}: dashboard must expose the incomplete edit"
    );
    let (status, _) = dashboard.get(&format!("/api/suggest?id={}", middle.id));
    assert_eq!(status, 422, "{SCENARIO}: invalid HTTP suggestion");

    edit(&root, &original.file, |metadata| {
        metadata.status = Status::Superseded;
    });
    assert_eq!(
        success(&root, &["chain", &original.id])["terminal"]["id"],
        middle.id,
        "{SCENARIO}: repaired first replacement"
    );

    // The inverse edit ordering produces the other realistic transient error.
    edit(&root, &middle.file, |metadata| {
        metadata.status = Status::Superseded;
    });
    let (_, invalid) = invoke(&root, &["validate"]);
    assert!(
        finding_codes(&invalid).contains(&"missing_replacement"),
        "{SCENARIO}: {invalid}"
    );
    edit(&root, &current.file, |metadata| {
        metadata.supersedes.push(middle.id.clone());
        metadata.depends_on.push(dependency.id.clone());
    });
    assert_eq!(
        success(&root, &["validate"])["records"],
        EXPECTED_RECORDS,
        "{SCENARIO}: repaired second replacement"
    );

    let renamed_original = "history-cache-external-api.md";
    let renamed_current = "current-cache-policy.md";
    fs::rename(root.join(&original.file), root.join(renamed_original)).unwrap();
    fs::rename(root.join(&current.file), root.join(renamed_current)).unwrap();
    assert_eq!(
        success(&root, &["show", &original.id])["decision"]["file"],
        renamed_original,
        "{SCENARIO}: historical rename"
    );
    assert_eq!(
        success(&root, &["show", &current.id])["decision"]["file"],
        renamed_current,
        "{SCENARIO}: current rename"
    );
    let chain = success(&root, &["chain", &original.id]);
    assert_eq!(
        chain["chain"]
            .as_array()
            .unwrap()
            .iter()
            .map(|decision| decision["id"].as_str().unwrap())
            .collect::<Vec<_>>(),
        [&original.id, &middle.id, &current.id],
        "{SCENARIO}: replacement history after rename"
    );
    assert_eq!(chain["terminal"]["applies"], true, "{SCENARIO}: {chain}");

    // A mistyped relationship is visible everywhere and remains repairable.
    edit(&root, &candidate.file, |metadata| {
        metadata.depends_on.push(MISSING_ID.into());
    });
    let (_, invalid) = invoke(&root, &["validate"]);
    assert!(
        finding_codes(&invalid).contains(&"missing_reference"),
        "{SCENARIO}: {invalid}"
    );
    assert_eq!(dashboard.get("/api/graph").1["data"]["valid"], false);
    edit(&root, &candidate.file, |metadata| {
        metadata.depends_on.clear();
    });
    success(&root, &["validate"]);

    let context = success(
        &root,
        &[
            "context",
            "external API cache",
            "--limit",
            "1",
            "--body-chars",
            "80",
        ],
    );
    assert_eq!(
        context["matches"][0]["id"], original.id,
        "{SCENARIO}: {context}"
    );
    assert_eq!(
        context["decisions"]
            .as_array()
            .unwrap()
            .iter()
            .map(|decision| decision["id"].as_str().unwrap())
            .collect::<Vec<_>>(),
        [&original.id, &middle.id, &current.id],
        "{SCENARIO}: context must retain complete replacement history"
    );
    assert!(
        context["boundary_decisions"]
            .as_array()
            .unwrap()
            .iter()
            .any(|decision| decision["id"] == dependency.id),
        "{SCENARIO}: dependency must remain explained at the context boundary"
    );
    assert_eq!(context["decisions"][0]["body_truncated"], true);

    // These are intentionally lexical expectations using ordinary project
    // vocabulary; the simulation does not claim semantic-search behavior.
    for (question, expected) in [
        ("adopt adaptive cache", &current.id),
        (
            "monotonic freshness generations origin change",
            &dependency.id,
        ),
    ] {
        assert_eq!(
            success(&root, &["context", question, "--limit", "1"])["matches"][0]["id"],
            expected.as_str(),
            "{SCENARIO}: unfiltered retrieval for {question}"
        );
    }
    assert!(
        success(&root, &["context", "quantum zebras", "--limit", "5"])["decisions"]
            .as_array()
            .unwrap()
            .is_empty(),
        "{SCENARIO}: unrelated vocabulary should not retrieve background noise"
    );

    let before_suggest = snapshot(&root);
    let suggestions = success(&root, &["suggest", &current.id, "--limit", "1"]);
    assert_eq!(
        suggestions["suggestions"][0]["decision"]["id"], candidate.id,
        "{SCENARIO}: {suggestions}"
    );
    assert_eq!(
        suggestions,
        success(&root, &["suggest", &current.id, "--limit", "1"]),
        "{SCENARIO}: suggestion ordering"
    );
    assert_eq!(
        snapshot(&root),
        before_suggest,
        "{SCENARIO}: suggest wrote files"
    );

    let accepted_cache = success(
        &root,
        &["list", "--status", "accepted", "--tag", "edge-cache"],
    );
    assert_eq!(
        accepted_cache["decisions"].as_array().unwrap().len(),
        1,
        "{SCENARIO}: only the current cache policy applies"
    );
    assert!(
        accepted_cache["decisions"]
            .as_array()
            .unwrap()
            .iter()
            .any(|decision| decision["id"] == current.id),
        "{SCENARIO}: current policy missing from filtered list"
    );

    let rebuilt = success(&root, &["rebuild"]);
    assert_eq!(rebuilt, success(&root, &["rebuild"]), "{SCENARIO}: rebuild");
    let (status, final_graph) = dashboard.get("/api/graph");
    assert_eq!(status, 200, "{SCENARIO}: {final_graph}");
    assert_eq!(
        final_graph["data"]["valid"], true,
        "{SCENARIO}: {final_graph}"
    );
    assert_eq!(
        final_graph["data"]["graph"]["decisions"][&current.id]["file"], renamed_current,
        "{SCENARIO}: live dashboard rename refresh"
    );
    let (status, http_suggestions) = dashboard.get(&format!("/api/suggest?id={}", current.id));
    assert_eq!(status, 200, "{SCENARIO}: {http_suggestions}");
    assert_eq!(
        http_suggestions["data"]["suggestions"][0]["decision"]["id"], candidate.id,
        "{SCENARIO}: HTTP and CLI retrieval must agree"
    );

    if std::env::var_os("VRDX_SIMULATION_KEEP_DIR").is_some() {
        eprintln!("{SCENARIO}: retained simulation at {}", root.display());
    }
}

#[test]
fn seeded_edit_sequences_preserve_the_reference_collection() {
    const SEEDS: [u64; 3] = [0x5EED_0001, 0x5EED_2026, 0x5EED_CAFE];
    const RECORDS: usize = 64;
    const ACTIONS: usize = 48;

    for seed in SEEDS {
        let root = TempDir::new().unwrap();
        let mut records = populate_background(root.path(), RECORDS);
        let mut rng = ReplayRng(seed);
        let mut injected_faults = 0_usize;

        for step in 0..ACTIONS {
            let index = rng.index(records.len());
            let action = if step % 13 == 0 { 4 } else { rng.next() % 5 };
            let replay = format!("{SCENARIO}/seed-{seed:016x}/step-{step:03}/action-{action}");
            match action {
                0 => {
                    let next_file = format!("replay-{seed:016x}-{step:03}-{index:03}.md");
                    fs::rename(
                        root.path().join(&records[index].file),
                        root.path().join(&next_file),
                    )
                    .unwrap();
                    records[index].file = next_file;
                }
                1 => {
                    let status = match rng.next() % 4 {
                        0 => Status::Accepted,
                        1 => Status::Proposed,
                        2 => Status::Rejected,
                        _ => Status::Deprecated,
                    };
                    edit(root.path(), &records[index].file, |metadata| {
                        metadata.status = status;
                    });
                    records[index].status = status;
                }
                2 => {
                    let has_tag = records[index]
                        .tags
                        .iter()
                        .any(|tag| tag == "seeded-activity");
                    edit(root.path(), &records[index].file, |metadata| {
                        if has_tag {
                            metadata.tags.retain(|tag| tag != "seeded-activity");
                        } else {
                            metadata.tags.push("seeded-activity".into());
                        }
                    });
                    if has_tag {
                        records[index].tags.retain(|tag| tag != "seeded-activity");
                    } else {
                        records[index].tags.push("seeded-activity".into());
                    }
                }
                3 => {
                    let target_index = index.saturating_add(1) % records.len();
                    let target = records[target_index].id.clone();
                    edit(root.path(), &records[index].file, |metadata| {
                        if metadata.related_to.contains(&target) {
                            metadata.related_to.retain(|id| id != &target);
                        } else {
                            metadata.related_to.push(target);
                        }
                    });
                }
                _ => {
                    injected_faults = injected_faults.saturating_add(1);
                    edit(root.path(), &records[index].file, |metadata| {
                        metadata.depends_on.push(MISSING_ID.into());
                    });
                    let (code, invalid) = invoke(root.path(), &["validate"]);
                    assert_eq!(code, 1, "{replay}: {invalid}");
                    assert!(
                        finding_codes(&invalid).contains(&"missing_reference"),
                        "{replay}: {invalid}"
                    );
                    edit(root.path(), &records[index].file, |metadata| {
                        metadata.depends_on.retain(|id| id != MISSING_ID);
                    });
                }
            }

            assert_eq!(
                success(root.path(), &["validate"])["records"],
                RECORDS,
                "{replay}: post-action validation"
            );
            if step % 8 == 0 {
                let expected_accepted: BTreeSet<_> = records
                    .iter()
                    .filter(|record| record.status == Status::Accepted)
                    .map(|record| record.id.clone())
                    .collect();
                let actual_accepted: BTreeSet<_> =
                    success(root.path(), &["list", "--status", "accepted"])["decisions"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|record| record["id"].as_str().unwrap().to_owned())
                        .collect();
                assert_eq!(actual_accepted, expected_accepted, "{replay}: status model");

                let expected_tagged: BTreeSet<_> = records
                    .iter()
                    .filter(|record| record.tags.iter().any(|tag| tag == "seeded-activity"))
                    .map(|record| record.id.clone())
                    .collect();
                let actual_tagged: BTreeSet<_> =
                    success(root.path(), &["list", "--tag", "SEEDED-ACTIVITY"])["decisions"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|record| record["id"].as_str().unwrap().to_owned())
                        .collect();
                assert_eq!(actual_tagged, expected_tagged, "{replay}: tag model");
            }
        }

        assert!(
            injected_faults >= 4,
            "{SCENARIO}/seed-{seed:016x}: fault budget"
        );
        let rebuilt = success(root.path(), &["rebuild"]);
        for record in records {
            let actual = &rebuilt["graph"]["decisions"][&record.id];
            assert_eq!(
                actual["file"], record.file,
                "{SCENARIO}/seed-{seed:016x}: rename model for {}",
                record.id
            );
            assert_eq!(
                actual["status"],
                record.status.label(),
                "{SCENARIO}/seed-{seed:016x}: status model for {}",
                record.id
            );
            assert_eq!(
                actual["tags"],
                serde_json::json!(record.tags),
                "{SCENARIO}/seed-{seed:016x}: tag model for {}",
                record.id
            );
        }
    }
}
