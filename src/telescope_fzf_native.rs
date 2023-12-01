use core::ffi::CStr;

use telescope_fzf_native::{Fzf, FzfQuery};

use crate::{Metric, Parser};

pub struct FzfParser(FzfQuery<'static>);

impl Default for FzfParser {
    #[inline(always)]
    fn default() -> Self {
        Self(FzfQuery::parse(Default::default()))
    }
}

impl Parser for FzfParser {
    type Query<'a> = &'a FzfQuery<'a>;

    fn parse<'a>(&'a mut self, _: &'static str) -> Self::Query<'a> {
        unimplemented!();
    }

    #[inline(always)]
    fn parse_cstr<'a>(&'a mut self, query: &'static CStr) -> &'a FzfQuery<'a> {
        self.0 = FzfQuery::parse(query);
        &self.0
    }
}

pub struct TelescopeFzfNative {
    fzf: telescope_fzf_native::Fzf,
}

impl Metric for TelescopeFzfNative {
    const NAME: &'static str = "telescope-fzf-native";

    const CSTR_CANDIDATES: bool = true;

    type Parser = FzfParser;

    #[inline(always)]
    fn new() -> Self {
        Self { fzf: Fzf::new() }
    }

    #[inline(always)]
    fn distance(&mut self, _: &FzfQuery, _: &[&str]) {
        unimplemented!();
    }

    #[inline(always)]
    fn distance_and_matches(&mut self, _: &FzfQuery, _: &[&str]) {
        unimplemented!();
    }

    #[inline(always)]
    fn cstr_distance(&mut self, query: &FzfQuery, candidates: &[&CStr]) {
        for candidate in candidates {
            let _ = self.fzf.score(query, candidate);
        }
    }

    #[inline(always)]
    fn cstr_distance_and_matches(&mut self, query: &FzfQuery, candidates: &[&CStr]) {
        for candidate in candidates {
            let score = self.fzf.score(query, candidate);
            if score > 1 {
                let _ = self.fzf.positions(query, candidate);
            }
        }
    }
}
