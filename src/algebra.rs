use std::ops::{Mul, Neg, Sub};
use std::ops::{Add, Index, IndexMut};
use std::process::Output;

/// Raises `element` to `power` using recursive
/// [binary exponentiation](https://en.wikipedia.org/wiki/Exponentiation_by_squaring). Complexity
/// is O(logP*K), where P is power and K is a complexity of multiplication.
///
/// # Arguments
/// * `element` - base of exponentiation
/// * `power` - power, must be at least 1
pub fn binary_exponentiation<T: Copy + Mul<Output=T>>(element: T, power: u64) -> T {
    // `power` must be at least 1
    assert!(power > 0);

    // base recursion case
    if power == 1 {
        return element;
    }
    // little optimization
    if power == 2 {
        return element * element;
    }

    // Raise element to power `power / 2`
    let half_power = binary_exponentiation(element, power / 2);
    // square half_power (now its `element` raised to `2 * (power / 2)`
    let half_power_squared = half_power * half_power;
    // return square if power is even and else element * square
    return if power % 2 == 0 {
        half_power_squared
    } else {
        element * half_power_squared
    };
}

/// Calculates [Greatest Common Divisor](https://en.wikipedia.org/wiki/Greatest_common_divisor)
/// of `a` and `b` and numbers `x`, `y` such ``a * x + b * y = g` (where `g` is GCD(a, b)) using
/// [Extended Euclidean Algorithm](https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm)
///
/// # Arguments:
/// * `a: i32` - first natural number,
/// * `b: i32` - second natural number,
/// * `x: &mut i32` - reference to x variable,
/// * `y: $mut i32` - reference to y variable
///
/// Returns GCD and changes `x`, `y`
pub fn extended_euclidean_gcd(a: i32, b: i32, x: &mut i32, y: &mut i32) -> i32 {
    assert!(a > 0 && b > 0);
    // Initialize x and y and x1, y1, a1, b1
    (*x, *y) = (1, 0);
    let (mut x1, mut y1, mut a1, mut b1) = (0, 1, a, b);

    // While we can divide
    while b1 != 0 {
        // Divide and compute x, y, x1, y1
        let q = a1 / b1;
        (*x, x1) = (x1, *x - q * x1);
        (*y, y1) = (y1, *y - q * y1);
        // Update a1 and b1
        (a1, b1) = (b1, a1 - q * b1);
    }
    return a1;
}


/// [Matrix](https://en.wikipedia.org/wiki/Matrix_(mathematics)) structure with generic type
/// elements
///
/// # Fields:
/// * `vals` - values, 2-dimensional vector
/// * `rows` - number of rows, `usize`,
/// * `cols` - number of columns, `usize`
#[derive(Clone)]
pub struct Matrix<T> where T: Clone {
    vals: Vec<Vec<T>>,
    rows: usize,
    cols: usize,
}

/// Creates new Matrix filled with default value of T
impl<T: Clone> Matrix<T>
    where
        T: Default + Clone,
{
    pub fn new(r: usize, c: usize) -> Matrix<T> {
        return Matrix {
            vals: vec![vec![T::default(); c]; r],
            rows: r,
            cols: c,
        };
    }
}

/// Returns row vector by index
impl<T: Clone> Index<usize> for Matrix<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.vals[index];
    }
}

/// Returns mutable row vector by index
impl<T: Clone> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.vals[index];
    }
}

/// Get element, Set element and get Access to element
impl<T: Clone> Matrix<T> {
    pub fn get(&self, r: usize, c: usize) -> Option<&T> {
        match self.vals.get(r) {
            Some(v) => v.get(c),
            None => None,
        }
    }

    pub fn set(&mut self, r: usize, c: usize, new_val: T) {
        self.vals[r][c] = new_val;
    }

    pub fn access(&mut self, r: usize, c: usize) -> &mut T {
        return &mut self.vals[r][c];
    }
}


/// Negate every element of matrix
impl<T: Clone + Copy> Neg for Matrix<T> where T: Neg<Output=T> {
    type Output = Option<Matrix<T>>;

    fn neg(self) -> Self::Output {
        let mut result = self.clone();
        for r in 0..self.rows {
            for c in 0..self.cols {
                match result.get(r, c) {
                    Some(x) => result.set(r, c, -*x),
                    None => return None
                }
            }
        }
        return Some(result);
    }
}


/// Add matrix to another matrix
impl<T: Clone + Copy> Add for Matrix<T> where T: Add<Output=T> {
    type Output = Option<Matrix<T>>;

    fn add(self, rhs: Self) -> Self::Output {
        if (self.rows, self.cols) != (rhs.rows, rhs.cols) {
            return None;
        }
        let mut result = self.clone();
        for r in 0..self.rows {
            for c in 0..self.cols {
                let (lhs_val, rhs_val);
                match result.get(r, c) {
                    Some(x) => lhs_val = x,
                    None => return None
                }
                match rhs.get(r, c) {
                    Some(x) => rhs_val = x,
                    None => return None
                }
                result.set(r, c, *lhs_val + *rhs_val);
            }
        }

        return Some(result);
    }
}


/// Sub one matrix from another (possibly could implement using addition and negation)
impl<T: Clone + Copy> Sub for Matrix<T> where T: Sub<Output=T> {
    type Output = Option<Matrix<T>>;

    fn sub(self, rhs: Self) -> Self::Output {
        if (self.rows, self.cols) != (rhs.rows, rhs.cols) {
            return None;
        }
        let mut result = self.clone();
        for r in 0..self.rows {
            for c in 0..self.cols {
                let (lhs_val, rhs_val);
                match result.get(r, c) {
                    Some(x) => lhs_val = x,
                    None => return None
                }
                match rhs.get(r, c) {
                    Some(x) => rhs_val = x,
                    None => return None
                }
                result.set(r, c, *lhs_val - *rhs_val);
            }
        }

        return Some(result);
    }
}


/// Multiply matrix by scalar
impl<T: Clone + Copy> Mul<T> for Matrix<T> where T: Mul<Output=T> {
    type Output = Matrix<T>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut result = self.clone();
        for r in 0..self.rows {
            for c in 0..self.cols {
                let val;
                match result.get(r, c) {
                    Some(x) => val = x,
                    None => continue
                }
                result.set(r, c, *val * rhs);
            }
        }

        return result;
    }
}


/// Multiply matrix by column
impl<T: Clone + Copy + Default> Mul for Matrix<T> where T: Mul<Output=T> + Add<Output=T> {
    type Output = Option<Matrix<T>>;

    fn mul(self, rhs: Self) -> Self::Output {
        if !(self.cols == rhs.rows) {
            return None;
        }

        let mut result = Matrix::new(self.rows, rhs.cols);
        for r in 0..result.rows {
            for c in 0..result.cols {
                for i in 0..self.cols {
                    let val;
                    match result.get(r, c) {
                        Some(x) => val = x,
                        None => return None
                    }
                    let (lhs_val, rhs_val);
                    match self.get(r, i) {
                        Some(x) => lhs_val = x,
                        None => return None
                    }
                    match self.get(i, c) {
                        Some(x) => rhs_val = x,
                        None => return None
                    }

                    result.set(r, c, *val + *lhs_val * *rhs_val);
                }
            }
        }

        return Some(result);
    }
}