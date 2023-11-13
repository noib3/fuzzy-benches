use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::ffi::CStr;

const MEDIUM_TEXT: &str =
    "Far far away, behind the word mountains, far from the countries Vokalia \
     and Consonantia, there live the blind texts. Separated they live in \
     Bookmarksgrove right at the coast of the Semantics, a large.";

const LONG_TEXT: &str = "Far far away, behind the word mountains, far from the countries Vokalia \
     and Consonantia, there live the blind texts. Separated they live in \
     Bookmarksgrove right at the coast of the Semantics, a large language \
     ocean. A small river named Duden flows by their place and supplies it \
     with the necessary regelialia. It is a paradisematic country, in which \
     roasted parts of sentences fly into your mouth. Even the all-powerful \
     Pointing has no control about the blind texts it is an almost \
     unorthographic life";

const MEDIUM_TEXT_NULL_TERMINATED: &CStr = unsafe {
    CStr::from_bytes_with_nul_unchecked(
        "Far far away, behind the word mountains, far from the countries Vokalia \
     and Consonantia, there live the blind texts. Separated they live in \
     Bookmarksgrove right at the coast of the Semantics, a large.\0"
            .as_bytes(),
    )
};

const LONG_TEXT_NULL_TERMINATED: &CStr = unsafe {
    CStr::from_bytes_with_nul_unchecked(
        "Far far away, behind the word mountains, far from the countries Vokalia \
     and Consonantia, there live the blind texts. Separated they live in \
     Bookmarksgrove right at the coast of the Semantics, a large language \
     ocean. A small river named Duden flows by their place and supplies it \
     with the necessary regelialia. It is a paradisematic country, in which \
     roasted parts of sentences fly into your mouth. Even the all-powerful \
     Pointing has no control about the blind texts it is an almost \
     unorthographic life\0"
            .as_bytes(),
    )
};

mod norm_fzf_v1 {
    use norm::fzf::*;
    use norm::*;

    use super::*;

    #[inline(always)]
    pub(super) fn emacs(c: &mut Criterion) {
        bench(c, "emacs", candidates::candidates(), "emacs");
    }

    #[inline(always)]
    pub(super) fn medium_start(c: &mut Criterion) {
        bench(c, "away", &[MEDIUM_TEXT], "medium-start");
    }

    #[inline(always)]
    pub(super) fn medium_middle(c: &mut Criterion) {
        bench(c, "blind", &[MEDIUM_TEXT], "medium-middle");
    }

    #[inline(always)]
    pub(super) fn medium_end(c: &mut Criterion) {
        bench(c, "Semantics", &[MEDIUM_TEXT], "medium-end");
    }

    #[inline(always)]
    pub(super) fn long_start(c: &mut Criterion) {
        bench(c, "mountains", &[LONG_TEXT], "long-start");
    }

    #[inline(always)]
    pub(super) fn long_middle(c: &mut Criterion) {
        bench(c, "Duden", &[LONG_TEXT], "long-middle");
    }

