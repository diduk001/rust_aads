/// Sorts vector using [bubble sort algorithm](https://en.wikipedia.org/wiki/Bubble_sort), time
/// complexity is O(N^2)
///
/// # Arguments
/// * v - mutable vector, that will be sorted
/// * cmp - function, comparator for sorting
pub fn bubble_sort<T: Copy>(v: &mut Vec<T>, cmp: fn(T, T) -> bool) {
    let n: usize = v.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if !cmp(v[j], v[j + 1]) {
                v.swap(j, j + 1);
            }
        }
    }
}

/// Sorts vector using [selection sort algorithm](https://en.wikipedia.org/wiki/Selection_sort),
/// time complexity is O(N^2)
///
/// # Arguments
/// * v - mutable vector, that will be sorted
/// * cmp - function, comparator for sorting
pub fn selection_sort<T: Copy>(v: &mut Vec<T>, cmp: fn(T, T) -> bool) {
    let n: usize = v.len();
    for i in 0..n - 1 {
        // Find first_idx (minimum) and place it into beginning of v
        let mut first_idx = i;
        for j in i + 1..n {
            if cmp(v[j], v[first_idx]) { first_idx = j; }
        }

        v.swap(i, first_idx);
    }
}