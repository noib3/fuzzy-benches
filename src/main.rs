use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

fn traces(c: &mut Criterion) {
    norm_fzf_v1::long(c);

    norm_fzf_v2::long(c);

    nucleo::long(c);

    telescope_fzf_native::long(c);
}

mod norm_fzf_v1 {
    use norm::fzf::*;
    use norm::*;

    use super::*;

    #[inline(always)]
    pub(super) fn long(c: &mut Criterion) {
        bench(c, "emacs", candidates::candidates(), "long");
    }

    fn bench(c: &mut Criterion, query: &str, candidates: &[&str], bench_name: &str) {
        let mut group = c.benchmark_group("norm");

        group.throughput(Throughput::Elements(candidates.len() as u64));

        group.bench_function(BenchmarkId::new("fzf-v1", bench_name), |b| {
            let mut fzf = FzfV1::new()
                .with_case_sensitivity(CaseSensitivity::Smart)
                .with_matched_ranges(false);

            let mut parser = FzfParser::new();

            let query = parser.parse(query);

            b.iter(|| {
                for candidate in candidates {
                    let _ = fzf.distance(query, candidate);
                }
            })
        });
    }
}

mod norm_fzf_v2 {
    use norm::fzf::*;
    use norm::*;

    use super::*;

    #[inline(always)]
    pub(super) fn long(c: &mut Criterion) {
        bench(c, "emacs", candidates::candidates(), "long");
    }

    fn bench(c: &mut Criterion, query: &str, candidates: &[&str], bench_name: &str) {
        let mut group = c.benchmark_group("norm");

        group.throughput(Throughput::Elements(candidates.len() as u64));

        group.bench_function(BenchmarkId::new("fzf-v2", bench_name), |b| {
            let mut fzf = FzfV2::new()
                .with_case_sensitivity(CaseSensitivity::Smart)
                .with_matched_ranges(false);

            let mut parser = FzfParser::new();

            let query = parser.parse(query);

            b.iter(|| {
                for candidate in candidates {
                    let _ = fzf.distance(query, candidate);
                }
            })
        });
    }
}

mod nucleo {
    use nucleo_matcher::pattern::*;
    use nucleo_matcher::*;

    use super::*;

    #[inline(always)]
    pub(super) fn long(c: &mut Criterion) {
        bench(c, "emacs", candidates::candidates(), "long");
    }

    fn bench(c: &mut Criterion, query: &str, candidates: &[&str], bench_name: &str) {
        let mut group = c.benchmark_group("nucleo");

        group.throughput(Throughput::Elements(candidates.len() as u64));

        group.bench_function(bench_name, |b| {
            let mut matcher = {
                let mut config = Config::DEFAULT;
                config.normalize = false;
                Matcher::new(config)
            };

            let query = Pattern::parse(query, CaseMatching::Smart);

            let mut buf = Vec::new();

            b.iter(|| {
                for candidate in candidates {
                    let candidate = Utf32Str::new(candidate, &mut buf);
                    let _ = query.score(candidate, &mut matcher);
                }
            })
        });
    }
}

mod telescope_fzf_native {
    use ::telescope_fzf_native::*;

    use super::*;

    use std::ffi::CStr;

    #[inline(always)]
    pub(super) fn long(c: &mut Criterion) {
        bench(c, "emacs", candidates::null_terminated(), "long");
    }

    fn bench(c: &mut Criterion, query: &str, candidates: &[&CStr], bench_name: &str) {
        let mut group = c.benchmark_group("telescope-fzf-native");

        group.throughput(Throughput::Elements(candidates.len() as u64));

        group.bench_function(bench_name, |b| {
            let mut fzf = Fzf::new();

            let query = FzfQuery::parse(query.to_owned());

            b.iter(|| {
                for candidate in candidates {
                    let _ = fzf.score(&query, candidate);
                }
            })
        });
    }
}

criterion_group!(benches, traces);

criterion_main!(benches);
