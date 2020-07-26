//! # ADSR

use super::*;

use std::time::Duration;

/// Enum of the states an ADSR filter exists in.
pub enum ADSRState {
    /// Attack state.
    Attack,
    /// Decay state.
    Decay,
    /// Sustain state.
    Sustain,
    /// Release state.
    Release,
    /// State for when the ADSR has completed it's processing.
    Stopped,
}

/// Attack-decay-sustain-release filter.
///
/// Creates a simple envelope for the given signal.
pub struct ADSR {
    a: Math,
    d: Math,
    s: Math,
    r: Math,

    sample_rate: Math,

    state: ADSRState,
    g: Math,
}

impl ADSR {
    /// Constructs an ADSR filter object.
    ///
    /// # Parameters
    ///
    /// * `a` - Attack time.
    /// * `d` - Decay time.
    /// * `s` - Sustain level in dBFS. Value is clamped to be less than 0.
    /// * `r` - Release time.
    /// * `sample_rate` - Sample rate of the engine.
    pub fn new(a: Duration, d: Duration, s: Math, r: Duration, sample_rate: Math) -> Self {
        let s = s.0.min(0.0).into();
        ADSR {
            a: (1.0 / (a.as_secs_f64() * sample_rate.0)).into(),
            d: ((db_to_linear(s).0 - 1.0) / (d.as_secs_f64() * sample_rate.0)).into(),
            s: db_to_linear(s),
            r: ((-db_to_linear(s).0) / (r.as_secs_f64() * sample_rate.0)).into(),
            sample_rate,
            state: ADSRState::Attack,
            g: Default::default(),
        }
    }

    /// Sets the attack time.
    pub fn attack(&mut self, a: Duration) {
        self.a.0 = 1.0 / (a.as_secs_f64() * self.sample_rate.0);
    }

    /// Sets the decay time.
    pub fn decay(&mut self, d: Duration) {
        self.d.0 = (self.s.0 - 1.0) / (d.as_secs_f64() * self.sample_rate.0);
    }

    /// Sets the sustain level in dBFS.
    pub fn sustain(&mut self, s: Math) {
        self.d.0 *= (db_to_linear(s).0 - 1.0) / (self.s.0 - 1.0);
        self.r.0 *= db_to_linear(s).0 / self.s.0;
        self.s.0 = db_to_linear(s).0;
    }

    /// Sets the release time.
    pub fn release(&mut self, r: Duration) {
        self.r.0 = -self.s.0 / (r.as_secs_f64() * self.sample_rate.0);
    }

    /// Changes state to release
    pub fn trigger_release(&mut self) {
        self.state = ADSRState::Release;
    }
}

impl Modifier for ADSR {
    fn process(&mut self, x: Sample) -> Sample {
        match self.state {
            ADSRState::Attack => {
                self.g.0 += self.a.0;
                if self.g.0 >= 1.0 {
                    self.state = ADSRState::Decay;
                    self.g.0 = 1.0;
                }

                (x.0 * self.g.0 as FastMath).into()
            }
            ADSRState::Decay => {
                self.g.0 += self.d.0;
                if self.g <= self.s {
                    self.state = ADSRState::Sustain;
                    self.g = self.s;
                }

                (x.0 * self.g.0 as FastMath).into()
            }
            ADSRState::Sustain => (x.0 * self.g.0 as FastMath).into(),
            ADSRState::Release => {
                self.g.0 += self.r.0;
                if self.g.0 <= 0.0 {
                    self.state = ADSRState::Stopped;
                    self.g.0 = 0.0;
                }

                (x.0 * self.g.0 as FastMath).into()
            }
            ADSRState::Stopped => Sample::default(),
        }
    }
}

impl BlockModifier for ADSR {
    fn process_block(&mut self, x_in: &[Sample], y_out: &mut[Sample]) {
        for (x, y) in x_in.iter().zip(y_out.iter_mut()) {
            if self.state == ADSRState::Stopped {
                *y = Sample::default();
                continue;
            }
            match self.state {
                ADSRState::Attack => {
                    self.g.0 += self.a.0;
                    if self.g.0 >= 1.0 {
                        self.state = ADSRState::Decay;
                        self.g.0 = 1.0;
                    }
                },
                ADSRState::Decay => {
                    self.g.0 += self.d.0;
                    if self.g <= self.s {
                        self.state = ADSRState::Sustain;
                        self.g = self.s;
                    }
                },
                ADSRState::Release => {
                    self.g.0 += self.r.0;
                    if self.g.0 <= 0.0 {
                        self.state = ADSRState::Stopped;
                        self.g.0 = 0.0;
                    }
                },
                _ => (),
            };

            *y = (x.0 * self.g.0 as FastMath).into();
        }
    }
}

impl Clone for ADSR {
    fn clone(&self) -> Self {
        ADSR {
            a: self.a,
            d: self.d,
            s: self.s,
            r: self.r,

            sample_rate: self.sample_rate,

            state: ADSRState::Attack,
            g: Default::default(),
        }
    }
}
