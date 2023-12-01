mod fuzzy_matcher;
mod metric;
mod norm;
mod nucleo;
mod sublime_fuzzy;
mod telescope_fzf_native;

use criterion::{criterion_group, criterion_main, Criterion};

use metric::{Metric, Parser};

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

fn medium_start<M: Metric>(c: &mut Criterion) {
    let query = "away";
    let candidates = &[MEDIUM_TEXT];
    let bench_name = "medium-start";
    metric::bench::<M>(c, query, candidates, bench_name);
}

fn medium_middle<M: Metric>(c: &mut Criterion) {
    let query = "blind";
    let candidates = &[MEDIUM_TEXT];
    let bench_name = "medium-middle";
    metric::bench::<M>(c, query, candidates, bench_name);
}

fn medium_end<M: Metric>(c: &mut Criterion) {
    let query = "Semantics";
    let candidates = &[MEDIUM_TEXT];
    let bench_name = "medium-end";
    metric::bench::<M>(c, query, candidates, bench_name);
}

fn long_start<M: Metric>(c: &mut Criterion) {
    let query = "mountains";
    let candidates = &[LONG_TEXT];
    let bench_name = "long-start";
    metric::bench::<M>(c, query, candidates, bench_name);
}

fn long_middle<M: Metric>(c: &mut Criterion) {
    let query = "Duden";
    let candidates = &[LONG_TEXT];
    let bench_name = "long-middle";
    metric::bench::<M>(c, query, candidates, bench_name);
}

fn long_end<M: Metric>(c: &mut Criterion) {
    let query = "unorthographic";
    let candidates = &[LONG_TEXT];
    let bench_name = "long-end";
    metric::bench::<M>(c, query, candidates, bench_name);
}

fn emacs<M: Metric>(c: &mut Criterion) {
    let query = "emacs";
    let candidates = candidates::CANDIDATES;
    let bench_name = "emacs";
    metric::bench::<M>(c, query, candidates, bench_name);
}

fn bench<M: Metric>(c: &mut Criterion) {
    medium_start::<M>(c);
    medium_middle::<M>(c);
    medium_end::<M>(c);

    long_start::<M>(c);
    long_middle::<M>(c);
    long_end::<M>(c);

    emacs::<M>(c);
}

fn fuzzy(c: &mut Criterion) {
    bench::<norm::FzfV1>(c);
    bench::<norm::FzfV2>(c);
    bench::<nucleo::Nucleo>(c);
    bench::<fuzzy_matcher::Clangd>(c);
    bench::<fuzzy_matcher::SkimV2>(c);
    bench::<sublime_fuzzy::SublimeFuzzy>(c);
    bench::<telescope_fzf_native::TelescopeFzfNative>(c);
}

criterion_group!(benches, fuzzy);

criterion_main!(benches);
