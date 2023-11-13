use std::ffi::{c_char, CStr, CString};

use crate::bindings::*;

pub struct Fzf {
    slab: *mut fzf_slab_t,
}

impl Default for Fzf {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Fzf {
    #[inline]
    pub fn new() -> Self {
        Self {
            slab: unsafe { fzf_make_default_slab() },
        }
    }

    #[inline]
    pub fn score(&mut self, query: &FzfQuery, candidate: &CStr) -> i32 {
        unsafe { fzf_get_score(candidate.as_ptr(), query.pattern, self.slab) }
    }

    #[inline]
    pub fn positions(&mut self, query: &FzfQuery, candidate: &CStr) -> FzfPositions {
        FzfPositions {
            pos: { unsafe { fzf_get_positions(candidate.as_ptr(), query.pattern, self.slab) } },
        }
    }
}

impl Drop for Fzf {
    #[inline]
    fn drop(&mut self) {
        unsafe { fzf_free_slab(self.slab) }
    }
}

pub struct FzfQuery {
    _query: *mut c_char,
    pattern: *mut fzf_pattern_t,
}

impl FzfQuery {
    #[inline]
    pub fn parse(query: String) -> Self {
        let query = CString::new(query).unwrap().into_raw();

        Self {
            pattern: unsafe { fzf_parse_pattern(fzf_case_types_CaseSmart, false, query, true) },
            _query: query,
        }
    }
}

impl Drop for FzfQuery {
    #[inline]
    fn drop(&mut self) {
        unsafe { fzf_free_pattern(self.pattern) }
    }
}

pub struct FzfPositions {
    pos: *mut fzf_position_t,
}

impl Drop for FzfPositions {
    #[inline]
    fn drop(&mut self) {
        unsafe { fzf_free_positions(self.pos) }
    }
}
