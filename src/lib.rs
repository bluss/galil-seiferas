
// Copyright (c) 2017 Ulrik Sverdrup "bluss"
// 
// See Cargo.toml and LICENSE-* files for more details


//!
//! String search in constant space, linear time, for nonorderable alphabets.
//!
//! In Rust terms this means we can define the function:
//!
//! ```rust
//! fn gs_find<T: Eq>(text: &[T], pattern: &[T]) -> Option<usize> {
//!     // ...
//! # None
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
//! Both papers are recommended reading. The comments in this crate’s
//! implementation are also meant to explain and point out important details,
//! so that’s recommended reading too.
//!
//! - [GS] Z. Galil and J. Seiferas,
//! *Time-Space-Optimal String Matching*,
//! Journal of Computer and System Sciences (1983)
//! - [CR] M. Crochemore and W. Rytter,
//! *Squares, Cubes, and Time-Space Efficient String Searching*,
//! Algorithmica (1995)


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
    for i in 0..data.len() + 1 {
        assert!(text_has_prefix(data, &data[..i]));
    }
}

/// The value *k* is a “large enough integer” whose usage becomes clear below;
/// [CS] shows k = 3 is the best, lowest value where the algorithm works
/// unmodified.
const GS_K: usize = 3;

/// Highly-repeating-prefix (HRP)
///
/// # Background
///
/// A string is *basic* if it is not of the form a^i for any word a and
/// integer i.
///
/// For example, "a", "aba", "abb" are basic and "aa", "abab" are not basic.
///
/// A string z is a *prefix period* of w if it is basic and z^i is a prefix of w
/// for some integer i.
///
/// For example, given w = "ababababa", z = "ab" is a prefix period.
/// For example, given w = "aaaaaa", z = "a" is a prefix period and "aa" is not.
///
/// # k-HRP
///
/// k-HRP means a periodic prefix that consists of at least k periods.
/// (The length doesn't have to be a multiple of the period.)
///
/// This function finds the two first k-HRP with period >= `period`; return each
/// as an optional Hrp value.
///
/// Examples:
///
/// x: a a a a b b b
///   |.|.|.|.|
///    HRP(x) period=1, len=4
///
/// x: a b a b a b a b a c
///   |...|...|...|...|.
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
        // find the greatest length (period + j) with the same period
        while period + j < m && get!(pattern, j) == get!(pattern, period + j) {
            j += 1;
        }

        let prefix_length = period + j;

        if prefix_length >= period * k {
            // we found the next HRP
            let next_hrp = Some(Hrp { period: period, len: prefix_length });
            match hrp1 {
                Some(_) => return (hrp1, next_hrp),
                None => hrp1 = next_hrp,
            }

            // period, j adjustments from [GS] An Integrated Implementation.
            //
            // pattern[..period + j] has period of length `period`
            // pattern[..period + j + 1] does not.
            // thus second period >= j.
            //
            // For example:
            //
            //   /.\/.\/.\/.  period = 3
            //   abcabcabcabdefghij  // the pattern
            //   \........./  prefix with period + j = 11
            //   next period must be >= to accomodate the mismatching char (here "d")
            //
            period = j;
            j = 0;
        } else {
            match hrp1 {
                Some(ref hrp1) if j >= hrp1.period * 2 && j <= hrp1.len => {
                    period += hrp1.period;
                    j -= hrp1.period;
                }
                _ => {
                    period += j / k + 1;
                    j = 0;
                }
            }
        }
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

#[cfg(any(test, debug_assertions))]
fn find_k_hrp<T: Eq>(period: usize, x: &[T]) -> Option<usize> {
    let mut pos = 0;
    let mut period = period;
    while pos < x.len() && period < x.len() {
        while pos + period < x.len() && x[pos] == x[pos + period] {
            pos += 1;
        }
        if pos + period >= GS_K * period {
            return Some(period);
        }
        pos = 0;
        period += 1;
    }
    None
}

