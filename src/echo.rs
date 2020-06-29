//! # Echo

use super::*;
use std::collections::VecDeque;
use std::time::Duration;

/// Simple Echo filter: H(z) = 1/(1-az^-d)
pub struct Echo {
    sample_rate: Math,
    delay: VecDeque<Sample>,
    gain: Sample,
}

impl Echo {
    /// Creates a new ['Echo'] object with the given delay duration and feedback
    /// amount.
    ///
    /// [`Echo`]: struct.Echo.html
    pub fn new(d: Duration, g: Math, sample_rate: Math) -> Self {
        let mut v = VecDeque::new();
        for _ in 0..((d.as_secs_f64() * sample_rate.0) as usize) {
            v.push_back(Sample::default());
        }

        Echo {
            sample_rate,
            delay: {
                let mut v = VecDeque::new();

                for _ in 0..((d.as_secs_f64() * sample_rate.0) as usize) {
                    v.push_back(Sample::default());
                }

                v
            },
            gain: Sample(g.0 as FastMath),
        }
    }
}

impl Modifier for Echo {
    fn process(&mut self, x: Sample) -> Sample {
        let wet = (self.delay.pop_front().unwrap().0 * self.gain.0 + x.0).into();
        self.delay.push_back(wet);

        wet
    }
}

impl Clone for Echo {
    fn clone(&self) -> Self {
        Echo {
            sample_rate: self.sample_rate,
            delay: {
                let mut v = VecDeque::new();

                for _ in 0..(self.delay.len() * self.sample_rate.0 as usize) {
                    v.push_back(Sample::default());
                }

                v
            },
            gain: self.gain,
        }
    }
}
