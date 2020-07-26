//! # HighPass
//!
//! 18dB/octave
//! Derived from 3rd Order Butterworth Low Pass Filter.

use super::*;

/// 3rd Order Butterworth Low Pass Filter with resonance.
pub struct LowPass {
    coeff: [Sample; 4],

    yn: [Sample; 3],

    sample_rate: Math,

    fc: Math,
    r: Math,
}

impl LowPass {
    /// Creates a new low pass from the given cutoff frequency and resonance
    /// values.
    ///
    /// # Parameters
    ///
    /// * `fc` - The cutoff frequency.
    /// * `r` - The resonance of the filter. Value should be in the range [0,1].
    /// If the value falls out of that range it is clamped to the closer value.
    pub fn new(fc: Math, r: Math, sample_rate: Math) -> LowPass {
        let fc = fc.0.min(sample_rate.0 / 2.0).into();
        let r = r.0.min(1.0).max(0.0).into();

        let mut lp = LowPass {
            coeff: [Sample::default(); 4],
            yn: [Sample::default(); 3],

            sample_rate,

            fc,
            r,
        };

        lp.reset();

        lp
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

    /// Sets the resonance of the filter.
    pub fn set_resonance(&mut self, r: Math) {
        let r = r.0.min(1.0).max(0.0).into();

        self.r = r;
        self.reset();
    }

    fn reset(&mut self) {
        let theta = (std::f64::consts::PI / 6.0) * (4.0 - self.r.0);
        let k = 1.0 - 2.0 * theta.cos();
        let w = 2.0 * std::f64::consts::PI * self.fc.0;
        let t = w / self.sample_rate.0;
        let g = t.powf(3.0) + k * t.powf(2.0) + k * t + 1.0;

        self.coeff[0] = ((t.powf(3.0) / g) as FastMath).into();
        self.coeff[1] = (((k * t.powf(2.0) + 2.0 * k * t + 3.0) / g) as FastMath).into();
        self.coeff[2] = (((-k * t - 3.0) / g) as FastMath).into();
        self.coeff[3] = ((1.0 / g) as FastMath).into();
    }
}

impl Modifier for LowPass {
    fn process(&mut self, x: Sample) -> Sample {
        let y = (
            self.coeff[0].0 * x.0
            + self.coeff[1].0 * self.yn[0].0
            + self.coeff[2].0 * self.yn[1].0
            + self.coeff[3].0 * self.yn[2].0
        ).into();

        self.yn.rotate_right(1);
        self.yn[0] = y;

        y
    }
}

impl BlockModifier for LowPass {
    fn process_block(&mut self, x: &[Sample], y: &mut[Sample]) {
        for (x, y) in x.iter().zip(y.iter_mut()) {
            *y = (
                self.coeff[0].0 * x.0
                + self.coeff[1].0 * self.yn[0].0
                + self.coeff[2].0 * self.yn[1].0
                + self.coeff[3].0 * self.yn[2].0
            ).into();

            self.yn.rotate_right(1);
            self.yn[0] = *y;
        }
    }
}

impl Clone for LowPass {
    fn clone(&self) -> Self {
        LowPass {
            coeff: self.coeff,
            yn: [Sample::default(); 3],

            sample_rate: self.sample_rate,

            fc: self.fc,
            r: self.r,
        }
    }
}
