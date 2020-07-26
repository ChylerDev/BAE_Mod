//! # Envelope

use super::*;

/// Envelope Follower filter. I don't remember my lectures well enough to write
/// a detailed description.
pub struct Envelope {
    sample_rate: Math,

    au: Sample,
    bu: Sample,
    ad: Sample,
    bd: Sample,

    x1: Sample,
    y1: Sample,
}

impl Envelope {
    /// Creates a new [`Envelope`] object from the given max and min frequencies
    /// to follow.
    ///
    /// [`Envelope`]: struct.Envelope.html
    pub fn new(lower: Math, upper: Math, sample_rate: Math) -> Envelope {
        let theta_u = (std::f64::consts::PI * upper.0 / sample_rate.0).tan();
        let theta_d = (std::f64::consts::PI * lower.0 / sample_rate.0).tan();

        Envelope {
            sample_rate,

            au: Sample((theta_u / (1.0 + theta_u)) as FastMath),
            bu: Sample(((1.0 - theta_u) / (1.0 + theta_u)) as FastMath),
            ad: Sample((theta_d / (1.0 + theta_d)) as FastMath),
            bd: Sample(((1.0 - theta_d) / (1.0 + theta_d)) as FastMath),

            x1: Sample::default(),
            y1: Sample::default(),
        }
    }
}

impl Modifier for Envelope {
    fn process(&mut self, x: Sample) -> Sample {
        let y = Sample(if x.0.abs() > self.y1.0 {
            self.au.0 * (x.0.abs() + self.x1.0.abs()) + self.bu.0 * self.y1.0
        } else {
            self.ad.0 * (x.0.abs() + self.x1.0.abs()) + self.bd.0 * self.y1.0
        });

        self.y1 = y;
        self.x1 = x;

        y
    }
}

impl BlockModifier for Envelope {
    fn process_block(&mut self, x: &[Sample], y: &mut [Sample]) {
        for (x, y) in x.iter().zip(y.iter_mut()) {
            *y = Sample(if x.0.abs() > self.y1.0 {
                self.au.0 * (x.0.abs() + self.x1.0.abs()) + self.bu.0 * self.y1.0
            } else {
                self.ad.0 * (x.0.abs() + self.x1.0.abs()) + self.bd.0 * self.y1.0
            });

            self.y1 = *y;
            self.x1 = *x;
        }
    }
}

impl Clone for Envelope {
    fn clone(&self) -> Self {
        Envelope {
            sample_rate: self.sample_rate,

            au: self.au,
            bu: self.bu,
            ad: self.ad,
            bd: self.bd,

            x1: Default::default(),
            y1: Default::default(),
        }
    }
}
