
#[cfg(test)]
use std::fmt;

#[cfg(test)]
pub(crate) struct Bytestring<'a, T: 'a>(pub &'a [T]);

#[cfg(test)]
impl<'a> fmt::Display for Bytestring<'a, u8> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for &elt in self.0 {
            for byte in ::std::ascii::escape_default(elt) {
                write!(f, "{}", byte as char)?;
            }
        }
        Ok(())
    }
}

#[cfg(any(test, feature = "test-functions"))]
pub fn brute_force_search<T: Eq>(text: &[T], pattern: &[T]) -> Option<usize> {
    let n = text.len();
    let m = pattern.len();
    if n < m {
        return None;
    }
    'outer: for i in 0..n - m + 1 {

        /* to use memcmp:
         * it's a tradeoff; memcmp is faster with more pathological-y inputs!
         * for relistic inputs where we quickly find a mismatch at most
         * postions, it's faster using just single element get.
        if get!(text, i .. i + m) == pattern {
            return Some(i);
        }
        */

        for j in 0..m {
            if get!(text, i + j) != get!(pattern, j) {
                continue 'outer;
            }
        }
        return Some(i);
    }
    None
}



#[test]
fn test_brute_force_search() {
    assert_eq!(brute_force_search(b"abcabcd", b"abc"), Some(0));
    assert_eq!(brute_force_search(b"abcabcd", b"abcd"), Some(3));
    assert_eq!(brute_force_search(b"ab", b"abc"), None);
}

