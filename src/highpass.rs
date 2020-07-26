//! # HighPass
//!
//! 18dB/octave
//! Derived from 3rd Order Butterworth Low Pass Filter.

use super::*;

/// High pass filter adapted from the 3rd Order Butterworth Low Pass Filter with
/// resonance.
pub struct HighPass {
    an: [Sample; 4],
    xn: [Sample; 3],

    bn: [Sample; 3],
    yn: [Sample; 3],

    sample_rate: Math,

    fc: Math,
    r: Math,
}

impl HighPass {
    /// Creates a new high pass from the given cutoff frequency and resonance
    /// values.
    ///
    /// # Parameters
    ///
    /// * `fc` - The cutoff frequency.
    /// * `r` - The resonance of the filter. Value should be in the range [0,1].
    /// If the value falls out of that range it is clamped to the closer value.
    pub fn new(fc: Math, r: Math, sample_rate: Math) -> HighPass {
        let fc = fc.0.min(sample_rate.0 / 2.0).into();
        let r = r.0.min(1.0).max(0.0).into();
        let mut hp = HighPass {
            an: [Default::default(); 4],
            bn: [Default::default(); 3],

            xn: [Default::default(); 3],
            yn: [Default::default(); 3],

            sample_rate,

            fc,
            r,
        };

        hp.reset();

        hp
    }

    /// Returns the central frequency of the filter.
    pub fn get_central_frequency(&self) -> Math {
        self.fc
    }

    /// Sets the central frequency of the filter.
    pub fn set_central_frequency(&mut self, fc: Math) {
        let fc = fc.0.min(self.sample_rate.0 / 2.0).into();

        self.fc = fc;
        self.reset();
    }

    /// Returns the resonance of the filter.
    pub fn get_resonance(&self) -> Math {
        self.r
    }

    /// Sets the resonance of the filter. Value should be in the range [0,1].
    /// If the value falls out of that range it is clamped to the closer value.
    pub fn set_resonance(&mut self, r: Math) {
        let r = r.0.min(1.0).max(0.0).into();

        self.r = r;
        self.reset();
    }

    fn reset(&mut self) {
        let theta = std::f64::consts::PI * (4.0 - self.r.0) / 6.0;
        let k = 1.0 - 2.0 * theta.cos();
        let w = 2.0 * std::f64::consts::PI * self.fc.0;
        let t = w / self.sample_rate.0;
        let g = t.powf(3.0) + k * t.powf(2.0) + k * t + 1.0;

        self.an[0] = ((1.0 / g) as FastMath).into();
        self.an[1] = ((-3.0 / g) as FastMath).into();
        self.an[2] = ((3.0 / g) as FastMath).into();
        self.an[3] = ((-1.0 / g) as FastMath).into();

        self.bn[0] = (((k * t.powf(2.0) + 2.0 * k * t + 3.0) / g) as FastMath).into();
        self.bn[1] = (((-k * t - 3.0) / g) as FastMath).into();
        self.bn[2] = ((1.0 / g) as FastMath).into();
    }
}

impl Modifier for HighPass {
    fn process(&mut self, x: Sample) -> Sample {
        let y = (
            self.an[0].0 * x.0
            + self.an[1].0 * self.xn[0].0
            + self.an[2].0 * self.xn[1].0
            + self.an[3].0 * self.xn[2].0
            + self.bn[0].0 * self.yn[0].0
            + self.bn[1].0 * self.yn[1].0
            + self.bn[2].0 * self.yn[2].0
        ).into();

        self.xn.rotate_right(1);
        self.xn[0] = x;
        self.yn.rotate_right(1);
        self.yn[0] = y;

        y
    }
}

impl BlockModifier for HighPass {
    fn process_block(&mut self, x: &[Sample], y: &mut[Sample]) {
        for (x, y) in x.iter().zip(y.iter_mut()) {
            *y = (
                self.an[0].0 * x.0
                + self.an[1].0 * self.xn[0].0
                + self.an[2].0 * self.xn[1].0
                + self.an[3].0 * self.xn[2].0
                + self.bn[0].0 * self.yn[0].0
                + self.bn[1].0 * self.yn[1].0
                + self.bn[2].0 * self.yn[2].0
            ).into();

            self.xn.rotate_right(1);
            self.xn[0] = *x;
            self.yn.rotate_right(1);
            self.yn[0] = *y;
        }
    }
}

impl Clone for HighPass {
    fn clone(&self) -> Self {
        HighPass {
            an: self.an,
            bn: self.bn,

            xn: [Sample::default(); 3],
            yn: [Sample::default(); 3],

            sample_rate: self.sample_rate,

            fc: self.fc,
            r: self.r,
        }
    }
}
