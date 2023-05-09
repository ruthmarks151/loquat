mod error_comparable;
pub use error_comparable::MeanErrorSquareComparable;
mod scaling;
pub use scaling::{ScalesTo, ScalesWith};
pub mod indexing;
mod interpolable;
pub use interpolable::Interpolable;
mod lenable;
pub use lenable::Lenable;
