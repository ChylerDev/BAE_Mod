//! # Passthrough

use super::*;

/// Passthrough filter, does nothing to the input.
#[derive(Copy, Clone, Default, Debug)]
pub struct Passthrough {}

impl Passthrough {
    /// Creates new Passthrough object.
    pub fn new() -> Passthrough {
        Passthrough {}
    }
}

impl Modifier for Passthrough {
    fn process(&mut self, x: Sample) -> Sample {
        x
    }
}

impl BlockModifier for Passthrough {
    fn process_block(&mut self, x: &[Sample], y: &mut [Sample]) {
        for (x, y) in x.iter().zip(y.iter_mut()) {
            *y = *x;
        }
    }
}
