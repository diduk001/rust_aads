/// Segment tree structure generic, that allows to efficiently compute associative function on a
/// segment.
///
/// Segment tree described [here](https://en.wikipedia.org/wiki/Segment_tree) and
/// [here](https://cp-algorithms.com/data_structures/segment_tree.html)
pub struct SegmentTree<T: Eq + Copy> {
    /// `tree` is a vector containing each node's value
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
    /// * `vec` - An initial vector
    /// * `identity` - An identity element, such as `0` for addition or `1` for multiplication
    /// * `combine_fn` - A function, which will be computed on query. Arguments are 2 `T`
    /// instances and return type is `T`
    pub fn new(vec: &Vec<T>, identity: T, combine_fn: fn(T, T) -> T) -> SegmentTree<T> {
        let mut seg_tree: SegmentTree<T> = SegmentTree {
            /// `tree` initially is a vector filled with
            /// `identity` of size `4 * vec.len`
            tree: vec![identity; 4 * vec.len()],
            n: vec.len(),
            identity,
            combine_fn,
        };

        // Building segment tree
        seg_tree.__build(vec, 0, 0, seg_tree.n);

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
    /// * `vec` - initial vector
    /// * `v` - current node index
    /// * `l` - left bound of node in array
    /// * `r` - right bound of node in array (non-inclusive)
    ///
    /// In other words, `tree[v]` contains result of `combine_fn` for `vec[l..r]`
    fn __build(&mut self, vec: &Vec<T>, v: usize, l: usize, r: usize) {
        // Node contains one element case
        if r - l == 1 {
            self.tree[v] = vec[l];
            return;
        }

        // Divide l..r into two halves, build two children of a node and combine two children's
        // result
        let m: usize = (l + r) / 2;
        self.__build(vec, 2 * v + 1, l, m);
        self.__build(vec, 2 * v + 2, m, r);
        self.tree[v] = self.__combine(self.tree[2 * v + 1], self.tree[2 * v + 2]);
    }

    /// Returns `combine_fn` result for `vec[lq..rq]` query
    ///
    /// # Arguments:
    ///
    /// * `v` - current node index
    /// * `l` - left bound of node in array
    /// * `r` - right bound of node in array (non-inclusive)
    /// * `lq` - left bound of query
    /// * `lq` - right bound of query (non-inclusive)
    fn __get(&self, v: usize, l: usize, r: usize, lq: usize, rq: usize) -> T {
        if lq <= l && r <= rq {
            // l..r completely lies in lq..rq
            return self.tree[v];
        } else if r <= lq || rq <= l {
            // l..r does not intersect with lq..rq
            return self.identity;
        }

        // Divide l..r into two halves and get answer from two children of a node
        let m: usize = (l + r) / 2;
        let l_child: T = self.__get(2 * v + 1, l, m, lq, rq);
        let r_child: T = self.__get(2 * v + 2, m, r, lq, rq);
        return self.__combine(l_child, r_child);
    }

    /// Friendly interface of `__get` with query bounds assert
    ///
    /// # Arguments:
    /// * `lq` - left bound of query
    /// * `lq` - right bound of query (non-inclusive)
    pub fn get(&self, lq: usize, rq: usize) -> T {
        // Assert query bounds
        assert!(lq < rq && (lq < self.n) && (0 < rq && rq <= self.n));
        return self.__get(0, 0, self.n, lq, rq);
    }

    /// Sets `i`-th element of segment tree to `new_val` and updates tree
    ///
    /// # Arguments
    /// * `v` - current node index
    /// * `l` - left bound of node in array
    /// * `r` - right bound of node in array (non-inclusive)
    /// * `i` - index of element in initial array
    /// * `new_val` - new value of `i`-th element
    fn __set(&mut self, v: usize, l: usize, r: usize, i: usize, new_val: T) {
        // Node contains one element and its index is `i`
        if i == l && r - l == 1 {
            self.tree[v] = new_val;
            return;
        }

        // Divide l..r into two halves and update current node value from children
        let m: usize = (l + r) / 2;
        if i < m {
            // `i`-th element lies in left child
            self.__set(2 * v + 1, l, m, i, new_val);
        } else if m <= i {
            // `i`-th element lies in right child
            self.__set(2 * v + 2, m, r, i, new_val);
        }
        self.tree[v] = self.__combine(self.tree[2 * v + 1], self.tree[2 * v + 2]);
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