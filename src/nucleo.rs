use nucleo_matcher::{
    pattern::{CaseMatching, Pattern},
    Matcher, Utf32Str,
};

use crate::{Metric, Parser};

impl Parser for Pattern {
    type Query<'a> = &'a Self;

    fn parse<'a>(&'a mut self, s: &str) -> &'a Self {
        self.reparse(s, CaseMatching::Smart);
        self
    }
}

#[derive(Default)]
pub struct Nucleo {
    matcher: Matcher,
    char_buf: Vec<char>,
    indices_buf: Vec<u32>,
}

impl Metric for Nucleo {
    const NAME: &'static str = "nucleo";

    type Parser = Pattern;

    #[inline(always)]
    fn new() -> Self {
        Self::default()
    }

    #[inline(always)]
    fn distance(&mut self, pattern: &Pattern, candidates: &[&str]) {
        for candidate in candidates {
            let haystack = Utf32Str::new(candidate, &mut self.char_buf);
            let _ = pattern.score(haystack, &mut self.matcher);
        }
    }

    #[inline(always)]
    fn distance_and_matches(&mut self, pattern: &Pattern, candidates: &[&str]) {
        for candidate in candidates {
            let haystack = Utf32Str::new(candidate, &mut self.char_buf);
            let _ = pattern.indices(haystack, &mut self.matcher, &mut self.indices_buf);
        }
    }
}
