#![allow(long_running_const_eval)]

use criterion::measurement::WallTime;
use criterion::{
    criterion_group, criterion_main, BenchmarkGroup, BenchmarkId, Criterion, Throughput,
};
use norm::fzf::*;
use norm::*;

fn traces(c: &mut Criterion) {
    let mut group = c.benchmark_group("telescope-fzf-native");

    group.bench_function("long", |b| {
        let mut fzf = telescope_fzf_native::Fzf::new();
        let query = telescope_fzf_native::FzfQuery::parse("emacs".to_owned());
        b.iter(|| {
            for candidate in candidates::null_terminated() {
                let _ = fzf.score(&query, candidate);
            }
        })
    });
}

fn fzf_v2() {
    let mut fzf = FzfV2::new()
        .with_case_sensitivity(CaseSensitivity::Smart)
        .with_matched_ranges(false);

    let mut parser = FzfParser::new();

    let query = parser.parse("emacs");
}

criterion_group!(benches, traces);

criterion_main!(benches);