    #[inline(always)]
    pub(super) fn long_end(c: &mut Criterion) {
        bench(c, "unorthographic", &[LONG_TEXT], "long-end");
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
    pub(super) fn emacs(c: &mut Criterion) {
        bench(c, "emacs", candidates::candidates(), "emacs");
    }

    #[inline(always)]
    pub(super) fn medium_start(c: &mut Criterion) {
        bench(c, "away", &[MEDIUM_TEXT], "medium-start");
    }

    #[inline(always)]
    pub(super) fn medium_middle(c: &mut Criterion) {
        bench(c, "blind", &[MEDIUM_TEXT], "medium-middle");
    }

    #[inline(always)]
    pub(super) fn medium_end(c: &mut Criterion) {
        bench(c, "Semantics", &[MEDIUM_TEXT], "medium-end");
    }

    #[inline(always)]
    pub(super) fn long_start(c: &mut Criterion) {
        bench(c, "mountains", &[LONG_TEXT], "long-start");
    }

    #[inline(always)]
    pub(super) fn long_middle(c: &mut Criterion) {
        bench(c, "Duden", &[LONG_TEXT], "long-middle");
    }

    #[inline(always)]
    pub(super) fn long_end(c: &mut Criterion) {
        bench(c, "unorthographic", &[LONG_TEXT], "long-end");
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
    pub(super) fn emacs(c: &mut Criterion) {
        bench(c, "emacs", candidates::candidates(), "emacs");
    }

    #[inline(always)]
    pub(super) fn medium_start(c: &mut Criterion) {
        bench(c, "away", &[MEDIUM_TEXT], "medium-start");
    }

    #[inline(always)]
    pub(super) fn medium_middle(c: &mut Criterion) {
        bench(c, "blind", &[MEDIUM_TEXT], "medium-middle");
    }

    #[inline(always)]
    pub(super) fn medium_end(c: &mut Criterion) {
        bench(c, "Semantics", &[MEDIUM_TEXT], "medium-end");
    }

    #[inline(always)]
    pub(super) fn long_start(c: &mut Criterion) {
        bench(c, "mountains", &[LONG_TEXT], "long-start");
    }

    #[inline(always)]
    pub(super) fn long_middle(c: &mut Criterion) {
        bench(c, "Duden", &[LONG_TEXT], "long-middle");
    }

    #[inline(always)]
    pub(super) fn long_end(c: &mut Criterion) {
        bench(c, "unorthographic", &[LONG_TEXT], "long-end");
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
    pub(super) fn emacs(c: &mut Criterion) {
        bench(c, "emacs", candidates::null_terminated(), "emacs");
    }

    #[inline(always)]
    pub(super) fn medium_start(c: &mut Criterion) {
        bench(c, "away", &[MEDIUM_TEXT_NULL_TERMINATED], "medium-start");
    }

    #[inline(always)]
    pub(super) fn medium_middle(c: &mut Criterion) {
        bench(c, "blind", &[MEDIUM_TEXT_NULL_TERMINATED], "medium-middle");
    }

    #[inline(always)]
    pub(super) fn medium_end(c: &mut Criterion) {
        bench(c, "Semantics", &[MEDIUM_TEXT_NULL_TERMINATED], "medium-end");
    }

    #[inline(always)]
    pub(super) fn long_start(c: &mut Criterion) {
        bench(c, "mountains", &[LONG_TEXT_NULL_TERMINATED], "long-start");
    }

    #[inline(always)]
    pub(super) fn long_middle(c: &mut Criterion) {
        bench(c, "Duden", &[LONG_TEXT_NULL_TERMINATED], "long-middle");
    }

    #[inline(always)]
    pub(super) fn long_end(c: &mut Criterion) {
        bench(
            c,
            "unorthographic",
            &[LONG_TEXT_NULL_TERMINATED],
            "long-end",
        );
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

fn traces(c: &mut Criterion) {
    norm_fzf_v1::emacs(c);
    norm_fzf_v1::medium_start(c);
    norm_fzf_v1::medium_middle(c);
    norm_fzf_v1::medium_end(c);
    norm_fzf_v1::long_start(c);
    norm_fzf_v1::long_middle(c);
    norm_fzf_v1::long_end(c);

    norm_fzf_v2::emacs(c);
    norm_fzf_v2::medium_start(c);
    norm_fzf_v2::medium_middle(c);
    norm_fzf_v2::medium_end(c);
    norm_fzf_v2::long_start(c);
    norm_fzf_v2::long_middle(c);
    norm_fzf_v2::long_end(c);

    nucleo::emacs(c);
    nucleo::medium_start(c);
    nucleo::medium_middle(c);
    nucleo::medium_end(c);
    nucleo::long_start(c);
    nucleo::long_middle(c);
    nucleo::long_end(c);

    telescope_fzf_native::emacs(c);
    telescope_fzf_native::medium_start(c);
    telescope_fzf_native::medium_middle(c);
    telescope_fzf_native::medium_end(c);
    telescope_fzf_native::long_start(c);
    telescope_fzf_native::long_middle(c);
    telescope_fzf_native::long_end(c);
}

criterion_group!(benches, traces);

criterion_main!(benches);
