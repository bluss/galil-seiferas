
// Copyright (c) 2017 Ulrik Sverdrup "bluss"
// 
// See Cargo.toml and LICENSE-* files for more details


//!
//! String search in constant space, linear time, for nonorderable alphabets.
//!
//! In Rust terms this means we can define the function:
//!
//! ```rust
//! fn gs_find<T>(text: &[T], pattern: &[T]) -> Option<usize>
//! where T: Eq
//! {
//!     // ...
//! # drop((text, pattern));
//! # unimplemented!()
//! }
//! ```
//!
//! and the function computes in **O(n)** time and **O(1)** space.
//! In the worst case, this algorithm makes **4 n** character comparisons.
//!
//! Note that the Crochemore-Perrin (“Two Way” algorithm) is much superior if
//! there is a linear order for the alphabet.
//!
//! This work is Copyright 2017 by Ulrik Sverdrup "bluss"; see license terms
//! in the package.
//!
//! # References
//!
//! - [GS] Z. Galil and J. Seiferas,
//! *Time-Space-Optimal String Matching*,
//! Journal of Computer and System Sciences (1983)
//! - [CR] M. Crochemore and W. Rytter,
//! *Squares, Cubes, and Time-Space Efficient String Searching*,
//! Algorithmica (1995)
//!   - Crochemore-Rytter's description of the Galil-Seiferas algoritm has been
//!   very helpful and it explains how it works in concepts that we could
//!   implement.
//!
//! # Implementation Notes
//!
//! We use *k* = 3 like [CR] recommend.


#![cfg_attr(feature = "benchmarks", feature(test))]

#[cfg(test)]
#[macro_use] extern crate matches;
#[macro_use] extern crate defmac;
extern crate unchecked_index;

/// Macro for debug-checked and release-unchecked indexing and slicing.
/// This removes bounds checks in some critial inner loops, where it has
/// a measurable impact.
defmac!(get slice, index => unsafe { ::unchecked_index::get_unchecked(slice, index) });

#[macro_use]
#[doc(hidden)]
pub mod util;
#[cfg(test)]
use util::Bytestring;

/// Test if `text` starts with `pattern`.
fn text_has_prefix<T: Eq>(text: &[T], pattern: &[T]) -> bool {
    get!(text, ..pattern.len()) == pattern
}

#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn test_has_prefix_oob() {
    text_has_prefix(b"abc", b"abcd");
}

#[test]
fn test_has_prefix() {
    let data = b"some text goes here";
    for i in 0..data.len() {
        assert!(text_has_prefix(data, &data[..i]));
    }
}

/// Highly-repeating-prefix (HRP)
///
/// # Background
///
/// A string is *basic* if it is not of the form *a^i* for any word *a* and
/// integer *i*.
///
/// For example, "a", "aba", "abb" are basic and "aa", "abab" are not basic.
///
/// A string z is a *prefix period* of w if it is basic and z^k is a prefix of w
/// (and k is explained below).
///
/// For example, given w = "ababababa", z = "ab" is a prefix period.
/// For example, given w = "aaaaaa", z = "a" is a prefix period and "aa" is not.
///
/// # k-HRP
///
/// k-HRP means a periodic prefix that consists of at least k periods where k is
/// a “reasonably large integer” (GS use k = 4 but [CR] show that k = 3 works
/// and is the smallest.)
///
/// Find the first k-HRP with period >= `period`; return its period and 
/// the length of the prefix (and the length doesn't have to be a multiple of
/// the period).
///
/// Examples:
///
/// x: a a a a b b b
///   |-|-|-|-|
///    HRP(x) period=1, len=4
///
/// x: a b a b a b a b a c
///   |---|---|---|---|-|
///    HRP(x) period=2, len=9
///
/// Throughout we say HRP1(x) for the first HRP of x, HRP2(x) for the second HRP
/// of x, etc. *Note* that a second HRP also starts from the beginning of x,
/// it just has a greater period.
///
fn hrp<T: Eq>(mut period: usize, pattern: &[T]) -> (Option<Hrp>, Option<Hrp>) {
    let k = GS_K;
    let m = pattern.len();
    let mut j = 0;
    let mut hrp1 = None;

    while period + j < m {
        while period + j < m && get!(pattern, j) == get!(pattern, period + j) {
            j += 1;
        }
        let prefix_len = period + j;

        if prefix_len >= period * k {
            let next_hrp = Some(Hrp { period: period, len: prefix_len });
            if let Some(_) = hrp1 {
                return (hrp1, next_hrp);
            } else {
                hrp1 = next_hrp;
            }

            // HRP1 has period `p1`
            //
            // HRP2 if it exists, must have a period p2 > (k - 1) p1

            j -= period;
            period = next_prefix_period(k, period);
            trace!("j -= {} (={})", j, j);
            trace!("period = {} × p + 1 (={})", k, period);
            continue;
        }
        if let Some(ref hrp1) = hrp1 {
            let scope_l = hrp1.period * 2;
            let scope_r = hrp1.len;
            if j >= scope_l && j <= scope_r {
                period += scope_l / 2;
                j -= scope_l / 2;
                trace!("period += {} (={})", scope_l / 2, period);
                trace!("j -= {} (={})", scope_l / 2, j);
                continue;
            }
        }
        period += j / k + 1;
        trace!("period += {} (={})", j / k + 1, period);
        j = 0;
        trace!("j = {}", j);
    }
    (hrp1, None)
}

