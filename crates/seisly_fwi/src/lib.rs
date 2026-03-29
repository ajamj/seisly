//! Full Waveform Inversion (FWI)
//!
//! Acoustic and elastic FWI for velocity model building.

pub mod acoustic;
pub mod elastic;
pub mod misfit;
pub mod gradient;

pub use acoustic::AcousticFWI;
pub use elastic::ElasticFWI;
pub use misfit::MisfitFunction;
pub use gradient::GradientCalculator;
