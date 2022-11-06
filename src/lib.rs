#![crate_name = "rust_aads"]

mod segtree;
mod sortings;

#[cfg(test)]
mod segment_tree_tests {
    use super::*;
    use rand::Rng;
    use segtree::SegmentTree;

    /// Combination function for adding i32s
    fn add_i32s(a: i32, b: i32) -> i32 {
        return a + b;
    }

    /// Naive iterative implementation of computing function on segment
    fn compute_function_on_segment<T: Copy>(
        v: &Vec<T>,
        func: fn(T, T) -> T,
        l_idx: usize,
        r_idx: usize,
    ) -> T {
        let mut result: T = v[l_idx];
        for i in l_idx + 1..r_idx {
            result = func(result, v[i]);
        }
        return result;
    }

    #[test]
    /// Check sum of all pairs of `[l..r]` indices in `[1..=10]` array
    fn basic_get_add_test() {
        let v: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let n = v.len();
        let segtree: SegmentTree<i32> = SegmentTree::new(&v, 0, add_i32s);

        for l_idx in 0..n {
            for r_idx in l_idx + 1..=n {
                let sum_correct: i32 = compute_function_on_segment(&v, add_i32s, l_idx, r_idx);
                let sum_computed_by_segtree: i32 = segtree.get(l_idx, r_idx);
                assert_eq!(sum_computed_by_segtree, sum_correct);
            }
        }
    }

    #[test]
    /// Check sum of all pairs of `[l..r]` indices in random 1000-element array
    fn random_1000_i32s_get_add_test() {
        let mut rng = rand::thread_rng();

        let n: usize = 1000;
        let v: Vec<i32> = (0..n).map(|_| rng.gen_range(-1000..1000)).collect();
        let segtree: SegmentTree<i32> = SegmentTree::new(&v, 0, add_i32s);

        for l_idx in 0..n {
            for r_idx in l_idx + 1..=n {
                let sum_correct: i32 = compute_function_on_segment(&v, add_i32s, l_idx, r_idx);
                let sum_computed_by_segtree: i32 = segtree.get(l_idx, r_idx);
                assert_eq!(sum_computed_by_segtree, sum_correct);
            }
        }
    }

    #[test]
    /// Change every element in 100-element random array and check sum of all pairs of `[l..r]` indices
    fn random_100_i32s_get_set_add_test() {
        let mut rng = rand::thread_rng();
        let n: usize = 100;
        let mut v: Vec<i32> = (0..n).map(|_| rng.gen_range(-10..10)).collect();
        let mut segtree: SegmentTree<i32> = SegmentTree::new(&v, 0, add_i32s);

        for idx_to_change in 0..n {
            let new_val = rng.gen_range(-10..10);
            v[idx_to_change] = new_val;
            segtree.set(idx_to_change, new_val);

            for l_idx in 0..n {
                for r_idx in l_idx + 1..=n {
                    let sum_correct: i32 =
                        compute_function_on_segment(&v, add_i32s, l_idx, r_idx);
                    let sum_computed_by_segtree: i32 = segtree.get(l_idx, r_idx);
                    assert_eq!(sum_computed_by_segtree, sum_correct);
                }
            }
        }
    }
}


#[cfg(test)]
mod sorting_tests {
    use super::*;
    use rand::Rng;
    use sortings::bubble_sort;

    /// Checks if v is sorted using comparator
    fn is_sorted<T: Copy>(v: &Vec<T>, cmp: fn(T, T) -> bool) -> bool {
        let n = v.len();
        for i in 1..n {
            if !cmp(v[i - 1], v[i]) {
                return false;
            }
        }
        return true;
    }

    /// Comparator for i32s
    fn less_equal(a: i32, b: i32) -> bool { return a <= b; }

    #[test]
    /// Basic 10 elements
    fn basic_10_elements_test() {
        let mut v: Vec<i32> = vec![6, 4, 7, 2, 3, 9, 1, 8, 10, 5];
        let correct: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        bubble_sort(&mut v, less_equal);
        assert_eq!(correct, v);
    }

    #[test]
    fn random_1000_i32s_test() {
        let mut rng = rand::thread_rng();
        let n: usize = 1000;
        let mut v: Vec<i32> = (0..n).map(|_| rng.gen_range(-1000..1000)).collect();

        bubble_sort(&mut v, less_equal);
        assert!(is_sorted(&v, less_equal));
    }
}