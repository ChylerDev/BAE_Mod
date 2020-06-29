//! # Gain
//!
//! Simple amplification filter.

use super::*;

/// Simple gain filter, the simplest filter of all.
#[derive(Default, Copy, Clone)]
pub struct Gain {
    /// The gain of the gain filter.
    pub a: Sample,
}

impl Gain {
    /// Constructs a new gain filter from the given gain.
    pub fn new(a: Sample) -> Gain {
        Gain { a }
    }
}

impl Modifier for Gain {
    fn process(&mut self, x: Sample) -> Sample {
        (x.0 * self.a.0).into()
    }
}
