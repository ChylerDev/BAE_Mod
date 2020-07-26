//! # Band Pass

use super::*;

/// -12dB per octave BandPass filter.
pub struct BandPass {
    coeffs: [Math; 3],

    xn: [Sample; 2],
    yn: [Sample; 2],

    sample_rate: Math,

    central_f: Math,
    quality: Math,
}

impl BandPass {
    /// Creates a new BandPass object from the given central frequency and q
    /// value.
    ///
    /// The filter's quality is set to the central frequency divided by the
    /// difference between the corner frequencies.
    pub fn new(f: Math, q: Math, sample_rate: Math) -> Self {
        let mut bp = BandPass {
            coeffs: [Default::default(); 3],

            xn: [Default::default(); 2],
            yn: [Default::default(); 2],

            sample_rate,

            central_f: f,
            quality: q,
        };

        bp.reset();

        bp
    }

    /// Creates a new BandPass object from given corner frequencies.
    pub fn from_corners(f: (Math, Math), sample_rate: Math) -> Self {
        let mut bp = BandPass {
            coeffs: [Default::default(); 3],

            xn: [Default::default(); 2],
            yn: [Default::default(); 2],

            sample_rate,

            central_f: (f.0 .0 * f.1 .0).abs().sqrt().into(),
            quality: ((f.0 .0 * f.1 .0).abs().sqrt() / (f.1 .0 - f.0 .0).abs()).into(),
        };

        bp.reset();

        bp
    }

    /// Returns the central frequency of the filter.
    pub fn get_central_frequency(&self) -> Math {
        self.central_f
    }

    /// Sets a new central frequency.
    pub fn set_central_frequency(&mut self, f: Math) {
        self.central_f = f;

        self.reset();
    }

    /// Returns the quality of the filter.
    pub fn get_quality(&self) -> Math {
        self.quality
    }

    /// Sets the quality of the filter.
    ///
    /// The filter's quality is set to the central frequency divided by the
    /// difference between the corner frequencies.
    pub fn set_quality(&mut self, q: Math) {
        self.quality = q;

        self.reset();
    }

    /// Returns the corner frequencies of the filter.
    pub fn get_corner_frequencies(&self) -> (Math, Math) {
        let b = -self.central_f.0 / self.quality.0;

        let (p, n) = quadratic(1.0, b, -self.central_f.0 * self.central_f.0);
        let fl = if p > 0.0 { p } else { n };
        let fh = fl + b;

        if fl < fh {
            (fl.into(), fh.into())
        } else {
            (fh.into(), fl.into())
        }
    }

    /// Sets the corner frequencies of the filter.
    pub fn set_corner_frequencies(&mut self, f: (Math, Math)) {
        self.central_f = (f.0 .0 * f.1 .0).sqrt().into();
        self.quality = (self.central_f.0 / (f.0 .0 - f.1 .0).abs()).into();

        self.reset();
    }

    fn reset(&mut self) {
        let (fh, fl) = self.get_corner_frequencies();

        let theta_l = (std::f64::consts::PI * fl.0 / self.sample_rate.0).tan();
        let theta_h = (std::f64::consts::PI * fh.0 / self.sample_rate.0).tan();

        let al = 1.0 / (1.0 + theta_l);
        let ah = 1.0 / (1.0 + theta_h);

        let bl = (1.0 - theta_l) / (1.0 + theta_l);
        let bh = (1.0 - theta_h) / (1.0 + theta_h);

        self.coeffs[0] = ((1.0 - al) * ah).into();
        self.coeffs[1] = (bl + bh).into();
        self.coeffs[2] = (bl * bh).into();
    }
}

impl Modifier for BandPass {
    fn process(&mut self, x: Sample) -> Sample {
        let y = ((self.coeffs[0].0 * (x.0 - self.xn[1].0) as AccurateMath + self.coeffs[1].0 * self.yn[0].0 as AccurateMath
            - self.coeffs[2].0 * self.yn[1].0 as AccurateMath) as FastMath).into();

        self.xn.rotate_right(1);
        self.xn[0] = x;
        self.yn.rotate_right(1);
        self.yn[0] = y;

        y
    }
}

impl BlockModifier for BandPass {
    fn process_block(&mut self, x_in: &[Sample], y_out: &mut[Sample]) {
        for (x, y) in x_in.iter().zip(y_out.iter_mut()) {
            *y = (
                (
                    self.coeffs[0].0 * (x.0 - self.xn[1].0) as AccurateMath +
                    self.coeffs[1].0 * self.yn[0].0 as AccurateMath -
                    self.coeffs[2].0 * self.yn[1].0 as AccurateMath
                ) as FastMath
            ).into();

            self.xn.rotate_right(1);
            self.xn[0] = *x;
            self.yn.rotate_right(1);
            self.yn[0] = *y;
        }
    }
}

fn quadratic(a: AccurateMath, b: AccurateMath, c: AccurateMath) -> (AccurateMath, AccurateMath) {
    (
        (-b + (b * b - 4.0 * a * c).sqrt()) / (2.0 * a),
        (-b - (b * b - 4.0 * a * c).sqrt()) / (2.0 * a),
    )
}

impl Clone for BandPass {
    fn clone(&self) -> Self {
        BandPass {
            coeffs: self.coeffs,

            xn: [Default::default(); 2],
            yn: [Default::default(); 2],

            sample_rate: self.sample_rate,

            central_f: self.central_f,
            quality: self.quality,
        }
    }
}
