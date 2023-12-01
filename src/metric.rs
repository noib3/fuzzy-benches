use core::ffi::CStr;
use std::ffi::CString;

use criterion::{BenchmarkId, Criterion, Throughput};

pub fn bench<M: Metric>(
    c: &mut Criterion,
    query: &'static str,
    candidates: &[&str],
    bench_name: &str,
) {
    let mut group = c.benchmark_group(bench_name);

    group.throughput(Throughput::Elements(candidates.len() as u64));

    let mut metric = M::new();

    let mut parser = M::Parser::default();

    if !M::CSTR_CANDIDATES {
        let query = parser.parse(query);

        group.bench_function(M::NAME, |b| {
            b.iter(|| {
                metric.distance(query, candidates);
            })
        });

        group.bench_function(BenchmarkId::new(M::NAME, "with_matches"), |b| {
            b.iter(|| {
                metric.distance_and_matches(query, candidates);
            })
        });
    } else {
        let ptr = CString::new(query.to_owned()).unwrap().into_raw();

        let query = unsafe { CStr::from_ptr(ptr) };

        let query = parser.parse_cstr(query);

        let candidates: Vec<_> = candidates
            .iter()
            .map(|candidate| CString::new(candidate.to_owned()).unwrap())
            .collect();

        let candidates: Vec<_> = candidates
            .iter()
            .map(|candidate| candidate.as_c_str())
            .collect();

        group.bench_function(M::NAME, |b| {
            b.iter(|| {
                metric.cstr_distance(query, &candidates);
            })
        });

        group.bench_function(BenchmarkId::new(M::NAME, "with_matches"), |b| {
            b.iter(|| {
                metric.cstr_distance_and_matches(query, &candidates);
            })
        });

        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}

pub trait Parser: Default {
    const CSTR_CANDIDATES: bool = false;

    type Query<'a>: Copy
    where
        Self: 'a;

    fn parse<'a>(&'a mut self, query: &'static str) -> Self::Query<'a>;

    fn parse_cstr<'a>(&'a mut self, _: &'static CStr) -> Self::Query<'a> {
        unimplemented!();
    }
}

pub trait Metric {
    const CSTR_CANDIDATES: bool = false;

    const NAME: &'static str;

    type Parser: Parser;

    fn new() -> Self;

    fn distance(&mut self, query: <Self::Parser as Parser>::Query<'_>, candidates: &[&str]);

    fn distance_and_matches(
        &mut self,
        query: <Self::Parser as Parser>::Query<'_>,
        candidates: &[&str],
    );

    fn cstr_distance(&mut self, _: <Self::Parser as Parser>::Query<'_>, _: &[&CStr]) {
        unimplemented!();
    }

    fn cstr_distance_and_matches(&mut self, _: <Self::Parser as Parser>::Query<'_>, _: &[&CStr]) {
        unimplemented!();
    }
}
