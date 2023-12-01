use core::ops::Range;

use norm::fzf::{self, FzfParser, FzfQuery};

use crate::{Metric, Parser};

impl Parser for FzfParser {
    type Query<'a> = FzfQuery<'a>;

    #[inline(always)]
    fn parse<'a>(&'a mut self, query: &'static str) -> Self::Query<'a> {
        self.parse(query)
    }
}

pub struct FzfV1 {
    v1: fzf::FzfV1,
    ranges_buf: Vec<Range<usize>>,
}

impl Metric for FzfV1 {
    const NAME: &'static str = "norm-fzf-v1";

    type Parser = FzfParser;

    #[inline(always)]
    fn new() -> Self {
        Self {
            v1: fzf::FzfV1::new(),
            ranges_buf: Vec::new(),
        }
    }

    #[inline(always)]
    fn distance(&mut self, query: FzfQuery, candidates: &[&str]) {
        use norm::Metric;

        for candidate in candidates {
            let _ = self.v1.distance(query, candidate);
        }
    }

    #[inline(always)]
    fn distance_and_matches(&mut self, query: FzfQuery, candidates: &[&str]) {
        use norm::Metric;

        for candidate in candidates {
            let _ = self
                .v1
                .distance_and_ranges(query, candidate, &mut self.ranges_buf);
        }
    }
}

pub struct FzfV2 {
    v2: fzf::FzfV2,
    ranges_buf: Vec<Range<usize>>,
}

impl Metric for FzfV2 {
    const NAME: &'static str = "norm-fzf-v2";

    type Parser = FzfParser;

    #[inline(always)]
    fn new() -> Self {
        Self {
            v2: fzf::FzfV2::new(),
            ranges_buf: Vec::new(),
        }
    }

    #[inline(always)]
    fn distance(&mut self, query: FzfQuery, candidates: &[&str]) {
        use norm::Metric;

        for candidate in candidates {
            let _ = self.v2.distance(query, candidate);
        }
    }

    #[inline(always)]
    fn distance_and_matches(&mut self, query: FzfQuery, candidates: &[&str]) {
        use norm::Metric;

        for candidate in candidates {
            let _ = self
                .v2
                .distance_and_ranges(query, candidate, &mut self.ranges_buf);
        }
    }
}
