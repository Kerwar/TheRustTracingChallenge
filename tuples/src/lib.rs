use std::ops::{Add, Div, Mul, Neg, Sub};

const EPS: f64 = 1e-5;

/// A structure for tuples, vectors has w = 0 and points w = 1
#[derive(Debug)]
struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < EPS
            && (self.y - other.y).abs() < EPS
            && (self.z - other.z).abs() < EPS
            && (self.w - other.w).abs() < EPS
    }
}

impl Add for Tuple {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Tuple {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Tuple {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl Neg for Tuple {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<f64> for Tuple {
    type Output = Tuple;
    fn mul(self, rhs: f64) -> Self::Output {
        Tuple {
            x: rhs * self.x,
            y: rhs * self.y,
            z: rhs * self.z,
            w: rhs * self.w,
        }
    }
}

impl Mul<Tuple> for f64 {
    type Output = Tuple;
    fn mul(self, rhs: Tuple) -> Self::Output {
        Tuple {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
            w: self * rhs.w,
        }
    }
}

impl Div<f64> for Tuple {
    type Output = Tuple;
    fn div(self, rhs: f64) -> Self::Output {
        Tuple {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Tuple { x, y, z, w }
    }

    pub fn new_point(x: f64, y: f64, z: f64) -> Self {
        Tuple { x, y, z, w: 1.0 }
    }

    pub fn new_vector(x: f64, y: f64, z: f64) -> Self {
        Tuple { x, y, z, w: 0.0 }
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_tuple_with_w_1_is_a_point() {
        let t = Tuple::new(4.3, -4.2, 3.1, 1.0);

        assert_eq!(t.x, 4.3);
        assert_eq!(t.y, -4.2);
        assert_eq!(t.z, 3.1);
        assert_eq!(t.w, 1.0);
        assert!(t.is_point());
        assert!(!t.is_vector());
    }

    #[test]
    fn a_tuple_with_w_0_is_a_point() {
        let t = Tuple::new(4.3, -4.2, 3.1, 0.0);

        assert_eq!(t.x, 4.3);
        assert_eq!(t.y, -4.2);
        assert_eq!(t.z, 3.1);
        assert_eq!(t.w, 0.0);
        assert!(!t.is_point());
        assert!(t.is_vector());
    }

    #[test]
    fn constructor_of_points() {
        let t = Tuple::new(4.0, -4.0, 3.0, 1.0);
        let p = Tuple::new_point(4.0, -4.0, 3.0);

        assert_eq!(t, p);
    }

    #[test]
    fn constructor_of_vectors() {
        let t = Tuple::new(4.0, -4.0, 3.0, 0.0);
        let p = Tuple::new_vector(4.0, -4.0, 3.0);

        assert_eq!(t, p);
    }

    #[test]
    fn adding_of_tupples() {
        let a1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let a2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);

        let addition_result = Tuple::new(1.0, 1.0, 6.0, 1.0);

        assert_eq!(addition_result, a1 + a2);
    }

    #[test]
    fn substracting_two_points_gives_a_vector() {
        let a1 = Tuple::new_point(3.0, 2.0, 1.0);
        let a2 = Tuple::new_point(5.0, 6.0, 7.0);

        let substraction_result = Tuple::new_vector(-2.0, -4.0, -6.0);

        assert_eq!(substraction_result, a1 - a2);
    }

    #[test]
    fn substracting_vector_from_point_gives_a_point() {
        let a1 = Tuple::new_point(3.0, 2.0, 1.0);
        let a2 = Tuple::new_vector(5.0, 6.0, 7.0);

        let substraction_result = Tuple::new_point(-2.0, -4.0, -6.0);

        assert_eq!(substraction_result, a1 - a2);
    }

    #[test]
    fn substracting_a_vector_from_the_zero_vector() {
        let zero = Tuple::new_vector(0.0, 0.0, 0.0);
        let v = Tuple::new_vector(1.0, -2.0, 3.0);

        assert_eq!(zero - v, Tuple::new_vector(-1.0, 2.0, -3.0));
    }

    #[test]
    fn negating_a_tuple() {
        let a = Tuple::new(1.0, -2.0, 3.0, 4.0);
        let a_negated = Tuple::new(-1.0, 2.0, -3.0, -4.0);

        assert_eq!(-a, a_negated);
    }

    #[test]
    fn multiplying_a_tuple_by_a_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let aclone = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let a_mult = Tuple::new(3.5, -7.0, 10.5, -14.0);

        assert_eq!(3.5 * a, a_mult);
        assert_eq!(aclone * 3.5, a_mult);
    }

    #[test]
    fn dividing_a_tuple_by_a_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let a_mult = Tuple::new(0.5, -1.0, 1.5, -2.0);

        assert_eq!(a / 2.0, a_mult);
    }
}
