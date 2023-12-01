use core::ffi::CStr;

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

pub struct FzfQuery<'a> {
    _query: &'a CStr,
    pattern: *mut fzf_pattern_t,
}

impl<'a> FzfQuery<'a> {
    #[inline]
    pub fn parse(query: &'a CStr) -> Self {
        let pattern = unsafe {
            fzf_parse_pattern(fzf_case_types_CaseSmart, false, query.as_ptr() as _, true)
        };

        Self {
            pattern,
            _query: query,
        }
    }
}

impl Drop for FzfQuery<'_> {
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