#[test]
fn test_hrp() {
    let s = b"aabaabaabaabaabaabbbb";

    println!("s: {}", Bytestring(s));
    assert_matches!(hrp(1, s), (Some(Hrp { period: 3, len: 18 }), None));
    assert_matches!(hrp(2, s), (Some(Hrp { period: 3, len: 18 }), None));
    assert_matches!(hrp(4, s), (Some(Hrp { period: 6, len: 18 }), None));
    assert_matches!(hrp(6, s), (Some(Hrp { period: 6, len: 18 }), None));
}

#[test]
fn test_hrp_length() {
    // test a string short one char
    let mut s = "aab".repeat(GS_K);
    assert_matches!(hrp(1, s.as_bytes()), (Some(Hrp { period: 3, .. }), None));
    s.pop();
    assert_matches!(hrp(1, s.as_bytes()), (None, None));
}

#[derive(Copy, Clone, Debug, PartialEq)]
/// Highly-repeating-prefix
struct Hrp {
    period: usize,
    len: usize,
}

impl Hrp {
    /// The special position for HRP1(x) with period v
    /// is the length of v1 ... vi where i is the maximal integer
    /// where |v1 ... vi| < |HRP2(x)|
    ///
    /// Taking |HRP2(x)| to mean the period of HRP2.
    ///
    /// cf. length of prefix period in [GS]
    fn special_position(&self, hrp2: &Self) -> usize {
        let max = hrp2.period - 1;
        debug_assert!(max >= 1);
        // avoid division in the most common cases
        max - (match self.period {
            1 => 0,
            2 => max % 2,
            3 => max % 3,
            other => max % other,
        })
    }
}


/// Return a lower bound for the next prefix period's size.
///
/// [GS] corollary says that for distinct prefix periods, p2 > (k - 1) p1.
#[inline]
fn next_prefix_period(k: usize, period: usize) -> usize {
    (k - 1) * period + 1
}

