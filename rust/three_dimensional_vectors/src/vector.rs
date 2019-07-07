#[derive(Copy, Clone)]
pub struct Vector {
    pub i: f64,
    pub j: f64,
    pub k: f64,
}

fn is_fuzzy_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < 0.000001
}

// Static
#[allow(dead_code)]
impl Vector {
    pub fn new(i: f64, j: f64, k: f64) -> Self {
        Vector { i, j, k }
    }

    pub fn get_i() -> Vector {
        Vector::new(1.0, 0.0, 0.0)
    }

    pub fn get_j() -> Vector {
        Vector::new(0.0, 1.0, 0.0)
    }

    pub fn get_k() -> Vector {
        Vector::new(0.0, 0.0, 1.0)
    }
}

#[allow(dead_code)]
impl Vector {
    fn is_zero(&self) -> bool {
        is_fuzzy_eq(self.get_magnitude(), 0.0)
    }

    pub fn get_magnitude(&self) -> f64 {
        (self.i.powi(2) + self.j.powi(2) + self.k.powi(2)).sqrt()
    }

    pub fn multiply_by_scalar(&self, scale: f64) -> Self {
        Vector::new(self.i * scale, self.j * scale, self.k * scale)
    }

    pub fn dot(&self, other: Self) -> f64 {
        self.i
            .mul_add(other.i, self.j.mul_add(other.j, self.k * other.k))
    }

    pub fn cross(&self, other: Self) -> Vector {
        Vector::new(
            self.j.mul_add(other.k, -(self.k * other.j)),
            self.k.mul_add(other.i, -(self.i * other.k)),
            self.i.mul_add(other.j, -(self.j * other.i)),
        )
    }

    pub fn is_parallel_to(&self, other: Self) -> bool {
        !self.is_zero() && !other.is_zero() && self.cross(other).is_zero()
    }

    pub fn is_perpendicular_to(&self, other: Self) -> bool {
        !self.is_zero() && !other.is_zero() && is_fuzzy_eq(self.dot(other), 0.0)
    }

    pub fn normalize(&self) -> Vector {
        self.multiply_by_scalar(self.get_magnitude().recip())
    }

    pub fn is_normalized(&self) -> bool {
        is_fuzzy_eq(self.get_magnitude(), 1.0)
    }

    pub fn add(&self, other: Self) -> Self {
        Vector::new(self.i + other.i, self.j + other.j, self.k + other.k)
    }
}
