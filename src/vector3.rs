use crate::Vector;
use std::convert::{From, Into};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    /// Create a new 3D vector
    ///
    /// ## Arguments
    ///
    /// * `x` - The x component of the vector
    /// * `y` - The y component of the vector
    /// * `z` - The z component of the vector
    ///
    /// ## Returns
    ///
    /// A new 3D vector
    ///
    /// ## Example
    ///
    /// ```
    /// use libvector::Vector3;
    ///
    /// let a = Vector3::new(1., 2., 3.);
    /// ```
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x, y, z }
    }

    /// Calculates the cross product of two vectors
    ///
    /// The cross product of two vectors is a vector that is perpendicular to both input vectors.
    /// In a 3D space, the cross product of two vectors can be calculated as the determinant of
    /// the matrix formed by the two vectors:
    ///
    /// | i  j  k  |
    /// | x1 y1 z1 |
    /// | x2 y2 z2 |
    ///
    /// ## Arguments
    ///
    /// * `other` - The other vector to calculate the cross product with
    ///
    /// ## Returns
    ///
    /// The cross product of the two vectors
    ///
    /// ## Example
    ///
    /// ```
    /// use libvector::{Vector3, Vector};
    ///
    /// let a = Vector3 { x: 1., y: 2., z: 3. };
    ///
    /// let b = Vector3 { x: 4., y: 5., z: 6. };
    ///
    /// let cross = a.cross(&b);
    ///
    /// assert_eq!(cross, Vector3 { x: -3., y: 6., z: -3. });
    /// ```
    pub fn cross(&self, other: &Self) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Vector for Vector3 {
    fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn normalize(&self) -> Self {
        let mag = self.magnitude();
        Vector3 {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }
}

impl From<[f64; 3]> for Vector3 {
    fn from(arr: [f64; 3]) -> Self {
        Vector3::new(arr[0], arr[1], arr[2])
    }
}

impl Into<[f64; 3]> for Vector3 {
    fn into(self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
}

impl From<(f64, f64, f64)> for Vector3 {
    fn from(tuple: (f64, f64, f64)) -> Self {
        Vector3::new(tuple.0, tuple.1, tuple.2)
    }
}

impl Into<(f64, f64, f64)> for Vector3 {
    fn into(self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Vector3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl Div<f64> for Vector3 {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Vector3::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cross() {
        let a = Vector3 {
            x: 1.,
            y: 2.,
            z: 3.,
        };
        let b = Vector3 {
            x: 4.,
            y: 5.,
            z: 6.,
        };
        let cross = a.cross(&b);
        assert_eq!(
            cross,
            Vector3 {
                x: -3.,
                y: 6.,
                z: -3.
            }
        );
    }

    #[test]
    fn test_dot() {
        let a = Vector3 {
            x: 1.,
            y: 2.,
            z: 3.,
        };
        let b = Vector3 {
            x: 4.,
            y: 5.,
            z: 6.,
        };
        let dot_product = a.dot(&b);
        assert_eq!(dot_product, 32.);
    }

    #[test]
    fn test_magnitude() {
        let vector = Vector3 {
            x: 1.,
            y: 2.,
            z: 3.,
        };
        let magnitude = vector.magnitude();
        assert_eq!(magnitude, (14.0 as f64).sqrt());
    }

    #[test]
    fn test_normalize() {
        let vector = Vector3 {
            x: 1.,
            y: 2.,
            z: 3.,
        };
        let normalized = vector.normalize();
        let magnitude = normalized.magnitude();
        assert_eq!(magnitude, 1.);
    }

    #[test]
    fn test_from_array() {
        let arr = [1., 2., 3.];
        let vector: Vector3 = arr.into();
        assert_eq!(
            vector,
            Vector3 {
                x: 1.,
                y: 2.,
                z: 3.
            }
        );
    }

    #[test]
    fn test_into_array() {
        let vector = Vector3 {
            x: 1.,
            y: 2.,
            z: 3.,
        };
        let arr: [f64; 3] = vector.into();
        assert_eq!(arr, [1., 2., 3.]);
    }

    #[test]
    fn test_from_tuple() {
        let tuple = (1., 2., 3.);
        let vector: Vector3 = tuple.into();
        assert_eq!(
            vector,
            Vector3 {
                x: 1.,
                y: 2.,
                z: 3.
            }
        );
    }

    #[test]
    fn test_into_tuple() {
        let vector = Vector3 {
            x: 1.,
            y: 2.,
            z: 3.,
        };
        let tuple: (f64, f64, f64) = vector.into();
        assert_eq!(tuple, (1., 2., 3.));
    }

    #[test]
    fn test_addition() {
        let a = Vector3 {
            x: 1.,
            y: 2.,
            z: 3.,
        };
        let b = Vector3 {
            x: 4.,
            y: 5.,
            z: 6.,
        };
        let sum = a + b;
        assert_eq!(
            sum,
            Vector3 {
                x: 5.,
                y: 7.,
                z: 9.
            }
        );
    }

    #[test]
    fn test_subtraction() {
        let a = Vector3 {
            x: 1.,
            y: 2.,
            z: 3.,
        };
        let b = Vector3 {
            x: 4.,
            y: 5.,
            z: 6.,
        };
        let difference = a - b;
        assert_eq!(
            difference,
            Vector3 {
                x: -3.,
                y: -3.,
                z: -3.
            }
        );
    }

    #[test]
    fn test_scalar_multiplication() {
        let vector = Vector3 {
            x: 1.,
            y: 2.,
            z: 3.,
        };
        let scalar = 2.;
        let result = vector * scalar;
        assert_eq!(
            result,
            Vector3 {
                x: 2.,
                y: 4.,
                z: 6.
            }
        );
    }

    #[test]
    fn test_scalar_division() {
        let vector = Vector3 {
            x: 1.,
            y: 2.,
            z: 3.,
        };
        let scalar = 2.;
        let result = vector / scalar;
        assert_eq!(
            result,
            Vector3 {
                x: 0.5,
                y: 1.,
                z: 1.5
            }
        );
    }

    #[test]
    fn test_equality() {
        let a = Vector3 {
            x: 1.,
            y: 2.,
            z: 3.,
        };
        let b = Vector3 {
            x: 1.,
            y: 2.,
            z: 3.,
        };
        assert_eq!(a, b);

        let b = Vector3 {
            x: 1.,
            y: 2.,
            z: 4.,
        };

        assert_ne!(a, b);
    }

    #[test]
    fn test_ordering() {
        let a = Vector3 {
            x: 1.,
            y: 2.,
            z: 3.,
        };
        let b = Vector3 {
            x: 4.,
            y: 5.,
            z: 6.,
        };
        assert!(a < b);
        assert!(b > a);
    }
}
