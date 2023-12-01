use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::{clangd::ClangdMatcher, skim::SkimMatcherV2};

use crate::{Metric, Parser};

#[derive(Default)]
pub struct FuzzyMatcherParser;

impl Parser for FuzzyMatcherParser {
    type Query<'a> = &'a str;

    #[inline(always)]
    fn parse<'a>(&'a mut self, query: &'static str) -> Self::Query<'a> {
        query
    }
}

pub struct SkimV2 {
    matcher: SkimMatcherV2,
}

impl Metric for SkimV2 {
    const NAME: &'static str = "fuzzy-matcher-skim-v2";

    type Parser = FuzzyMatcherParser;

    #[inline(always)]
    fn new() -> Self {
        Self {
            matcher: SkimMatcherV2::default().smart_case(),
        }
    }

    #[inline(always)]
    fn distance(&mut self, query: &str, candidates: &[&str]) {
        for candidate in candidates {
            let _ = self.matcher.fuzzy_match(candidate, query);
        }
    }

    #[inline(always)]
    fn distance_and_matches(&mut self, query: &str, candidates: &[&str]) {
        for candidate in candidates {
            let _ = self.matcher.fuzzy_indices(candidate, query);
        }
    }
}

pub struct Clangd {
    matcher: ClangdMatcher,
}

impl Metric for Clangd {
    const NAME: &'static str = "fuzzy-matcher-clangd";

    type Parser = FuzzyMatcherParser;

    #[inline(always)]
    fn new() -> Self {
        Self {
            matcher: ClangdMatcher::default().smart_case(),
        }
    }

    #[inline(always)]
    fn distance(&mut self, query: &str, candidates: &[&str]) {
        for candidate in candidates {
            let _ = self.matcher.fuzzy_match(candidate, query);
        }
    }

    #[inline(always)]
    fn distance_and_matches(&mut self, query: &str, candidates: &[&str]) {
        for candidate in candidates {
            let _ = self.matcher.fuzzy_indices(candidate, query);
        }
    }
}