/// Decompose `pattern` into two words u, v where u is "short" and v is k-simple.
///
/// When *k* >= 3, words satisfy a remarkable combinatorial property:
/// 
/// > each pattern p can be decomposed into u v where u is "short" and v is
/// >  a k-simple word.
///
/// + k-simple: v has at most one k-HRP; if it exists, it is returned as well.
/// + "short": |u| <= 2 per(v)
///
/// Definitions from [CR] section 3. Cube Prefixes.
///
//
// When k >= 3, words satisfy a remarkable combinatorial property:
// 
//  each pattern p can be decomposed into uv where u is "short" and v is
//  a k-simple word.
// 
// The composition p = uv is k-perfect iff v is k-simple. and |u| < 2per(v)
//
// lemma 7a. HRP2(x) is at least twice as long as HRP1(x)
//
// x:  [ - - - - - - - - - - - - - - - - ]
//       HRP1(x)
// x:  [ v1 | v1 | - - - - - - - - - - - ]
//
// x':           [ - - - - - - - - - - - ]
//                HRP1(x')
// x':           [  v2  |  v2  | - - - - ]
//
// Size is nondecreasing: |HRP1(x)| <= |HRP1(x')|
//
// The elements V(x) = (v1, v2, ..) are the working factors.
// v1 is HRP1(x); let x' = v1 x, then v2 is HRP1(x') until there are no
// more HRP1.
//
// Define i: greatest integer where |v1 ... v_i| < |HRP2(x)|
// if v_i+1 exists, |v_i+1| >= |HRP2(x)|
//
// Define: first special position is the length of |v1 .. v_i|
// Define: second special position. let x = v1 ... v_i x', then
//         it is the first special position of x'.
//     and so on with further special positions.
//
// If HRP2 does not exist, or HRP1 does not exist, 0 is the only special
// position.
//
// Theorem 5 (Decomposition) Let j be the last special position of x
// Let u = x[1 .. j] and v = x[j + 1 .. n]. Then the decomposition uv of x
// is k-perfect for k >= 3.
//
fn decompose<T: Eq>(pattern: &[T]) -> (&[T], &[T], Option<Hrp>) {
    let mut j = 0;
    let (mut hrp1_opt, mut hrp2_opt) = hrp(1, pattern);
    loop {
        if let Some(hrp1) = hrp1_opt {
            if let Some(hrp2) = hrp2_opt {
                // x' = x[j..]
                j += hrp1.special_position(&hrp2);

                // compute HRP1(x') and HRP2(x')
                // size is nondecreasing: so use the HRP1(x) period.
                let (h1, h2) = hrp(hrp1.period, get!(pattern, j..));
                hrp1_opt = h1;
                hrp2_opt = h2;
                continue;
            }
        }
        break;
    }
    let (a, b) = (get!(pattern, ..j), get!(pattern, j..));
    #[cfg(debug_assertions)]
    assert_perfect_decomp(GS_K, (a, b));
    (a, b, hrp1_opt)
}

#[test]
fn test_decompose() {
    let s = b"banana";
    assert_matches!(decompose(s), (_, _, None));
    let s = b"aaabaaabaaabaabbbb";
    assert_matches!(decompose(s), (_, _, None));
    let s = b"abababababababababababcabcabcabcabc";
    assert_matches!(decompose(s), (_, _, Some(Hrp { period: 2, len: _ })));
    let s = b"ananananananananan in the face";
    assert_matches!(decompose(s), (_, _, Some(Hrp { period: 2, .. })));
}

#[test]
fn test_decompose_2() {
    let period = "aaaaacargo";
    let pattern = period.repeat(50 / period.len());
    let pattern = pattern.as_bytes();
    let (u, v, hrp) = decompose(pattern);
    assert_eq!(u, &pattern[..u.len()]);
    assert_eq!(v, &pattern[u.len()..]);
    assert_matches!(hrp, Some(Hrp { period: 10, len: _ }));
}

/// Assert that the input = u v is a perfect factorization
#[cfg(debug_assertions)]
fn assert_perfect_decomp<T: Eq>(k: usize, input: (&[T], &[T])) {
    // require that a decomp x = u v
    // that u is "short" and v is k-simple.
    // k-simple means it has at most one k-HRP which also means it has no k-HRP2
    assert!(k >= 3);
    let (u, v) = input;
    if let (Some(hrp1), hrp2) = hrp(1, v) {
        if let Some(hrp2) = hrp2 {
            panic!("Factorization u, v = {} , {} is not k-simple because
                    v's {}-HRP1 is {:?} and {}-HRP2 is {:?}",
                    u.len(), v.len(), k, hrp1, k, hrp2);
            
        }
    }
    // ok
}


/// The value k is a “large enough integer”; [CS] shows k = 3 is the best,
/// lowest value where the algorithm works (unmodified).
const GS_K: usize = 3;

/// This is the Galil-Seiferas string matching algorithm.
///
/// If a match exists where `pattern` is a substring of `text`, return the
/// offset to the start of the match inside `Some(_)`. If not, return `None`.
pub fn gs_find<T: Eq>(text: &[T], pattern: &[T]) -> Option<usize> {
    // trivial cases; the empty pattern is a match
    if pattern.len() > text.len() {
        return None;
    } else if pattern.is_empty() {
        return Some(0);
    }

    // preprocess the pattern into u, v
    let (u, v, hrp1) = decompose(pattern);

    // find each occurence of v in the text; then check if u precedes it
    let mut pos = 0;
    while let Some(i) = search_simple(get!(text, u.len()..), v, &mut pos, &hrp1) {
        if text_has_prefix(get!(text, i..), u) {
            return Some(i);
        }
    }
    None
}

