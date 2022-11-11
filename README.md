# rust_aads - Rust Algorithms And Data Structures
`rust_aads` is an open repository with algorithms and data structures, used in computer science and 
competitive programming, written in Rust. I'm doing it in my own to practice and if you want more 
algorithms, you should search in [The Algorithms/Rust](https://github.com/TheAlgorithms/Rust) 
repository.

`lib.rs` file contains tests. Algorithms and data structures are separated into 
their own files.

**Please, do PR if you know a better way to do something!**

# Currently implemented Data Structures
* [Segment Tree](https://en.wikipedia.org/wiki/Segment_tree) - `SegmentTree` struct in 
  `segtree.rs` for generic type elements and combination function.

# Currently implemented Algorithms
* [Bubble Sort](https://en.wikipedia.org/wiki/Bubble_sort) - `bubble_sort` function in `sortings. rs`
for generic type elements
* [Selection Sort](https://en.wikipedia.org/wiki/Selection_sort) - `selection_sort` function in 
  `sortings.rs`  for generic type elements
* [Insertion Sort](https://en.wikipedia.org/wiki/Insertion_sort) - `insertion_sort` function in 
  `sortings.rs` for generic type elements
* [Counting Sort](https://en.wikipedia.org/wiki/Counting_sort) - `counting_sort` function in 
  `sortings.rs`. **NOTE:** this is not a generic implementation, it can only be used for 
  `Vec<i32>`. Generic implementation uses
  [`Step` trait](https://doc.rust-lang.org/std/iter/trait.Step.html), see 
  [issue #42168](https://github.com/rust-lang/rust/issues/42168).
* [Binary Exponentiation](https://en.wikipedia.org/wiki/Exponentiation_by_squaring) - 
  `binary_exponentiation` function in `algebra.rs`, implemented for generic type elements that 
  support multiplication.
* [Extended Euclidean GCD Algorithm](https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm) -
`extended_euclidean_algorithm` function in `algebra.rs`. Implemented only for `i32`'s
# TODO:
* Add more tests (with different data types and operations)
* Range operations in Segment tree
* Sorting algorithms:
  * Merge Sort
  * Quick Sort (qsort)
* String Processing
  * Polynomial Hashing
  * Rabin-Karp algorithm
  * Prefix Function
  * Z-Function
  * Trie
  * Manacher's algorithm
* Data Structures
  * Minimum / Maximum Stack & Queue
  * Fenwick Tree
  * Sparse Table
  * Disjoint Set Union
  * Treap (Cartesian tree)
* Algebra and Number Theory
  * Matrices
  * Primality tests (Fermat's theorem)
  * Fibonacci numbers (+ O(logN)) 
  * Factorization
* Graph algorithms
  * BFS, DFS
  * Topological sort
  * Dijkstra's algorithm
  * MST(Prim's algorithm, Kruskal's algorithm)
  * Floyd-Warshall algorithm
* Geometry
  * Vector addition, subtraction, multiplication by scalar
  * Distance between points
  * Dot product, Cross product
