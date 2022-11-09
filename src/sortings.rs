/// Sorts vector using [bubble sort algorithm](https://en.wikipedia.org/wiki/Bubble_sort), time
/// complexity is O(N^2)
///
/// # Arguments:
/// * v - mutable vector, that will be sorted
pub fn bubble_sort<T: Ord + Copy>(v: &mut Vec<T>) {
    let n: usize = v.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if v[j] > v[j + 1] {
                v.swap(j, j + 1);
            }
        }
    }
}

/// Sorts vector using [selection sort algorithm](https://en.wikipedia.org/wiki/Selection_sort),
/// time complexity is O(N^2)
///
/// # Arguments:
/// * v - mutable vector, that will be sorted
pub fn selection_sort<T: Copy + Ord>(v: &mut Vec<T>) {
    let n: usize = v.len();
    for i in 0..n - 1 {
        // Find first_idx (minimum) and place it into beginning of v
        let mut first_idx = i;
        for j in i + 1..n {
            if v[first_idx] > v[j] {
                first_idx = j;
            }
        }

        v.swap(i, first_idx);
    }
}

/// Sorts vector using [insertion sort algorithm](https://en.wikipedia.org/wiki/Insertion_sort),
/// time complexity is O(N^2)
///
/// # Arguments:
/// * v - mutable vector, that will be sorted
pub fn insertion_sort<T: Copy + Ord>(v: &mut Vec<T>) {
    let n: usize = v.len();
    for i in 1..n {
        let mut j = i;
        while j > 0 && v[j - 1] > v[j] {
            v.swap(j, j - 1);
            j -= 1;
        }
    }
}

/// Sorts a vector od `i32`s using
/// [counting sort algorithm](https://en.wikipedia.org/wiki/Counting_sort),
/// time complexity is O(N), where `N` is a length of `[min_element, max_element]`.
///
/// **NOTE:** this is not a generic implementation, it can only be used for `Vec<i32>`. Generic
/// implementation uses [`Step` trait](https://doc.rust-lang.org/std/iter/trait.Step.html), see
/// [issue #42168](https://github.com/rust-lang/rust/issues/42168).
///
/// <details>
///     <summary>Generic implementation</summary>
///     ``` use std::hash::Hash;
/// use std::iter::Step;
///
/// pub fn counting_sort<T: Copy + Ord + Step + Hash>(v: &mut Vec<T>) {
///     let n = v.len();
///     if n == 0 { return; }
///
///     use std::collections::HashMap;
///     let mut counter = HashMap::new();
///
///     let mut min_element = v[0];
///     let mut max_element = v[0];
///
///     for &el in v.iter() {
///         if el < min_element { min_element = el; }
///         if max_element < el { max_element = el; }
///
///         let new_cnt: u64;
///         match counter.get(&el) {
///             Some(old_cnt) => new_cnt = old_cnt + 1,
///             None => new_cnt = 1
///         }
///
///         counter.insert(el, new_cnt);
///     }
///
///     let mut v_idx = 0;
///     for val in min_element..=max_element {
///         if !counter.contains_key(&val) { continue; }
///         for _ in 0..counter[&val] {
///             v[v_idx] = val;
///             v_idx += 1;
///         }
///     }
/// } ```
/// </details>
///
///
/// # Arguments:
/// * v - mutable vector of `i32`s, that will be sorted
pub fn counting_sort(v: &mut Vec<i32>) {
    let n = v.len();
    if n == 0 {
        return;
    }

    use std::collections::HashMap;
    // Count occurrences of elements
    let mut counter = HashMap::new();

    // Minimum and maximum elements for range
    let mut min_element = v[0];
    let mut max_element = v[0];

    for &el in v.iter() {
        // Update minimum and maximum
        if el < min_element {
            min_element = el;
        }
        if max_element < el {
            max_element = el;
        }

        // Update occurrences' count
        let new_cnt: u64;
        match counter.get(&el) {
            Some(old_cnt) => new_cnt = old_cnt + 1,
            None => new_cnt = 1,
        }

        counter.insert(el, new_cnt);
    }

    // v_idx - index of updating element in v
    let mut v_idx = 0;
    // iterate over elements range
    for val in min_element..=max_element {
        // if val is not present in v
        if !counter.contains_key(&val) {
            continue;
        }

        // change v
        for _ in 0..counter[&val] {
            v[v_idx] = val;
            v_idx += 1;
        }
    }
}
