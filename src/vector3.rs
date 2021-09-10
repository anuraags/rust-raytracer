use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn zero() -> Vector3 {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Vector3 {
        let length = self.length();
        Vector3 {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }
    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}
impl fmt::Display for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl std::ops::Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

impl std::ops::Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

#[test]
fn test_vector_length() {
    let vector = Vector3 {
        x: 3.0,
        y: 4.0,
        z: 5.0,
    };
    assert_eq!(vector.length(), (50.0 as f64).sqrt());
}
#[test]
fn test_vector_normalize() {
    let vector = Vector3 {
        x: 3.0,
        y: 4.0,
        z: 5.0,
    };
    let vector = vector.normalize();

    assert_eq!(
        vector,
        Vector3 {
            x: 3.0 / (50.0 as f64).sqrt(),
            y: 4.0 / (50.0 as f64).sqrt(),
            z: 5.0 / (50.0 as f64).sqrt()
        }
    );
}
