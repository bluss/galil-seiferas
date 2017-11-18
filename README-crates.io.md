
General string search in constant space, linear time, for nonorderable alphabets.
Also known as exact string matching.

In Rust terms this means we can define the function:

```rust
fn gs_find<T: Eq>(text: &[T], pattern: &[T]) -> Option<usize> {
    // ...
}
```

and the function computes in **O(n)** time and **O(1)** space.
In the worst case, this algorithm makes **4 n** character comparisons.

Note that the Crochemore-Perrin (“Two Way” algorithm) is much superior if
there is a linear order for the alphabet.

This work is Copyright 2017 by Ulrik Sverdrup "bluss"; see license terms
in the package.

# References

Both papers are recommended reading. The comments in this crate’s
implementation are also meant to explain and point out important details,
so that’s recommended reading too.

- [GS] Z. Galil and J. Seiferas,
*Time-Space-Optimal String Matching*,
Journal of Computer and System Sciences (1983)
- [CR] M. Crochemore and W. Rytter,
*Squares, Cubes, and Time-Space Efficient String Searching*,
Algorithmica (1995)
