//! # Modifiers
//!
//! Module including many of the common/basic filters including lowpass,
//! bandpass, echo, delay, etc.

#![warn(missing_docs)]
#![doc(html_root_url = "https://docs.rs/bae_mod/0.14.0")]

use bae_types::*;
use bae_utils::*;

pub mod adsr;
pub mod bandpass;
pub mod delay;
pub mod echo;
pub mod envelope;
pub mod gain;
pub mod generic;
pub mod highpass;
pub mod lowpass;
pub mod passthrough;

pub use adsr::*;
pub use bandpass::*;
pub use delay::*;
pub use echo::*;
pub use envelope::*;
pub use gain::*;
pub use generic::*;
pub use highpass::*;
pub use lowpass::*;
pub use passthrough::*;

/// The `Modifier` trait defines types that filter audio samples.
pub trait Modifier {
    /// Filters the given audio sample.
    ///
    /// # Parameters
    ///
    /// * `x` - The "dry" audio sample before filtering.
    fn process(&mut self, x: Sample) -> Sample;
}

/// The `BlockModifier` trait defines types that filter audio samples in blocks,
/// or chunks.
pub trait BlockModifier {
    /// Filters the given chunk of audio samples.
    ///
    /// # Parameters
    ///
    /// * `x` - The "dry" audio samples before filtering.
    /// * `y` - The mutable slice that will store the filtered samples. This
    ///         slice must be at minimum the same size as `x`.
    fn process_block(&mut self, x: &[Sample], y: &mut[Sample]);
}
