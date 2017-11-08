
#[cfg(unused)]
use std::fmt;

#[cfg(not(debug_assertions))]
macro_rules! println {
    ($($t:tt)*) => { }
}

#[cfg(unused)]
pub(crate) struct Bytestring<'a, T: 'a>(pub &'a [T]);

#[cfg(unused)]
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

// using slow loop
#[cfg(test)]
pub(crate) fn brute_force_search<T: Eq>(text: &[T], pattern: &[T]) -> Option<usize> {
    let n = text.len();
    let m = pattern.len();
    if n < m {
        return None;
    }
    'outer: for i in 0..n - m + 1 {
        for j in 0..m {
            if text[i + j] != pattern[j] {
                continue 'outer;
            }
        }
        return Some(i);
    }
    None
}

// using memcmp
#[cfg(test)]
pub(crate) fn brute_force_fast<T: Eq>(text: &[T], pattern: &[T]) -> Option<usize> {
    let n = text.len();
    let m = pattern.len();
    if n < m {
        return None;
    }
    for i in 0..n - m + 1 {
        if &text[i .. i + m] == pattern {
            return Some(i);
        }
    }
    None
}



#[test]
fn test_brute_force_search() {
    assert_eq!(brute_force_search(b"abcabcd", b"abc"), Some(0));
    assert_eq!(brute_force_search(b"abcabcd", b"abcd"), Some(3));
    assert_eq!(brute_force_fast(b"abcabcd", b"abc"), Some(0));
    assert_eq!(brute_force_fast(b"abcabcd", b"abcd"), Some(3));
    assert_eq!(brute_force_fast(b"ab", b"abc"), None);
}

