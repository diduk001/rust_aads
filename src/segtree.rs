/// Segment tree structure generic, that allows to efficiently compute associative function on a
/// segment.
///
/// Segment tree described [here](https://en.wikipedia.org/wiki/Segment_tree) and
/// [here](https://cp-algorithms.com/data_structures/segment_tree.html).
///
/// Memory complexity - O(N)
///
/// Time complexity:
/// * `get` - O(logN)
/// * `set` - O(logN)
pub struct SegmentTree<T: Eq + Copy> {
    /// `tree` is a vector containing each node's value. `tree[0]` is a root node
    tree: Vec<T>,
    /// `n` is initial vector's size
    n: usize,
    /// `identity` is an element `i` such `combine(e, i) = e` for every `e`
    identity: T,
    /// `combine_fn` is function of combination such as addition or gcd.
    /// Must be associative
    combine_fn: fn(T, T) -> T,
}

/// Segment tree generic implementation.
///
/// Type `T` must support `Eq` for checking equality with identity.
///
/// Type `T` must support `Copy` for moving elements from vector tree and creating initial tree
/// filled with identities.
impl<T: Eq + Copy> SegmentTree<T> {
    /// Constructs and returns segment tree based on given vector, operation and identity element
    ///
    /// # Arguments:
    ///
    /// * `v` - An initial vector
    /// * `identity` - An identity element, such as `0` for addition or `1` for multiplication
    /// * `combine_fn` - A function, which will be computed on query. Arguments are 2 `T`
    /// instances and return type is `T`
    pub fn new(v: &Vec<T>, identity: T, combine_fn: fn(T, T) -> T) -> SegmentTree<T> {
        let mut seg_tree: SegmentTree<T> = SegmentTree {
            /// `tree` initially is a vector filled with
            /// `identity` of size `4 * vec.len`
            tree: vec![identity; 4 * v.len()],
            n: v.len(),
            identity,
            combine_fn,
        };

        // Building segment tree
        seg_tree.__build(v, 0, 0, seg_tree.n);

        return seg_tree;
    }

    /// Combines two elements with handling `identity` cases
    fn __combine(&self, a: T, b: T) -> T {
        return if a == self.identity {
            b
        } else if b == self.identity {
            a
        } else {
            (self.combine_fn)(a, b)
        };
    }

    /// Builds subtree based on its children
    ///
    /// # Arguments:
    ///
    /// * `v` - initial vector
    /// * `tree_id` - current node index
    /// * `tree_l` - left bound of node in array
    /// * `tree_r` - right bound of node in array (non-inclusive)
    ///
    /// In other words, `tree[tree_id]` contains result of `combine_fn` for `v[tree_l..tree_r]`
    fn __build(&mut self, v: &Vec<T>, tree_id: usize, tree_l: usize, tree_r: usize) {
        // Node contains one element case
        if tree_r - tree_l == 1 {
            self.tree[tree_id] = v[tree_l];
            return;
        }

        // Divide l..r into two halves, build two children of a node and combine two children's
        // result
        let m: usize = (tree_l + tree_r) / 2;
        self.__build(v, 2 * tree_id + 1, tree_l, m);
        self.__build(v, 2 * tree_id + 2, m, tree_r);
        self.tree[tree_id] = self.__combine(self.tree[2 * tree_id + 1], self.tree[2 * tree_id + 2]);
    }

    /// Returns `combine_fn` result for `v[lq..rq]` query
    ///
    /// # Arguments:
    ///
    /// * `tree_id` - current node index
    /// * `tree_l` - left bound of node in array
    /// * `tree_r` - right bound of node in array (non-inclusive)
    /// * `query_l` - left bound of query
    /// * `query_r` - right bound of query (non-inclusive)
    fn __get(&self, tree_id: usize, tree_l: usize, tree_r: usize, query_l: usize, query_r: usize) -> T {
        if query_l <= tree_l && tree_r <= query_r {
            // tree_l..tree_r completely lies in query_l..query_r
            return self.tree[tree_id];
        } else if tree_r <= query_l || query_r <= tree_l {
            // tree_l..tree_r doesn't intersect with query_l..query_r
            return self.identity;
        }

        // Divide tree_l..tree_r into two halves and get answer from two children of a node
        let m: usize = (tree_l + tree_r) / 2;
        let l_child: T = self.__get(2 * tree_id + 1, tree_l, m, query_l, query_r);
        let r_child: T = self.__get(2 * tree_id + 2, m, tree_r, query_l, query_r);
        return self.__combine(l_child, r_child);
    }

    /// Friendly interface of `__get` with query bounds assert
    ///
    /// # Arguments:
    /// * `query_l` - left bound of query
    /// * `query_r` - right bound of query (non-inclusive)
    pub fn get(&self, query_l: usize, query_r: usize) -> T {
        // Assert query bounds
        assert!(query_l < query_r && (query_l < self.n) && (0 < query_r && query_r <= self.n));
        return self.__get(0, 0, self.n, query_l, query_r);
    }

    /// Sets `i`-th element of segment tree to `new_val` and updates tree
    ///
    /// # Arguments
    /// * `tree_id` - current node index
    /// * `tree_l` - left bound of node in array
    /// * `tree_r` - right bound of node in array (non-inclusive)
    /// * `i` - index of element in initial array
    /// * `new_val` - new value of `i`-th element
    fn __set(&mut self, tree_id: usize, tree_l: usize, tree_r: usize, i: usize, new_val: T) {
        // Node contains one element and its index is `i`
        if i == tree_l && tree_r - tree_l == 1 {
            self.tree[tree_id] = new_val;
            return;
        }

        // Divide tree_l..tree_r into two halves and update current node value from children
        let m: usize = (tree_l + tree_r) / 2;
        if i < m {
            // `i`-th element lies in left child
            self.__set(2 * tree_id + 1, tree_l, m, i, new_val);
        } else if m <= i {
            // `i`-th element lies in right child
            self.__set(2 * tree_id + 2, m, tree_r, i, new_val);
        }
        self.tree[tree_id] = self.__combine(self.tree[2 * tree_id + 1], self.tree[2 * tree_id + 2]);
    }

    /// Friendly interface of `__set` with `i` assert
    ///
    /// # Arguments
    /// * `i` - index of element in initial array
    /// * `new_val` - new value of `i`-th element
    pub fn set(&mut self, i: usize, new_val: T) {
        assert!(i <= self.n); // Asserts `i` bounds
        self.__set(0, 0, self.n, i, new_val);
    }
}