#[test]
fn test_gs_search() {
    defmac!(test_str text, pat => assert_eq!(text.find(pat), gs_find(text.as_bytes(), pat.as_bytes())));
    test_str!("abc", "");
    test_str!("abc", "a");
    test_str!("abc", "z");
    test_str!("abbaababx", "abab");
    test_str!("bbbaaaaaaaaaaaaaaaaaaa", "aaaaaa");
    test_str!("bbbaaaaaaaaaaaaaaaaaaaanananananananananan", "anananananananananan");
    test_str!("nananananananananananabcabc", "anananananananananan");
    test_str!("anananananananananananabcabc", "anananananananananan");
    test_str!("aa\u{0}\u{0}a", "aaaa");
    test_str!("bbbbabaa", "bbbbbbaa");
    test_str!("ababaaabbbabbbbbbbabaabababbbaaaaaaaaaabbbbabaa", "bbbbbba");
    test_str!("abbbbbaabab", "bbbbbbab");
    test_str!("abbbbbaabaaaab", "bbbbbbab");
    test_str!("aaaaaabaaab", "aaaaaabaab");
    test_str!("", "");
    test_str!("", "aaaaaa");
}

/// Search `text` for the `pattern` with the requirement that the pattern
/// is k-simple; which means it has at most one k-HRP.
///
/// `start_pos` is the position to start the search, and it is updated after
/// the function returns with a match.
fn search_simple<T: Eq>(text: &[T], pattern: &[T],
                        start_pos: &mut usize, hrp1: &Option<Hrp>)
    -> Option<usize>
{
    // 
    #[cfg(debug_assertions)]
    assert_perfect_decomp(GS_K, (&[], pattern));
    debug_assert!(pattern.len() <= text.len());
    debug_assert_eq!(hrp(1, pattern).0, *hrp1);
    let n = text.len();
    let m = pattern.len();

    let (has_scope, scope_l, scope_r) = if let Some(hrp1) = *hrp1 {
        // Scope of the k-HRP1 is [L, R]
        // where
        //  L = |v²| = 2 × period
        //  R = z = length of prefix
        //
        // See Lemma 2 in [CR]:
        //
        // Any nonempty prefix u of x satisfies
        //
        // per(u) = Li / 2 if |u| is in [Li, Ri] for some i
        // per(u) > |u| / k if not
        //
        let scope_l = hrp1.period * 2;
        let scope_r = hrp1.len;
        debug_assert!(scope_l <= scope_r);
        (true, scope_l, scope_r)
    } else {
        (false, 0, 0)
    };
    trace!("scope_l={}, scope_r={}", scope_l, scope_r);

    let mut pos = *start_pos;
    let mut j = 0;
    while pos <= n - m {
        while j < m && get!(pattern, j) == get!(text, pos + j) {
            j += 1;
            trace!("j += {} (={})", 1, j);
        }
        trace!("pos={}, j={}, m={}", pos, j, m);
        let has_match = if j == m { Some(pos) } else { None };
        if has_scope && j >= scope_l && j <= scope_r {
            pos += scope_l / 2;
            j -= scope_l / 2;
            trace!("pos += {} (={})", scope_l / 2, pos);
            trace!("j -= {} (={})", scope_l / 2, j);
        } else {
            pos += j / GS_K + 1;
            trace!("pos += {} (={})", j / GS_K + 1, pos);
            j = 0;
            trace!("j = 0");
        }
        if let Some(match_pos) = has_match {
            *start_pos = pos;
            return Some(match_pos);
        }
    }
    None
}

#[test]
fn test_periodic() {
    let n = 10;
    let haystack = ("ab".repeat(n - 1) + "bb").repeat(n);
    let pattern = "ab".repeat(n);
    let res = gs_find(haystack.as_bytes(), pattern.as_bytes());
    println!("{:?}", res);
}

#[cfg(all(test, feature = "benchmarks"))]
mod benches {
    extern crate test;
    use self::test::Bencher;
    use super::gs_find;
    use super::decompose;
    use super::util::brute_force_search;
    use super::util::brute_force_fast;

    const DECOMPOSE_LEN: usize = 50;

    #[bench]
    fn bench_g_decompose_ab(b: &mut Bencher) {
        let period = "ab";
        let pattern = period.repeat(DECOMPOSE_LEN / period.len());

        b.iter(|| {
            decompose(pattern.as_bytes());
        });
        b.bytes = pattern.len() as u64;
    }

    #[bench]
    fn bench_g_decompose_aaacargo(b: &mut Bencher) {
        let period = "aaaaacargo";
        let pattern = period.repeat(DECOMPOSE_LEN / period.len());

        b.iter(|| {
            decompose(pattern.as_bytes());
        });
        b.bytes = pattern.len() as u64;
    }

