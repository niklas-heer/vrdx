//! Opt-in executable benchmark: no machine-dependent pass/fail timing threshold.
#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    reason = "Bounded benchmark fixtures and sample indexing"
)]
use std::{
    fs,
    path::PathBuf,
    process::{Command, Stdio},
    time::Instant,
};
use tempfile::TempDir;

#[test]
#[ignore = "Run mise run bench for release-command measurements"]
fn release_command_latency() {
    let binary = std::env::var_os("VRDX_TEST_BINARY")
        .map_or_else(|| PathBuf::from(env!("CARGO_BIN_EXE_vrdx")), PathBuf::from);
    let root = TempDir::new().unwrap();
    let records = 1_000_u128;
    for index in 1..=records {
        let id = ulid::Ulid::from(index).to_string();
        let previous = ulid::Ulid::from(index.saturating_sub(1)).to_string();
        let links = if index > 1 && index <= 250 {
            format!("supersedes = [\"{previous}\"]\n")
        } else {
            String::new()
        };
        let status = if index < 250 {
            "superseded"
        } else {
            "accepted"
        };
        fs::write(root.path().join(format!("{index:04}.md")), format!("+++\nschema_version = 1\nid = \"{id}\"\ntitle = \"Cache topic {}\"\ndate = \"2026-09-19\"\nstatus = \"{status}\"\ntags = [\"topic{}\"]\n{links}+++\n\n## Decision\n\nCache reads for one minute.\n\n## Why\n\nRepeated reads are expensive.\n\n## Consequences\n\n- Fewer requests.\n- Stale reads.\n", index % 20, index % 20)).unwrap();
    }
    let first = ulid::Ulid::from(1_u128).to_string();
    let workflows = [
        vec!["guide"],
        vec!["list"],
        vec!["validate"],
        vec!["context", "cache", "--limit", "5"],
        vec!["suggest", first.as_str()],
        vec!["chain", first.as_str()],
    ];
    println!(
        "binary={} records={records} supersession_chain=250 samples=20 (warm filesystem; subprocess startup included)",
        binary.display()
    );
    for args in workflows {
        let mut samples = Vec::new();
        for iteration in 0..23 {
            let start = Instant::now();
            let output = Command::new(&binary)
                .arg("--dir")
                .arg(root.path())
                .arg("--json")
                .args(&args)
                .stdout(Stdio::null())
                .stderr(Stdio::piped())
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "{:?}: {}",
                args,
                String::from_utf8_lossy(&output.stderr)
            );
            if iteration >= 3 {
                samples.push(start.elapsed());
            }
        }
        samples.sort();
        println!(
            "{:<9} median={:.2}ms p95={:.2}ms",
            args[0],
            samples[10].as_secs_f64() * 1_000.0,
            samples[18].as_secs_f64() * 1_000.0
        );
    }
}
