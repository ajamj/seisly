//! Full Waveform Inversion (FWI)
//!
//! Acoustic and elastic FWI for velocity model building.

pub mod acoustic;
pub mod elastic;
pub mod gradient;
pub mod misfit;

pub use acoustic::AcousticFWI;
pub use elastic::ElasticFWI;
pub use gradient::GradientCalculator;
pub use misfit::MisfitFunction;
