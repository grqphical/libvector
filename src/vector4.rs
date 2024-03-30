use crate::Vector;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Vector4 {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
}

impl Vector4 {
    /// Create a new 4D vector
    ///
    /// ## Arguments
    ///
    /// * `a` - The a component of the vector
    /// * `b` - The b component of the vector
    /// * `c` - The c component of the vector
    /// * `d` - The d component of the vector
    ///
    /// ## Returns
    ///
    /// A new 4D vector
    ///
    /// ## Example
    ///
    /// ```
    /// use libvector::Vector4;
    ///
    /// let a = Vector4::new(1., 2., 3., 4.);
    /// ```
    pub fn new(a: f64, b: f64, c: f64, d: f64) -> Self {
        Self { a, b, c, d }
    }
}

impl Vector for Vector4 {
    /// Calculate the dot product of two vectors
    ///
    /// ## Arguments
    ///
    /// * `other` - The other vector to calculate the dot product with
    ///
    /// ## Returns
    ///
    /// The dot product of the two vectors
    ///
    /// ## Example
    ///
    /// ```
    /// use libvector::{Vector4, Vector};
    ///
    /// let a = Vector4::new(1., 2., 3., 4.);
    ///
    /// let b = Vector4::new(5., 6., 7., 8.);
    ///
    /// let dot = a.dot(&b);
    ///
    /// assert_eq!(dot, 70.);
    /// ```
    fn dot(&self, other: &Self) -> f64 {
        self.a * other.a + self.b * other.b + self.c * other.c + self.d * other.d
    }

    /// Calculate the magnitude of the vector
    ///
    /// ## Returns
    ///
    /// The magnitude of the vector
    ///
    /// ## Example
    ///
    /// ```
    /// use libvector::{Vector4, Vector};
    ///
    /// let a = Vector4::new(1., 2., 3., 4.);
    ///
    /// let mag = a.magnitude();
    ///
    /// assert_eq!(mag, 5.477225575051661);
    /// ```
    fn magnitude(&self) -> f64 {
        (self.a * self.a + self.b * self.b + self.c * self.c + self.d * self.d).sqrt()
    }

    /// Normalize the vector
    ///
    /// ## Returns
    ///
    /// The normalized vector
    ///
    /// ## Example
    ///
    /// ```
    /// use libvector::{Vector4, Vector};
    ///
    /// let a = Vector4::new(1., 2., 3., 4.);
    ///
    /// let norm = a.normalize();
    ///
    /// assert_eq!(norm, Vector4::new(0.18257418583505536, 0.3651483716701107, 0.5477225575051661, 0.7302967433402214));
    /// ```
    fn normalize(&self) -> Self {
        let mag = self.magnitude();
        Vector4 {
            a: self.a / mag,
            b: self.b / mag,
            c: self.c / mag,
            d: self.d / mag,
        }
    }
}

impl Add for Vector4 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector4 {
            a: self.a + other.a,
            b: self.b + other.b,
            c: self.c + other.c,
            d: self.d + other.d,
        }
    }
}

impl Sub for Vector4 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector4 {
            a: self.a - other.a,
            b: self.b - other.b,
            c: self.c - other.c,
            d: self.d - other.d,
        }
    }
}

impl Mul<f64> for Vector4 {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Vector4 {
            a: self.a * scalar,
            b: self.b * scalar,
            c: self.c * scalar,
            d: self.d * scalar,
        }
    }
}

impl Div<f64> for Vector4 {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Vector4 {
            a: self.a / scalar,
            b: self.b / scalar,
            c: self.c / scalar,
            d: self.d / scalar,
        }
    }
}

impl From<[f64; 4]> for Vector4 {
    fn from(array: [f64; 4]) -> Self {
        Vector4::new(array[0], array[1], array[2], array[3])
    }
}

impl From<Vector4> for [f64; 4] {
    fn from(vector: Vector4) -> Self {
        [vector.a, vector.b, vector.c, vector.d]
    }
}

impl From<(f64, f64, f64, f64)> for Vector4 {
    fn from(tuple: (f64, f64, f64, f64)) -> Self {
        Vector4::new(tuple.0, tuple.1, tuple.2, tuple.3)
    }
}

impl From<Vector4> for (f64, f64, f64, f64) {
    fn from(vector: Vector4) -> Self {
        (vector.a, vector.b, vector.c, vector.d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize() {
        let a = Vector4::new(1., 2., 3., 4.);
        let norm = a.normalize();
        assert_eq!(
            norm,
            Vector4::new(
                0.18257418583505536,
                0.3651483716701107,
                0.5477225575051661,
                0.7302967433402214
            )
        );
    }

    #[test]
    fn test_add() {
        let a = Vector4::new(1., 2., 3., 4.);
        let b = Vector4::new(5., 6., 7., 8.);
        let result = a + b;
        assert_eq!(result, Vector4::new(6., 8., 10., 12.));
    }

    #[test]
    fn test_sub() {
        let a = Vector4::new(5., 6., 7., 8.);
        let b = Vector4::new(1., 2., 3., 4.);
        let result = a - b;
        assert_eq!(result, Vector4::new(4., 4., 4., 4.));
    }

    #[test]
    fn test_mul() {
        let a = Vector4::new(1., 2., 3., 4.);
        let scalar = 2.0;
        let result = a * scalar;
        assert_eq!(result, Vector4::new(2., 4., 6., 8.));
    }

    #[test]
    fn test_div() {
        let a = Vector4::new(4., 6., 8., 10.);
        let scalar = 2.0;
        let result = a / scalar;
        assert_eq!(result, Vector4::new(2., 3., 4., 5.));
    }

    #[test]
    fn test_from_array() {
        let array = [1., 2., 3., 4.];
        let result: Vector4 = array.into();
        assert_eq!(result, Vector4::new(1., 2., 3., 4.));
    }

    #[test]
    fn test_into_array() {
        let vector = Vector4::new(1., 2., 3., 4.);
        let result: [f64; 4] = vector.into();
        assert_eq!(result, [1., 2., 3., 4.]);
    }

    #[test]
    fn test_from_tuple() {
        let tuple = (1., 2., 3., 4.);
        let result: Vector4 = tuple.into();
        assert_eq!(result, Vector4::new(1., 2., 3., 4.));
    }

    #[test]
    fn test_into_tuple() {
        let vector = Vector4::new(1., 2., 3., 4.);
        let result: (f64, f64, f64, f64) = vector.into();
        assert_eq!(result, (1., 2., 3., 4.));
    }
}
