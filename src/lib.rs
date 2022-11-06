#![crate_name = "rust_aads"]

mod segtree;

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
        vec: &Vec<T>,
        func: fn(T, T) -> T,
        l_idx: usize,
        r_idx: usize,
    ) -> T {
        let mut result: T = vec[l_idx];
        for i in l_idx + 1..r_idx {
            result = func(result, vec[i]);
        }
        return result;
    }

    #[test]
    /// Check sum of all pairs of `[l..r]` indices in `[1..=10]` array
    fn basic_get_add_test() {
        let vec: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let n = vec.len();
        let segtree: SegmentTree<i32> = SegmentTree::new(&vec, 0, add_i32s);

        for l_idx in 0..n {
            for r_idx in l_idx + 1..=n {
                let sum_correct: i32 = compute_function_on_segment(&vec, add_i32s, l_idx, r_idx);
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
        let vec: Vec<i32> = (0..n).map(|_| rng.gen_range(-1000..1000)).collect();
        let segtree: SegmentTree<i32> = SegmentTree::new(&vec, 0, add_i32s);

        for l_idx in 0..n {
            for r_idx in l_idx + 1..=n {
                let sum_correct: i32 = compute_function_on_segment(&vec, add_i32s, l_idx, r_idx);
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
        let mut vec: Vec<i32> = (0..n).map(|_| rng.gen_range(-10..10)).collect();
        let mut segtree: SegmentTree<i32> = SegmentTree::new(&vec, 0, add_i32s);

        for idx_to_change in 0..n {
            let new_val = rng.gen_range(-10..10);
            vec[idx_to_change] = new_val;
            segtree.set(idx_to_change, new_val);

            for l_idx in 0..n {
                for r_idx in l_idx + 1..=n {
                    let sum_correct: i32 =
                        compute_function_on_segment(&vec, add_i32s, l_idx, r_idx);
                    let sum_computed_by_segtree: i32 = segtree.get(l_idx, r_idx);
                    assert_eq!(sum_computed_by_segtree, sum_correct);
                }
            }
        }
    }
}
