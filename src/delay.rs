//! # Delay

use super::*;
use std::collections::VecDeque;
use std::time::Duration;

/// Delay modifier, delays a signal by a given amount of time rounded to the
/// nearest sample.
pub struct Delay {
    sample_rate: Math,
    delay: VecDeque<Sample>,
}

impl Delay {
    /// Creates a new Delay object from the given duration rounded to the
    /// nearest sample.
    pub fn new(d: Duration, sample_rate: Math) -> Self {
        Delay {
            sample_rate,
            delay: {
                let mut v = VecDeque::new();

                for _ in 0..((d.as_secs_f64() * sample_rate.0).round() as usize) {
                    v.push_back(Sample::default());
                }

                v
            },
        }
    }

    /// Returns the delay of the Modifier in a Duration value.
    pub fn get_delay(&self) -> Duration {
        Duration::from_secs_f64(self.delay.len() as AccurateMath / self.sample_rate.0)
    }
}

impl Modifier for Delay {
    fn process(&mut self, x: Sample) -> Sample {
        self.delay.push_back(x);

        self.delay.pop_front().unwrap()
    }
}

impl BlockModifier for Delay {
    fn process_block(&mut self, x: &[Sample], y: &mut [Sample]) {
        for (x, y) in x.iter().zip(y.iter_mut()) {
            self.delay.push_back(*x);
            *y = self.delay.pop_front().unwrap();
        }
    }
}

impl Clone for Delay {
    fn clone(&self) -> Self {
        Delay {
            sample_rate: self.sample_rate,
            delay: {
                let mut v = VecDeque::new();

                for _ in 0..(self.delay.len() * self.sample_rate.0 as usize) {
                    v.push_back(Sample::default());
                }

                v
            },
        }
    }
}
