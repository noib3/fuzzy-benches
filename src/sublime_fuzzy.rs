use sublime_fuzzy::best_match;

use crate::{Metric, Parser};

#[derive(Default)]
pub struct SublimeParser;

impl Parser for SublimeParser {
    type Query<'a> = &'a str;

    #[inline(always)]
    fn parse<'a>(&'a mut self, query: &'static str) -> Self::Query<'a> {
        query
    }
}

pub struct SublimeFuzzy;

impl Metric for SublimeFuzzy {
    const NAME: &'static str = "sublime-fuzzy";

    type Parser = SublimeParser;

    #[inline(always)]
    fn new() -> Self {
        Self
    }

    #[inline(always)]
    fn distance(&mut self, query: &str, candidates: &[&str]) {
        for candidate in candidates {
            let _ = best_match(query, candidate);
        }
    }

    #[inline(always)]
    fn distance_and_matches(&mut self, query: &str, candidates: &[&str]) {
        self.distance(query, candidates);
    }
}
