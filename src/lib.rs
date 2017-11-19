
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
//!
//! # Crate Features
//!
//! The crate is always `no_std`


#![cfg_attr(not(test), no_std)]
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
mod test_util;
#[cfg(test)]
use test_util::Bytestring;
#[cfg(feature = "test-functions")]
pub use test_util::brute_force_search;


/// Test if `text` starts with `pattern`.
// One can use either a direct loop here, or use the libcore == for slices
// the latter will for example call memcmp in some situations.
// This function is intended for our use case here, where the (prefix of the)
// pattern is very short or empty
fn text_has_prefix<T: Eq>(text: &[T], pattern: &[T]) -> bool {
    longest_common_prefix_from(0, text, pattern) == pattern.len()
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

/// Find the greatest shared prefix, starting at from, of text and pattern.
/// Return the length of the prefix (including `from`).
///
/// Example:
///          __________
/// text:    aabaabaaabbb
/// pattern: aabaabaaaba
///              \.....x
///           from = 4 \ return value: from + .. = 4 + 6 = 10
fn longest_common_prefix_from<T: Eq>(from: usize, text: &[T], pattern: &[T]) -> usize {
    debug_assert!(pattern.len() <= text.len());
    debug_assert!(from <= pattern.len());
    let mut i = from;
    while i < pattern.len() {
        if get!(text, i) != get!(pattern, i) { return i; }
        i += 1;
    }
    i
}

#[test]
fn test_longest_common_prefix_from() {
    let a = b"abcabcaabc";
    let b = b"abcabcabc";
    assert_eq!(longest_common_prefix_from(0, a, b), 7);
    assert_eq!(longest_common_prefix_from(0, a, &b[1..]), 0);
}

/// The value *k* is a “large enough integer” whose usage becomes clear below;
/// [CR] shows k = 3 is the best, lowest value where the algorithm works
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
/// Compute HRP2, if the period for HRP1 is >= hrp2_period
fn hrp<T: Eq>(mut period: usize, pattern: &[T], hrp2_period: Option<usize>)
    -> (Option<Hrp>, Option<Hrp>)
{
    let k = GS_K;
    let m = pattern.len();
    let mut j = 0;        // pattern position
    let mut hrp1 = None;
    let hrp2_period_limit = hrp2_period.unwrap_or(0);

    while period + j < m {
        // find the greatest length (period + j) with the same period
        j = longest_common_prefix_from(j, pattern, get!(pattern, period..));

        let prefix_length = period + j;

        if prefix_length >= period * k {
            // we found the next HRP
            let next_hrp = Some(Hrp { period: period, len: prefix_length });
            match hrp1 {
                Some(_) => return (hrp1, next_hrp),
                None => {
                    hrp1 = next_hrp;
                    if period < hrp2_period_limit {
                        break;
                    }
                }
            }

            // periodicity lemma [GS]/[CR]
            //
            //  If a string x with p + q <= |x| has periods of length p and
            //  q, then it has a period of length gcd(p, q)
            //
            // period, j adjustments from [GS] Preprocessing A Pattern.
            //
            // pattern[..period + j] has period of length `period`
            // pattern[..period + j + 1] does not.
            // thus second period >= j
            // by the periodicity lemma.
            //
            // For example:
            //
            //   /.\/.\/.\/.  period = 3
            //   abcabcabcabdefghij  // the pattern
            //   \........./  prefix with period + j = 3 + 8 = 11
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
fn test_hrp_1() {
    let s = b"aabaabaabaabaabaabbbb";

    println!("s: {}", Bytestring(s));
    assert_matches!(hrp(1, s, None), (Some(Hrp { period: 3, len: 18 }), None));
    assert_matches!(hrp(2, s, None), (Some(Hrp { period: 3, len: 18 }), None));
    // the next is not a proper HRP since it is not basic (we're calling it with
    // a too low starting period)
    assert_matches!(hrp(4, s, None), (Some(Hrp { period: 6, len: 18 }), None));
}

#[test]
fn test_hrp_2() {
    //        1..
    //        2222...|...|..
    let s = b"aaabaaabaaabaa";
    assert_matches!(hrp(1, s, None),
                    (Some(Hrp { period: 1, len: 3 }),
                     Some(Hrp { period: 4, len: 14 })));
}

#[test]
fn test_hrp_length() {
    // test a string short one char
    let mut s = "aab".repeat(GS_K);
    assert_matches!(hrp(1, s.as_bytes(), None), (Some(Hrp { period: 3, .. }), None));
    s.pop();
    assert_matches!(hrp(1, s.as_bytes(), None), (None, None));
}

#[test]
fn test_hrp_fuzz_1() {
    // this input is tricky because hrp1.len < hrp2.period
    let s = b"baababaababaabaababaabaabaababaababaabaababaabaabaababaababaabaab\
            abaabaababaababaabaababaabaababaababaabaababaababaabababaabaababaab\
            abaabaababaababaabaababaababaabaababaabaab";
    let hrps = hrp(1, s, None);
    assert_matches!(hrps, (Some(Hrp { period: 5, len: 15 }), Some(Hrp { period: 24, len: 74 })));
    let (u, _, hrp1) = decompose(s);
    assert_eq!(u.len(), 5);
    assert_matches!(hrp1, None);
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
/// cf. length of prefix period in [GS]
///
/// Define: |HRP(x)| to mean the period of HRP(x) (a prefix period of x).
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
    let (mut hrp1_opt, mut hrp2_opt) = hrp(1, pattern, None);
    loop {
        if let Some(hrp1) = hrp1_opt {
            if let Some(hrp2) = hrp2_opt {
                // if x = v1 x' where v1 is a prefix period of x (v1 is HRP1)
                // x' = x[p..] where p = |v1|
                j += hrp1.period;

                // size is nondecreasing: so start with the HRP1(x) period.
                // compute HRP1(x') and (if needed) HRP2(x')
                let (h1, h2) = hrp(hrp1.period, get!(pattern, j..),
                                   Some(hrp2.period));
                hrp1_opt = h1;
                if let Some(ref hrp1) = h1 {
                    if hrp1.period >= hrp2.period {
                        hrp2_opt = h2;
                    }
                    continue;
                }
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
    assert_matches!(hrp, Some(Hrp { period: 4, len: 15 }));

    let s = String::from("aaa") + &"ab".repeat(4);
    let (u, v, hrp) = decompose(s.as_bytes());
    println!("u,v = {},{} hrp1={:?}", Bytestring(u), Bytestring(v), hrp);
    assert_eq!(u.len(), 0);
    assert_matches!(hrp, Some(Hrp { period: 1, .. }));

    let s = b"aaabaaabaaabaabbbb";
    let (u, v, hrp) = decompose(s);
    println!("u,v = {},{} hrp1={:?}", Bytestring(u), Bytestring(v), hrp);
    assert_eq!(u.len(), 1);
    assert_matches!(hrp, Some(Hrp { period: 4, .. }));

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
    assert_eq!(u.len(), 3);
    assert_matches!(hrp, Some(Hrp { period: 10, len: _ }));
}

#[test]
fn test_decompose_period_1() {
    let period = "a";
    let pattern = period.repeat(50 / period.len());
    let pattern = pattern.as_bytes();
    let (u, v, hrp) = decompose(pattern);
    assert_matches!(::hrp(1, pattern, None), (Some(_), None));
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

#[test]
fn test_decompose_period_mega() {
    let alphabet = "abcdefghi";
    let mut s = String::from("");
    for ch in alphabet.chars() {
        s.push(ch);
        s = s.repeat(4);
    }
    println!("{}", &s[..70]);

    const HRP1: usize = 87381;
    let (u, v, hrp) = decompose(s.as_bytes());
    assert_eq!(u.len() + v.len(), s.len());
    assert_matches!(hrp, Some(Hrp { period: HRP1, .. }));
}


/// Assert that the input = u v is a perfect factorization
#[cfg(debug_assertions)]
fn assert_perfect_decomposition<T: Eq>(k: usize, u: &[T], v: &[T]) {
    // require that a decomp x = u v
    // that u is "short" and v is k-simple.
    // k-simple means it has at most one k-HRP which also means it has no k-HRP2
    assert!(k >= 3);
    if let (Some(hrp1), hrp2) = hrp(1, v, None) {
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

/// Search `text` for the `pattern` with the requirement that the pattern
/// is k-simple; which means it has at most one k-HRP.
///
/// `start_pos` is the position to start the search, and it is updated after
/// the function returns with a match.
fn search_simple<T: Eq>(text: &[T], pattern: &[T],
                        start_pos: &mut usize,
                        start_j: &mut usize,
                        hrp1: &Option<Hrp>)
    -> Option<usize>
{
    debug_assert!(pattern.len() <= text.len());
    debug_assert_eq!(hrp(1, pattern, None), (*hrp1, None));

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

    let mut pos = *start_pos; // text position
    let mut j = *start_j;     // pattern position
    while pos <= n - m {
        j = longest_common_prefix_from(j, get!(text, pos..), pattern);
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
            *start_j = j;
            return Some(match_pos);
        }
    }
    None
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
    let (mut pos, mut j) = (0, 0);
    while let Some(i) = search_simple(get!(text, u.len()..), v,
                                      &mut pos, &mut j, &hrp1)
    {
        if text_has_prefix(get!(text, i..), u) {
            return Some(i);
        }
    }
    None
}

// Test that gs_find(text, pat) has the same result as str::find
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
    // found by cargo fuzz; no bug but proved the need of scope_l/scope_r check in hrp
    let s = "\x0abaaabaaabaaabaaabaaabbbbb";
    let n = "aaabaaabaaabaaabbbbb";
    test_str!(s, n);
}

// Test that the substring &text[range] is found inside text.
// Note that we can only find the first of identical substrings.
#[cfg(test)]
defmac!(assert_find_substring text, range => {
    let text = &text[..];
    let needle = &text[range.clone()];
    assert!(text_has_prefix(&text[range.start..], needle),
            "buggy test: not a substring at {:?}", range);
    assert_eq!(Some(range.start), gs_find(text, needle));
});


#[test]
fn test_find_fuzz_1() {
    // cargo fuzz found this one, but it required feeding it with fib words as dict
    let data = b"\x9caababaababaabaababaababaabaababaabaabaababaababaabaababaab\
               aabaababaababaabaababaabaababaababaabaababaabaababaababaabaababa\
               ababaabababaabaababaababaabaababaababaabaababaababaabaababaabaab\
               \x9c\x9c\x9cbaabaabababaabababaabababaabaababaabaab\x9c\x9c\x9c\
               \x9c\x9c\x28\x9c\x9c\x9c\x9c\x9c\x9c\x2a\x9c\x9c\x9c\x9c\x9c\x9c\
               \x9caabaababaabaab\x9c\x9c\x9c\x9c\x9c\x28\x9c\x9c\x9c\x9c\x9c\
               \x9c\x9c\x9c\x27\x9c\x9c\x9c\x9c\x9c\x9c\x9c\x9c\x9c";
    assert_find_substring!(data, 13 .. 13 + 266);
}

#[test]
fn test_find_fuzz_2() {
    let data = b"abaabaababaababaabaababaabaababababaabaababababaabaababababaab\
               aababaab\xffaabaabaab\x00\x00\xff\x28\xffaab\xffaabaabaab\x00\
               \x00\xff\x28\xff";
    assert_find_substring!(data, 21..21 + 68);
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
    use super::test_util::brute_force_search;

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

    #[bench]
    fn bench_g_decompose_normal1(b: &mut Bencher) {
        let pattern = "english";

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
    fn bench_brute_periodic2_10(b: &mut Bencher) {
        let n = 10;
        let haystack = haystack!(n);
        let pattern = "ab".repeat(n);


        b.iter(|| {
            brute_force_search(haystack.as_bytes(), pattern.as_bytes())
        });
        b.bytes = haystack.len() as u64;
    }

    #[bench]
    fn bench_brute_periodic2_50(b: &mut Bencher) {
        let n = 50;
        let haystack = haystack!(n);
        let pattern = "ab".repeat(n);


        b.iter(|| {
            brute_force_search(haystack.as_bytes(), pattern.as_bytes())
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
    fn bench_brute_periodic2_large(b: &mut Bencher) {
        let n = PER_LARGE;
        let haystack = haystack!(n);
        let pattern = "ab".repeat(n);


        b.iter(|| {
            brute_force_search(haystack.as_bytes(), pattern.as_bytes())
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

    // test some regular non-periodic words
    #[bench]
    fn bench_gs_find_itself1(b: &mut Bencher) {
        let haystack = "itself";
        let pattern = "itself";

        b.iter(|| {
            gs_find(haystack.as_bytes(), pattern.as_bytes())
        });
        b.bytes = haystack.len() as u64;
    }

    #[bench]
    fn bench_gs_find_itself2(b: &mut Bencher) {
        let haystack = "the word itself";
        let pattern = "itself";

        b.iter(|| {
            gs_find(haystack.as_bytes(), pattern.as_bytes())
        });
        b.bytes = haystack.len() as u64;
    }

    #[bench]
    fn bench_gs_find_itself3(b: &mut Bencher) {
        let haystack = "this is actually a longer text where them self tself\
            could be tricked by and so on.".repeat(10) + "itself.";
        let pattern = "itself";

        b.iter(|| {
            gs_find(haystack.as_bytes(), pattern.as_bytes())
        });
        b.bytes = haystack.len() as u64;
    }

    #[bench]
    fn bench_gs_find_itself4(b: &mut Bencher) {
        let haystack = "this is actually a longer text where them xxxx xxxxx\
            could be tricked by and so on.".repeat(10) + "itself.";
        let pattern = "itself";

        b.iter(|| {
            gs_find(haystack.as_bytes(), pattern.as_bytes())
        });
        b.bytes = haystack.len() as u64;
    }

    #[bench]
    fn bench_brute_itself4(b: &mut Bencher) {
        let haystack = "this is actually a longer text where them xxxx xxxxx\
            could be tricked by and so on.".repeat(10) + "itself.";
        let pattern = "itself";

        b.iter(|| {
            brute_force_search(haystack.as_bytes(), pattern.as_bytes())
        });
        b.bytes = haystack.len() as u64;
    }

    #[bench]
    fn bench_gs_find_itself5(b: &mut Bencher) {
        let haystack = "this is actually a longer text where them itsel itselg\
            could be tricked by and so on.".repeat(10) + "itself.";
        let pattern = "itself";

        b.iter(|| {
            gs_find(haystack.as_bytes(), pattern.as_bytes())
        });
        b.bytes = haystack.len() as u64;
    }

    #[bench]
    fn bench_brute_itself5(b: &mut Bencher) {
        let haystack = "this is actually a longer text where them itsel itselg\
            could be tricked by and so on.".repeat(10) + "itself.";
        let pattern = "itself";

        b.iter(|| {
            brute_force_search(haystack.as_bytes(), pattern.as_bytes())
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
            brute_force_search(&haystack, &needle)
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
            brute_force_search(&haystack, &needle)
        });
    }
}
