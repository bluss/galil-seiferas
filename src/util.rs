
use std::fmt;
pub(crate) struct Bytestring<'a>(pub &'a [u8]);

#[cfg(not(debug_assertions))]
macro_rules! println {
    ($($t:tt)*) => { }
}

impl<'a> fmt::Display for Bytestring<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for &elt in self.0 {
            for byte in ::std::ascii::escape_default(elt) {
                write!(f, "{}", byte as char)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
pub(crate) fn brute_force_search<T: Eq>(text: &[T], pattern: &[T]) -> Option<usize> {
    let n = text.len();
    let m = pattern.len();
    for i in 0..n - m + 1 {
        if text[i..i + m] == *pattern {
            return Some(i);
        }
    }
    None
}

#[test]
fn test_brute_force_search() {
    assert_eq!(brute_force_search(b"abcabcd", b"abc"), Some(0));
    assert_eq!(brute_force_search(b"abcabcd", b"abcd"), Some(3));
}


