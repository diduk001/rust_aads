use std::ops::Mul;

/// Raises `element` to `power` using recursive
/// [binary exponentiation](https://en.wikipedia.org/wiki/Exponentiation_by_squaring). Complexity
/// is O(logP*K), where P is power and K is a complexity of multiplication.
///
/// # Arguments
/// * `element` - base of exponentiation
/// * `power` - power, must be at least 1
pub fn binary_exponentiation<T: Copy + Mul<Output = T>>(element: T, power: u64) -> T {
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
