//! # Generic

use super::*;
use std::collections::VecDeque;

/// Alias for a [`VecDeque`] describing a list of zeros for a filter.
///
/// [`VecDeque`]: https://doc.rust-lang.org/std/collections/struct.VecDeque.html
pub type Zeros = VecDeque<(usize, Sample)>;

/// Alias for a [`VecDeque`] describing a list of poles for a filter.
///
/// [`VecDeque`]: https://doc.rust-lang.org/std/collections/struct.VecDeque.html
pub type Poles = VecDeque<(usize, Sample)>;

/// Alias for a [`VecDeque`] describing a list of samples for a filter.
///
/// [`VecDeque`]: https://doc.rust-lang.org/std/collections/struct.VecDeque.html
pub type Samples = VecDeque<Sample>;

/// Generic filter object.
pub struct Generic {
    zeros: Zeros,
    poles: Poles,
    inputs: Samples,
    outputs: Samples,
}

impl Generic {
    /// Creates a new Generic filter from the given pole and zero parameters.
    pub fn new(zeros: Zeros, poles: Poles) -> Generic {
        let z_back = if let Some(z) = zeros.back() {
            z.0 + 1
        } else {
            0
        };
        let p_back = if let Some(p) = poles.back() {
            p.0 + 1
        } else {
            0
        };
        Generic {
            zeros,
            poles,
            inputs: {
                let mut v = Samples::new();
                for _ in 0..z_back {
                    v.push_back(Sample::default());
                }
                v
            },
            outputs: {
                let mut v = Samples::new();
                for _ in 0..p_back {
                    v.push_back(Sample::default());
                }
                v
            },
        }
    }
}

impl Modifier for Generic {
    fn process(&mut self, x: Sample) -> Sample {
        let mut y = Sample::default();

        self.inputs.push_front(x);
        self.inputs.pop_back();

        for z in &self.zeros {
            y.0 += self.inputs[z.0].0 * z.1 .0;
        }
        for p in &self.poles {
            y.0 += self.outputs[p.0].0 * p.1 .0;
        }

        self.outputs.push_front(y);
        self.outputs.pop_back();

        y
    }
}

impl BlockModifier for Generic {
    fn process_block(&mut self, x: &[Sample], y: &mut [Sample]) {
        for (x, y) in x.iter().zip(y.iter_mut()) {
            *y = Sample::default();

            self.inputs.push_front(*x);
            self.inputs.pop_back();

            for z in &self.zeros {
                y.0 += self.inputs[z.0].0 * z.1 .0;
            }
            for p in &self.poles {
                y.0 += self.outputs[p.0].0 * p.1 .0;
            }

            self.outputs.push_front(*y);
            self.outputs.pop_back();
        }
    }
}

impl Clone for Generic {
    fn clone(&self) -> Self {
        Generic {
            zeros: self.zeros.clone(),
            poles: self.poles.clone(),
            inputs: {
                let mut v = Samples::new();
                for _ in 0..self.inputs.len() {
                    v.push_back(Sample::default());
                }
                v
            },
            outputs: {
                let mut v = Samples::new();
                for _ in 0..self.outputs.len() {
                    v.push_back(Sample::default());
                }
                v
            },
        }
    }
}
