use std::ops::Mul;

/// Raises `element` to `power` using recursive
/// [binary exponentiation](https://en.wikipedia.org/wiki/Exponentiation_by_squaring). Asymptotic
/// is O(logP*K), where P is power and K is asymptotic of multiplication.
///
/// # Arguments
/// * `element` - base of exponentiation
/// * `power` - power, must be at least 1
pub fn binary_exponentiation<T: Copy + Mul<Output = T>>(element: T, power: u64) -> T {
    // `power` must be at least 1
    assert!(power > 0);

    // base recursion vase
    if power == 1 { return element; }
    // little optimization
    if power == 2 { return element * element; }

    // Raise element to power `power / 2`
    let half_power = binary_exponentiation(element, power / 2);
    // square half_power (now its `element` raised to `2 * (power / 2)`
    let half_power_squared = half_power * half_power;
    // return square if power is even and else element * square
    return if power % 2 == 0 { half_power_squared } else { element * half_power_squared }
}