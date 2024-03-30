mod dynamic_vector;
mod vector2;
mod vector3;
mod vector4;

pub use vector2::*;
pub use vector3::*;
pub use vector4::*;
pub use vector_macro::vector_macro as vector;

/// Base trait for all vector types
pub trait Vector {
    fn dot(&self, other: &Self) -> f64;
    fn magnitude(&self) -> f64;
    fn normalize(&self) -> Self;
}