#[test]
fn test_find_period() {
    assert_matches!(find_k_hrp(1, b"aab"), None);
    assert_matches!(find_k_hrp(1, b"aaab"), Some(1));
    assert_matches!(find_k_hrp(2, b"abababac"), Some(2));
    assert_matches!(find_k_hrp(1, b""), None);
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
///
/// lemma 7a. HRP2(x) is at least twice as long as HRP1(x)
///
/// x : [ - - - - - - - - - - - - - - - - ]
///       HRP1(x)
/// x : [ v1 | v1 | - - - - - - - - - - - ]
///
/// x':           [ - - - - - - - - - - - ]
///                HRP1(x')
/// x':           [  v2  |  v2  | - - - - ]
///
/// Size is nondecreasing: |HRP1(x)| <= |HRP1(x')|
///
/// The elements V(x) = (v1, v2, ..) are the working factors.
/// v1 is HRP1(x); let x' = v1 x, then v2 is HRP1(x') until there are no
/// more HRP1.
///
/// Define i: greatest integer where |v1 ... v_i| < |HRP2(x)|
/// if v_i+1 exists, |v_i+1| >= |HRP2(x)|
///
/// Define: first special position is the length of |v1 .. v_i|
/// Define: second special position. let x = v1 ... v_i x', then
///         it is the first special position of x'.
///     and so on with further special positions.
///
/// If HRP2 does not exist, or HRP1 does not exist, 0 is the only special
/// position.
///
/// Theorem 5 (Decomposition) Let j be the last special position of x
/// Let u = x[1 .. j] and v = x[j + 1 .. n]. Then the decomposition uv of x
/// is k-perfect for k >= 3.
///
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
    assert_perfect_decomposition(GS_K, a, b);
    (a, b, hrp1_opt)
}

