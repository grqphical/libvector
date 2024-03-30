/// A 2D vector struct
use crate::Vector;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    /// Creates a new 2D vector
    ///
    /// ## Arguments
    ///
    /// * `x` - The x component of the vector
    /// * `y` - The y component of the vector
    ///
    /// ## Returns
    ///
    /// A new 2D vector
    ///
    /// ## Example
    ///
    /// ```
    /// use libvector::Vector2;
    ///
    /// let a = Vector2::new(1., 2.);
    /// ```
    pub fn new(x: f64, y: f64) -> Vector2 {
        Vector2 { x, y }
    }

    /// Calculates the cross product of two vectors
    ///
    /// The cross product of two vectors is a vector that is perpendicular to both input vectors.
    /// In a 2D space, the cross product of two vectors can be calculated as the determinant of
    /// the matrix formed by the two vectors:
    ///
    /// | self.x  self.y |
    /// | other.x other.y |
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
    /// use libvector::{Vector2, Vector};
    ///
    /// let a = Vector2 { x: 1., y: 2. };
    /// let b = Vector2 { x: 3., y: 4. };
    ///
    /// let cross = a.cross(&b);
    ///
    /// assert_eq!(cross, -2.);
    /// ```
    pub fn cross(&self, other: &Self) -> f64 {
        self.x * other.y - self.y * other.x
    }
}

impl Vector for Vector2 {
    /// Calculates the dot product of two vectors
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
    /// use libvector::{Vector2, Vector};
    ///
    /// let a = Vector2 { x: 1., y: 2. };
    /// let b = Vector2 { x: 3., y: 4. };
    ///
    /// let dot = a.dot(&b);
    ///
    /// assert_eq!(dot, 11.);
    /// ```
    fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y
    }

    /// Calculates the magnitude of the vector
    ///
    /// The magnitude of a vector is the length of the vector. In a 2D space, the magnitude of a
    /// vector can be calculated as the square root of the sum of the squares of the vector's
    /// components:
    ///
    /// sqrt(x^2 + y^2)
    ///
    /// ## Returns
    ///
    /// The magnitude of the vector
    ///
    /// ## Example
    ///
    /// ```
    /// use libvector::{Vector2, Vector};
    ///
    /// let a = Vector2 { x: 3., y: 4. };
    ///
    /// let mag = a.magnitude();
    ///
    /// assert_eq!(mag, 5.);
    /// ```
    fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Normalizes the vector
    ///
    /// Normalizing a vector means to scale the vector so that its magnitude is 1. This is done by
    /// dividing each component of the vector by the magnitude of the vector.
    ///
    /// ## Returns
    ///
    /// The normalized vector
    ///
    /// ## Example
    ///
    /// ```
    /// use libvector::{Vector2, Vector};
    ///
    /// let a = Vector2 { x: 3., y: 4. };
    ///
    /// let norm = a.normalize();
    ///
    /// assert_eq!(norm.x, 3. / 5.);
    /// assert_eq!(norm.y, 4. / 5.);
    /// ```
    fn normalize(&self) -> Self {
        let mag = self.magnitude();
        Vector2 {
            x: self.x / mag,
            y: self.y / mag,
        }
    }
}

impl Add for Vector2 {
    type Output = Vector2;

    fn add(self, other: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vector2 {
    type Output = Vector2;

    fn sub(self, other: Vector2) -> Vector2 {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f64> for Vector2 {
    type Output = Vector2;

    fn mul(self, scalar: f64) -> Vector2 {
        Vector2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Div<f64> for Vector2 {
    type Output = Vector2;

    fn div(self, scalar: f64) -> Vector2 {
        Vector2 {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl From<(f64, f64)> for Vector2 {
    fn from(v: (f64, f64)) -> Self {
        Vector2 { x: v.0, y: v.1 }
    }
}

impl From<Vector2> for (f64, f64) {
    fn from(v: Vector2) -> Self {
        (v.x, v.y)
    }
}

impl From<[f64; 2]> for Vector2 {
    fn from(v: [f64; 2]) -> Self {
        Vector2 { x: v[0], y: v[1] }
    }
}

impl From<Vector2> for [f64; 2] {
    fn from(v: Vector2) -> Self {
        [v.x, v.y]
    }
}

#[cfg(test)]
mod tests {
    use crate::{Vector, Vector2};

    #[test]
    fn test_magnitude() {
        let a = Vector2 { x: 3., y: 4. };
        let mag = a.magnitude();
        assert_eq!(mag, 5.);
    }

    #[test]
    fn test_cross() {
        let a = Vector2 { x: 1., y: 2. };
        let b = Vector2 { x: 3., y: 4. };
        let cross = a.cross(&b);
        assert_eq!(cross, -2.);
    }

    #[test]
    fn test_normalize() {
        let a = Vector2 { x: 3., y: 4. };
        let norm = a.normalize();
        assert_eq!(norm.x, 3. / 5.);
        assert_eq!(norm.y, 4. / 5.);
    }

    #[test]
    fn test_add() {
        let a = Vector2 { x: 1., y: 2. };
        let b = Vector2 { x: 3., y: 4. };
        let result = a + b;
        assert_eq!(result.x, 4.);
        assert_eq!(result.y, 6.);
    }

    #[test]
    fn test_sub() {
        let a = Vector2 { x: 5., y: 6. };
        let b = Vector2 { x: 2., y: 3. };
        let result = a - b;
        assert_eq!(result.x, 3.);
        assert_eq!(result.y, 3.);
    }

    #[test]
    fn test_mul() {
        let a = Vector2 { x: 2., y: 3. };
        let scalar = 2.5;
        let result = a * scalar;
        assert_eq!(result.x, 5.);
        assert_eq!(result.y, 7.5);
    }

    #[test]
    fn test_div() {
        let a = Vector2 { x: 6., y: 8. };
        let scalar = 2.;
        let result = a / scalar;
        assert_eq!(result.x, 3.);
        assert_eq!(result.y, 4.);
    }

    #[test]
    fn test_equal() {
        let a = Vector2 { x: 1., y: 2. };
        let b = Vector2 { x: 1., y: 2. };
        assert_eq!(a, b);

        let b = Vector2 { x: 1., y: 3. };
        assert_ne!(a, b);
    }

    #[test]
    fn test_ordering() {
        let a = Vector2 { x: 1., y: 2. };
        let b = Vector2 { x: 1., y: 3. };
        assert!(a < b);
        assert!(b > a);
    }

    #[test]
    fn test_dot() {
        let a = Vector2 { x: 1., y: 2. };
        let b = Vector2 { x: 3., y: 4. };
        let result = a.dot(&b);
        assert_eq!(result, 11.);
    }

    #[test]
    fn test_to_from_tuple() {
        let a = Vector2 { x: 1., y: 2. };
        let tuple: (f64, f64) = a.into();
        assert_eq!(tuple, (1., 2.));
        let b = Vector2::from(tuple);
        assert_eq!(a, b);
    }

    #[test]
    fn test_to_from_array() {
        let a = Vector2 { x: 1., y: 2. };
        let array: [f64; 2] = a.into();
        assert_eq!(array, [1., 2.]);
        let b = Vector2::from(array);
        assert_eq!(a, b);
    }
}
