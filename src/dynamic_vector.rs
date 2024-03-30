use crate::Vector;

/// A Vector that can be expanded to any length
///
/// This is a dynamic vector that can be expanded to any length. It is implemented using a Vec<f64>
///
/// **NOTE:** All operations done with this vector will have a time complexity of **O(n)** where **n** is the length of the vector
/// if you need a more performant custom Vector, consider using the `vector!` macro
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct DynamicVector {
    data: Vec<f64>,
}

impl DynamicVector {
    /// Create a new dynamic vector
    ///
    /// ## Arguments
    ///
    /// * `length` - The length of the vector
    ///
    /// ## Returns
    ///
    /// A new dynamic vector of the specified length
    pub fn new(length: usize) -> Self {
        let mut data = Vec::with_capacity(length);

        for _ in 0..length {
            data.push(0.0);
        }

        DynamicVector { data }
    }

    /// Gets a value from the Vector
    ///
    /// ## Arguments
    ///
    /// * `index` - The index of the value to get
    ///
    /// ## Returns
    ///
    /// The value at the specified index
    pub fn get(&self, index: usize) -> f64 {
        self.data[index]
    }

    /// Sets a value in the Vector
    ///
    /// ## Arguments
    ///
    /// * `index` - The index of the value to set
    pub fn set(&mut self, index: usize, value: f64) {
        self.data[index] = value;
    }
}

impl Vector for DynamicVector {
    /// Calculate the dot product of two vectors
    ///
    /// ## Arguments
    ///
    /// * `other` - The other vector to calculate the dot product with
    ///
    /// ## Returns
    ///
    /// The dot product of the two vectors
    fn dot(&self, other: &Self) -> f64 {
        self.data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a * b)
            .sum()
    }

    /// Calculate the magnitude of the vector
    ///
    /// ## Returns
    ///
    /// The magnitude of the vector
    fn magnitude(&self) -> f64 {
        self.data.iter().map(|a| a * a).sum::<f64>().sqrt()
    }

    /// Normalize the vector
    ///
    /// ## Returns
    ///
    /// A new vector that is the normalized version of the original vector
    fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        DynamicVector {
            data: self.data.iter().map(|a| a / magnitude).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let vector = DynamicVector::new(5);
        assert_eq!(vector.data.len(), 5);
        assert_eq!(vector.data.capacity(), 5);
    }

    #[test]
    fn test_get() {
        let mut vector = DynamicVector::new(3);
        vector.data = vec![1.0, 2.0, 3.0];
        assert_eq!(vector.get(0), 1.0);
        assert_eq!(vector.get(1), 2.0);
        assert_eq!(vector.get(2), 3.0);
    }

    #[test]
    fn test_set() {
        let mut vector = DynamicVector::new(3);
        vector.set(0, 1.0);
        vector.set(1, 2.0);
        vector.set(2, 3.0);
        assert_eq!(vector.data, vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_dot() {
        let vector1 = DynamicVector {
            data: vec![1.0, 2.0, 3.0],
        };
        let vector2 = DynamicVector {
            data: vec![4.0, 5.0, 6.0],
        };
        assert_eq!(vector1.dot(&vector2), 32.0);
    }

    #[test]
    fn test_magnitude() {
        let vector = DynamicVector {
            data: vec![3.0, 4.0],
        };
        assert_eq!(vector.magnitude(), 5.0);
    }

    #[test]
    fn test_normalize() {
        let vector = DynamicVector {
            data: vec![3.0, 4.0],
        };
        let normalized_vector = vector.normalize();
        assert_eq!(normalized_vector.data, vec![0.6, 0.8]);
    }
}