    defmac!(haystack n => ("ab".repeat(n - 1) + "bb").repeat(n));
    defmac!(haystack_inv n => (String::from("bb") + &"ab".repeat(n - 1)).repeat(n));

    #[bench]
    fn bench_gs_periodic2_10(b: &mut Bencher) {
        let n = 10;
        let pattern = "ab".repeat(n);
        let haystack = haystack!(n);

        b.iter(|| {
            gs_find(haystack.as_bytes(), pattern.as_bytes())
        });
        b.bytes = haystack.len() as u64;
    }

    #[bench]
    fn bench_gs_periodic2_50(b: &mut Bencher) {
        let n = 50;
        let haystack = haystack!(n);
        let pattern = "ab".repeat(n);


        b.iter(|| {
            gs_find(haystack.as_bytes(), pattern.as_bytes())
        });
        b.bytes = haystack.len() as u64;
    }

    #[bench]
    fn bench_brute_simple_periodic2_10(b: &mut Bencher) {
        let n = 10;
        let haystack = haystack!(n);
        let pattern = "ab".repeat(n);


        b.iter(|| {
            brute_force_search(haystack.as_bytes(), pattern.as_bytes())
        });
        b.bytes = haystack.len() as u64;
    }

    #[bench]
    fn bench_brute_simple_periodic2_50(b: &mut Bencher) {
        let n = 50;
        let haystack = haystack!(n);
        let pattern = "ab".repeat(n);


        b.iter(|| {
            brute_force_search(haystack.as_bytes(), pattern.as_bytes())
        });
        b.bytes = haystack.len() as u64;
    }

    #[bench]
    fn bench_brute_fast_periodic2_10(b: &mut Bencher) {
        let n = 10;
        let haystack = haystack!(n);
        let pattern = "ab".repeat(n);


        b.iter(|| {
            brute_force_fast(haystack.as_bytes(), pattern.as_bytes())
        });
        b.bytes = haystack.len() as u64;
    }

    #[bench]
    fn bench_brute_fast_periodic2_50(b: &mut Bencher) {
        let n = 50;
        let haystack = haystack!(n);
        let pattern = "ab".repeat(n);


        b.iter(|| {
            brute_force_fast(haystack.as_bytes(), pattern.as_bytes())
        });
        b.bytes = haystack.len() as u64;
    }

    const PER_LARGE: usize = 100;

    #[bench]
    fn bench_gs_periodic2_large(b: &mut Bencher) {
        let n = PER_LARGE;
        let haystack = haystack!(n);
        let pattern = "ab".repeat(n);


        b.iter(|| {
            gs_find(haystack.as_bytes(), pattern.as_bytes())
        });
        b.bytes = haystack.len() as u64;
    }

    #[bench]
    fn bench_brute_simple_periodic2_large(b: &mut Bencher) {
        let n = PER_LARGE;
        let haystack = haystack!(n);
        let pattern = "ab".repeat(n);


        b.iter(|| {
            brute_force_search(haystack.as_bytes(), pattern.as_bytes())
        });
        b.bytes = haystack.len() as u64;
    }

    #[bench]
    fn bench_brute_fast_periodic2_large(b: &mut Bencher) {
        let n = PER_LARGE;
        let haystack = haystack!(n);
        let pattern = "ab".repeat(n);


        b.iter(|| {
            brute_force_fast(haystack.as_bytes(), pattern.as_bytes())
        });
        b.bytes = haystack.len() as u64;
    }

    #[bench]
    fn bench_twoway_periodic2_large(b: &mut Bencher) {
        let n = PER_LARGE;
        let haystack = haystack_inv!(n);
        let pattern = "ab".repeat(n);

        b.iter(|| {
            haystack.find(&pattern)
        });
        b.bytes = haystack.len() as u64;
    }

    #[bench]
    fn bench_gs_periodic5_50(b: &mut Bencher) {
        defmac!(haystack5 n => ("bacba".repeat(n - 1) + "bbbbb").repeat(n));
        defmac!(needle5 n => "bacba".repeat(n));
        let n = 50;
        let haystack = haystack5!(n);
        let pattern = needle5!(n);

        b.iter(|| {
            gs_find(haystack.as_bytes(), pattern.as_bytes())
        });
        b.bytes = haystack.len() as u64;
    }

}
