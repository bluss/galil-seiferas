

//!
//! String search in constant space, linear time, for nonorderable alphabets.
//!
//! In Rust terms this means we can define the function:
//!
//! ```rust
//! fn find_substring<T>(text: &[T], pattern: &[T]) -> Option<usize>
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
//! Note that the Crochemore-Perrin ("Two Way" algorithm) is superior if
//! the alphabet has a linear order.
//!
//! # References
//!
//! Crochemore-Rytter's description of the Galil-Seiferas algoritm has been
//! very helpful and it explains how it works in concepts that we could
//! implement:
//!
//! M. Crochemore and W. Rytter,
//! *Squares, Cubes, and Time-Space Efficient String Searching*,
//! Algorithmica (1995)


#![feature(test)]

#[cfg(test)]
#[macro_use] extern crate matches;
#[cfg(test)]
#[macro_use] extern crate defmac;

use std::fmt;

struct Bytestring<'a>(&'a [u8]);

#[cfg(not(debug_assertions))]
macro_rules! println {
    ($($t:tt)*) => { }
}

impl<'a> fmt::Display for Bytestring<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for &elt in self.0 {
            for byte in std::ascii::escape_default(elt) {
                write!(f, "{}", byte as char)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
fn naive_search<T: Eq>(text: &[T], pattern: &[T]) -> Option<usize> {
    let n = text.len();
    let m = pattern.len();
    for i in 0..n - m + 1 {
        if text[i..i + m] == *pattern {
            return Some(i);
        }
    }
    None
}

fn text_has_prefix<T: Eq, FEq>(text: &[T], pattern: &[T], _f: &mut FEq) -> bool
    where FEq: FnMut(&T, &T) -> bool
{
    debug_assert!(text.len() >= pattern.len());
    text[..pattern.len()] == *pattern
    //text.iter().zip(pattern).all(move |(ea, eb)| f(ea, eb))
}

#[test]
fn test_naive() {
    assert_eq!(naive_search(b"abcabcd", b"abc"), Some(0));
    assert_eq!(naive_search(b"abcabcd", b"abcd"), Some(3));
}


#[cfg(never)]
fn not_so_naive<T: Eq>(text: &[T], pattern: &[T]) -> Option<usize> {
    let n = text.len();
    let m = pattern.len();
    if m == 0 {
        return Some(0);
    } else if m == 1 {
        return text.iter().position(|elt| *elt == pattern[0]);
    }

    let (step, skip) = if pattern[0] == pattern[1] {
        (2, 1)
    } else {
        (1, 2)
    };

    unimplemented!();
    for i in 0..n - m + 1 {
        if text[i..i + m] == *pattern {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
pub fn gl_scopes(k: usize, pattern: &[u8]) -> Vec<(usize, usize)> {

    // Longest prefix z
    // with prefix period v.
    //
    // Thus, we can consider the longest prefix z of x which has the prefix
    // period v. Then the scope of v is the interval of integers [L, R] defined
    // by:
    //
    //  L = |v²| and R = |z|
    //
    // a highly repeating prefix (HRP).
    // with k = 3 we have a "cube prefix" (three times repreated)
    //
    // What happens if the pattern has at most one k-HRP? We say
    // that such a pattern is k-simple.
    //
    // When k <= 3, words satisfy a remarkable combinatorial property:
    // 
    //  each pattern p can be decomposed into uv where u is "short" and v is
    //  a k-simple word.
    // 
    // The composition p = uv is k-perfect iff v is k-simple. and |u| < 2per(v)
    //
    //
    let mut scope = Vec::new();
    let mut period = 1;
    let mut j = 0;
    let m = pattern.len();
    let mut n_hrp = 0;
    while period + j < m {
        // scan while the string is periodic
        println!("j={}, period={}, scope={:?}", j, period, scope);
        while period + j < m && pattern[j] == pattern[period + j] {
            j += 1;
        }
        if j > 1 {
            println!("found period={}, j={} which is {:?}", period, j, &pattern[..j]);
            n_hrp += 1;
            println!("{}-HRP{} is {:?}", k, n_hrp, &pattern[..period]);
        }
        if period <= (period + j) / k {
            scope.push((2 * period, period + j));
        }
        /* if j belongs to some scope */
        if let Some(&s) = scope.iter().find(|s| s.0 <= j && j <= s.1) {
            period += s.0 / 2;
            j -= s.0 / 2;
        } else {
            period += j / k + 1;
            j = 0;
        }
    }
    println!("end with scope={:?}", scope);
    scope
}

#[test]
fn test_gl_scopes() {
    gl_scopes(2, b"abaabaabaabaaaaa");
    assert_eq!(gl_scopes(2, b"abaababaabaababaababaabaababaabaab"),
               vec![(6, 6), (10, 11), (16, 19), (26, 32)]);
}

type T = u8;

#[derive(Copy, Clone, Debug, PartialEq)]
/// Highly-repeating-prefix
struct Hrp {
    period: usize,
    len: usize,
}

impl Hrp {
    #[cfg(test)]
    fn from(period: usize, len: usize) -> Self {
        Hrp { period, len }
    }

    fn special_position(&self, hrp2: &Self) -> usize {
        //let max = hrp2.len - 1;
        let max = hrp2.period - 1;
        debug_assert!(max >= 1);
        max - max % self.period
    }
}

/// HRP: Highly-repeating-prefix
///
/// k-HRP means that the prefix consists of at least k periods.
///
/// Find the first k-HRP with period >= `period`; return its period and 
/// the length of the prefix.
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
fn hrp(k: usize, mut period: usize, pattern: &[T]) -> Option<Hrp> {
    println!("Enter hrp: k={}, period={}, pattern={}", k, period, Bytestring(pattern));
    let m = pattern.len();
    let mut j = 0;
    while period + j < m {
        while period + j < m && pattern[j] == pattern[period + j] {
            j += 1;
        }
        let prefix_len = period + j;
        if period <= (period + j) / k {
            println!("Exit  hrp: k={}, period={}, len={}, prefix={}", k, period,
                     prefix_len, Bytestring(&pattern[..prefix_len]));

            return Some(Hrp { period: period, len: prefix_len });
        }
        // [ a b a b x x ]
        //    \ 
        //     j  
        // period == 1; j == 0; k == 3; period += 1;
        //
        // [ a b a b x x ]
        //          \
        //           j
        // period == 2; j == 3; k == 3; period += 2;
        period += j / k + 1;
        j = 0;
    }
    println!("Exit  hrp: k={}, no HRP", k);
    None
}

#[test]
fn test_hrp() {
    let s = b"aabaabaabaabbbb";

    assert_eq!(hrp(2, 1, s), Some(Hrp::from(1, 2)));
    assert_eq!(hrp(2, 2, s), Some(Hrp::from(3, 12)));
    assert_eq!(hrp(3, 2, s), Some(Hrp::from(3, 12)));
    assert_eq!(hrp(2, 4, s), Some(Hrp::from(6, 12)));
}

fn assert_perfect_decomp(k: usize, input: (&[T], &[T])) {
    // require that a decomp x = u v
    // that u is "short" and v is k-simple.
    // k-simple means it has at most one k-HRP which also means it has no k-HRP2
    assert!(k >= 3);
    let (u, v) = input;
    if let Some(hrp1) = hrp(k, 1, v) {
        if let Some(hrp2) = hrp(k, hrp1.period * 2, v) {
            panic!("Factorization u, v = {} , {} is not k-simple because
                    v's {}-HRP1 is {:?} and {}-HRP2 is {:?}",
                    Bytestring(u), Bytestring(v), k, hrp1, k, hrp2);
            
        }
    }
    // ok
}

/// Decompose `pattern` into two words u, v where u is "short" and v is k-simple.
///
/// k-simple: v has at most one k-HRP; if it exists, it is returned as well.
fn decompose(k: usize, pattern: &[T]) -> (&[T], &[T], Option<Hrp>) {
    // When k >= 3, words satisfy a remarkable combinatorial property:
    // 
    //  each pattern p can be decomposed into uv where u is "short" and v is
    //  a k-simple word.
    // 
    // The composition p = uv is k-perfect iff v is k-simple. and |u| < 2per(v)
    //

    debug_assert!(k >= 3);
    let mut j = 0;
    let mut hrp1_opt = hrp(k, 1, pattern);
    let mut hrp2_opt = hrp1_opt.and_then(|hrp1| hrp(k, hrp1.period * 2, pattern));
    loop {
        if let Some(hrp1) = hrp1_opt {
            if let Some(hrp2) = hrp2_opt {

                println!("HRP1={} with prefix length={} and special position={}", Bytestring(&pattern[..hrp1.period]), hrp1.len, hrp1.special_position(&hrp2));
                j += hrp1.special_position(&hrp2);

                // x' = x[j..]
                // compute HRP1(x')
                hrp1_opt = hrp(k, hrp1.period, &pattern[j..]); // use hrp1 period; size is nodecreasing
                if let Some(hrp1) = hrp1_opt {
                    //if hrp1.period >= hrp2.period {
                    // HRP2(x')
                    hrp2_opt = hrp(k, hrp1.period * 2, &pattern[j..]);
                    //}
                } else {
                    // breaking soon
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }
    let (a, b) = pattern.split_at(j);
    println!("decomposition is {}, {}", Bytestring(a), Bytestring(b));
    #[cfg(debug_assertions)]
    assert_perfect_decomp(k, (a, b));
    (a, b, hrp1_opt)
}

#[test]
fn test_decompose() {
    let s = b"banana";
    assert_matches!(decompose(3, s), (_, _, None));
    let s = b"aaabaaabaaabaabbbb";
    assert_matches!(decompose(3, s), (_, _, None));
    let s = b"abababababababababababcabcabcabcabc";
    assert_matches!(decompose(3, s), (_, _, Some(Hrp { period: 2, len: 10 })));
    let s = b"ananananananananan in the face";
    assert_matches!(decompose(3, s), (_, _, Some(_)));
}

const GS_K: usize = 3;

pub fn cube_search(text: &[T], pattern: &[T]) -> Option<usize> {
    if pattern.len() > text.len() {
        return None;
    } else if pattern.is_empty() {
        return Some(0); // trivial match
    }
    let (u, v, hrp1) = decompose(GS_K, pattern);
    let mut pos = 0;
    while let Some(i) = search_simple(&text[u.len()..], v, &mut pos, &hrp1) {
        if text_has_prefix(&text[i..], u, &mut PartialEq::eq) {
            return Some(i);
        }
    }
    None
}

#[test]
fn test_cube_search() {
    /*
    assert_eq!(cube_search(b"abc", b"a"), Some(0));
    assert_eq!(cube_search(b"abc", b""), Some(0));
    assert_eq!(cube_search(b"abc", b"x"), None);
    */

    defmac!(test_str text, pat => assert_eq!(text.find(pat), cube_search(text.as_bytes(), pat.as_bytes())));
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

/// pattern is k-simple which means it has at most one k-HRP
fn search_simple(text: &[T], pattern: &[T], start_pos: &mut usize, hrp1: &Option<Hrp>) -> Option<usize> {
    // 
    #[cfg(debug_assertions)]
    assert_perfect_decomp(GS_K, (&[], pattern));
    debug_assert!(pattern.len() <= text.len());
    let n = text.len();
    let m = pattern.len();
    //let hrp1 = hrp(GS_K, 1, pattern);
    println!("search_simple hrp={:?}", hrp1);
    println!("search_simple text={}, pattern={}", Bytestring(text), Bytestring(pattern));

    let (has_scope, scope_l, scope_r) = if let Some(hrp1) = *hrp1 {
        // Scope of the k-HRP1 is [L, R]
        // compute the scope L = |v²|, R = z
        let scope_l = hrp1.period * 2;
        let scope_r = hrp1.len;
        println!("scope_l={}, scope_r={}", scope_l, scope_r);
        debug_assert!(scope_l <= scope_r);
        (true, scope_l, scope_r)
    } else {
        (false, 0, 0)
    };

    let mut pos = *start_pos;
    let mut j = 0;
    while pos <= n - m {
        while j < m && pattern[j] == text[pos + j] {
            j += 1;
        }
        if j == m {
            *start_pos = pos + 1;
            return Some(pos);
        }
        if has_scope && j >= scope_l && j <= scope_r {
            pos += scope_l / 2;
            j -= scope_l / 2;
        } else {
            j = 0;
            pos += j / GS_K + 1;
        }
    }
    *start_pos = pos;
    None
}

// find first `out.len()` k-HRP scopes of `pattern`
#[cfg(test)]
fn gl_first_scopes(k: usize, pattern: &[u8], scope_out: &mut [(usize, usize)]) {

    //
    //
    // Define: primitive is not a power of another word and is not the empty
    // string.
    //
    // Longest prefix z
    // with prefix period v.
    //
    // # Scope
    //
    // Thus, we can consider the longest prefix z of x which has the prefix
    // period v. Then the scope of v is the interval of integers [L, R] defined
    // by:
    //
    //  L = |v²| and R = |z|
    //
    // a highly repeating prefix (HRP).
    // with k = 3 we have a "cube prefix" (three times repreated)
    //
    // What happens if the pattern has at most one k-HRP? We say
    // that such a pattern is k-simple.
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
    //
    let scope = scope_out;
    let mut scope_index = 0;
    let mut period = 1;
    let mut j = 0;
    let m = pattern.len();
    let mut n_hrp = 0;
    while period + j < m {
        // scan while the string is periodic
        println!("j={}, period={}, scope={:?}", j, period, scope);
        while period + j < m && pattern[j] == pattern[period + j] {
            j += 1;
        }
        if j > 1 {
            println!("found period={}, j={} which is {}", period, j, Bytestring(&pattern[..j]));
            n_hrp += 1;
            println!("{}-HRP{} is {:?}", k, n_hrp, &pattern[..period]);
        }
        if period <= (period + j) / k {
            scope[scope_index] = (2 * period, period + j);
            scope_index += 1;
            if scope_index >= scope.len() {
                break;
            }
        }
        /* if j belongs to some scope */
        if let Some(&s) = scope[..scope_index].iter().find(|s| s.0 <= j && j <= s.1) {
            period += s.0 / 2;
            j -= s.0 / 2;
        } else {
            period += j / k + 1;
            j = 0;
        }
    }
    println!("end with scope={:?}", scope);
}

#[test]
fn test_gl_scopes_2() {
    let mut scope = [(0, 0); 4];
    let answer = [(6, 6), (10, 11), (16, 19), (26, 32)];
    gl_first_scopes(2, b"abaababaabaababaababaabaababaabaab", &mut scope);
    assert_eq!(scope, answer);
}

// Galil-Seiferas algorithm
#[cfg(test)]
pub fn gl_search(text: &[u8], pattern: &[u8]) -> Option<usize> {

    // const K: usize = 4;

    // given pattern x
    // given text y
    //
    // reach, where i in [0, m)
    // reach(i) = i + max { i' <= m - i : x[0..i'] = x[i + 1 .. i' + i + 1] }
    // prefix x[0..p] of x is a prefix period if it is basic and reach(p) >= k p
    //
    // The preprocessing phase of the Galil-Seiferas algorithm consists in
    // finding a decomposition uv of x such that v has at most one prefix period
    // and |u|=O(per(v)). Such a decomposition is called a perfect
    // factorization.
    //
    // Then the searching phase consists of scanning the text y for every
    // occurrences of v and when v occurs to check naively if u occurs just
    // before in y.
    //
    //
    // http://www-igm.univ-mlv.fr/~lecroq/string/node25.html


    // newP1 find shortest prefix period
    // newP2 find second shortest prefix period
    //
    None
}


// more thorough tests in the tests/ directory
#[test]
fn basic_test() {
    let body = "G";
    let pattern = "GCAGAGAG";
    let search = cube_search;
    search(body.as_bytes(), pattern.as_bytes());
    macro_rules! test {
        ($body:expr, $pattern:expr) => {
            assert_eq!($body.find($pattern),
                       search($body.as_bytes(), $pattern.as_bytes()),
                       "assertion failed for body={}, pattern={}",
                       $body, $pattern)
        }
    }
    test!("xyz", "a");
    test!("xyz", "x");
    test!("xyz", "y");
    test!("xyz", "z");
    test!("substrinstring", "string");
    test!("abcαaαβγ", "αβ");

    //let result = search(&[1729, 1, 1729, 3, 4], &[1729, 3]);
    //assert_eq!(result, Some(2));
}

#[cfg(test)]
mod benches {
    extern crate test;
    use self::test::Bencher;
    use super::cube_search;
    use super::decompose;
    use super::GS_K;

    #[test]
    fn test_periodic() {
        let n = 10;
        let haystack = ("bb".to_string() + &"ab".repeat(n - 1)).repeat(n);
        let pattern = "ab".repeat(n);
        let res = cube_search(haystack.as_bytes(), pattern.as_bytes());
        println!("{:?}", res);
    }

    #[bench]
    fn bench_decompose(b: &mut Bencher) {
        let n = 10;
        let pattern = "ab".repeat(n);

        b.iter(|| {
            decompose(GS_K, pattern.as_bytes());
        });
        b.bytes = pattern.len() as u64;
    }

    #[bench]
    fn bench_periodic2(b: &mut Bencher) {
        let n = 10;
        let haystack = ("bb".to_string() + &"ab".repeat(n - 1)).repeat(n);
        let pattern = "ab".repeat(n);


        b.iter(|| {
            cube_search(haystack.as_bytes(), pattern.as_bytes())
        });
        b.bytes = haystack.len() as u64;
    }
}