#[test]
fn test_decompose() {
    let s = b"banana";
    assert_matches!(decompose(s), (_, _, None));

    // aaa as HRP1 (per=1) and aaabaaabaaab.. as HRP2 (per=4)
    let s = "aaab".repeat(4) + "bbbb";
    let (u, v, hrp) = decompose(s.as_bytes());
    println!("u,v = {},{} hrp1={:?}", Bytestring(u), Bytestring(v), hrp);
    assert_eq!(u.len(), 3);
    assert_matches!(hrp, Some(Hrp { period: 4, len: 13 }));

    let s = String::from("aaa") + &"ab".repeat(4);
    let (u, v, hrp) = decompose(s.as_bytes());
    println!("u,v = {},{} hrp1={:?}", Bytestring(u), Bytestring(v), hrp);
    assert_eq!(u.len(), 0);
    assert_matches!(hrp, Some(Hrp { period: 1, .. }));

    let s = b"aaabaaabaaabaabbbb";
    let (u, v, hrp) = decompose(s);
    println!("u,v = {},{} hrp1={:?}", Bytestring(u), Bytestring(v), hrp);
    assert_eq!(u.len(), 3);
    assert_matches!(hrp, None);

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

#[test]
fn test_decompose_period_1() {
    let period = "a";
    let pattern = period.repeat(50 / period.len());
    let pattern = pattern.as_bytes();
    let (u, v, hrp) = decompose(pattern);
    assert_matches!(::hrp(1, pattern), (Some(_), None));
    println!("u, v = {}, {}", Bytestring(u), Bytestring(v));
    assert_eq!(u, &pattern[..u.len()]);
    assert_eq!(v, &pattern[u.len()..]);
    assert_eq!(u.len(), 0);
    assert_matches!(hrp, Some(Hrp { period: 1, len: 50 }));
}

#[test]
fn test_decompose_period_2() {
    let n = 10;
    let pattern = "ab".repeat(n);
    let (u, v, hrp) = decompose(pattern.as_bytes());
    println!("u, v = {}, {}", Bytestring(u), Bytestring(v));
    println!("hrp = {:?}", hrp);
    assert_eq!(u.len(), 0);
    assert_matches!(hrp, Some(Hrp { period: 2, .. }));
}


/// Assert that the input = u v is a perfect factorization
#[cfg(debug_assertions)]
fn assert_perfect_decomposition<T: Eq>(k: usize, u: &[T], v: &[T]) {
    // require that a decomp x = u v
    // that u is "short" and v is k-simple.
    // k-simple means it has at most one k-HRP which also means it has no k-HRP2
    assert!(k >= 3);
    if let (Some(hrp1), hrp2) = hrp(1, v) {
        if let Some(hrp2) = hrp2 {
            panic!("Factorization u, v = {} , {} is not k-simple because
                    v's {}-HRP1 is {:?} and {}-HRP2 is {:?}",
                    u.len(), v.len(), k, hrp1, k, hrp2);
            
        }
    }
    // independent check
    if let Some(prefix_period1) = find_k_hrp(1, v) {
        // ok, but must not have a second one, or if it has it's a multiple
        if let Some(prefix_period2) = find_k_hrp(prefix_period1 * 2 + 1, v) {
            assert_eq!(prefix_period2 % prefix_period1, 0);
        }
    }
    // ok
}


/// This is the Galil-Seiferas string matching algorithm.
///
/// If a match exists where `pattern` is a substring of `text`, return the
/// offset to the start of the match inside `Some(_)`. If not, return `None`.
pub fn gs_find<T: Eq>(text: &[T], pattern: &[T]) -> Option<usize> {
    if pattern.len() > text.len() {
        return None;
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

#[cfg(test)]
defmac!(test_str text, pat => assert_eq!(text.find(pat), gs_find(text.as_bytes(), pat.as_bytes())));

#[test]
fn test_gs_find_vs_str_find() {
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

#[test]
fn test_gs_find2() {
    // found by cargo fuzz; proves need of scope_l/scope_r check in hrp
    let s = "\x0abaaabaaabaaabaaabaaabbbbb";
    let n = "aaabaaabaaabaaabbbbb";
    test_str!(s, n);
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

    let mut pos = *start_pos;
    let mut j = 0;
    while pos <= n - m {
        while j < m && get!(pattern, j) == get!(text, pos + j) {
            j += 1;
        }
        let has_match = if j == m { Some(pos) } else { None };
        if has_scope && j >= scope_l && j <= scope_r {
            pos += scope_l / 2;
            j -= scope_l / 2;
        } else {
            pos += j / GS_K + 1;
            j = 0;
        }
        if let Some(match_pos) = has_match {
            *start_pos = pos;
            return Some(match_pos);
        }
    }
    None
}

//
// Remark on KMP, using borders. Finding a period of 4 means there is a border
// of length - 4 (where a suffix matches a prefix.) This is used in the KMP
// shift table.
//
// // abcdab
// // xx..xx  border of 2 <=> period of length - 2
//    |--|--  per(abcdab) = 4
//

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

    #[bench]
    fn bench_gs_strings_bad(b: &mut Bencher) {
        let n = 50;
        let haystack_s = ("bacbax".repeat(n - 1) + "bbbbb").repeat(n);
        let needle_s = "bacbax".repeat(n);
        let haystack: Vec<_> = haystack_s.split("x").collect();
        let needle: Vec<_> = needle_s.split("x").collect();
        b.iter(|| {
            gs_find(&haystack, &needle)
        });
    }

    #[bench]
    fn bench_brute_strings_bad(b: &mut Bencher) {
        let n = 50;
        let haystack_s = ("bacbax".repeat(n - 1) + "bbbbb").repeat(n);
        let needle_s = "bacbax".repeat(n);
        let haystack: Vec<_> = haystack_s.split("x").collect();
        let needle: Vec<_> = needle_s.split("x").collect();
        b.iter(|| {
            brute_force_fast(&haystack, &needle)
        });
    }

    #[bench]
    fn bench_gs_strings_good(b: &mut Bencher) {
        let n = 200;
        let haystack: Vec<_> = (0..n).map(|i| format!("foo{}", i)).collect();
        let needle: Vec<_> = (n - 10..n).map(|i| format!("foo{}", i)).collect();
        b.iter(|| {
            gs_find(&haystack, &needle)
        });
    }

    #[bench]
    fn bench_brute_strings_good(b: &mut Bencher) {
        let n = 200;
        let haystack: Vec<_> = (0..n).map(|i| format!("foo{}", i)).collect();
        let needle: Vec<_> = (n - 10..n).map(|i| format!("foo{}", i)).collect();
        b.iter(|| {
            brute_force_fast(&haystack, &needle)
        });
    }
}
