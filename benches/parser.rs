use std::fmt::Write as _;
use std::hint::black_box;
use std::path::PathBuf;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use vrdx::document::Document;

fn document_text(count: usize) -> String {
    let mut text = String::from("# Engineering decisions\n\n<!-- vrdx start -->\n");
    for id in 1..=count {
        // Writing into a String cannot fail; avoid a panic even in the benchmark fixture.
        let _ = writeln!(
            text,
            "### {id} Decision {id}\n* **Status**: ✅ Accepted\n* **Decision**: Keep records in Markdown.\n* **Context**: Records need code review.\n* **Consequences**: Records stay with the repository.\n"
        );
    }
    text.push_str("<!-- vrdx end -->\n\nOther repository documentation.\n");
    text
}

fn parse_documents(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("parse_decisions");
    for count in [1, 100, 1_000] {
        let text = document_text(count);
        assert!(
            Document::parse(PathBuf::from("decisions.md"), text.clone()).is_ok(),
            "the benchmark must measure successful document parsing"
        );
        if let Ok(bytes) = u64::try_from(text.len()) {
            group.throughput(Throughput::Bytes(bytes));
        }
        group.bench_with_input(
            BenchmarkId::from_parameter(count),
            &text,
            |bencher, text| {
                bencher.iter(|| {
                    black_box(Document::parse(
                        PathBuf::from("decisions.md"),
                        black_box(text.clone()),
                    ))
                });
            },
        );
    }
    group.finish();
}

criterion_group!(benches, parse_documents);
criterion_main!(benches);
