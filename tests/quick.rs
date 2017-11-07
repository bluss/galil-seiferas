
#![allow(dead_code)]

extern crate galil_seiferas;

#[macro_use] extern crate quickcheck;
extern crate odds;
#[macro_use] extern crate macro_attr;
#[macro_use] extern crate newtype_derive;


use galil_seiferas::gs_find;
use std::ops::Deref;

use odds::string::StrExt;

use quickcheck as qc;
use quickcheck::TestResult;
use quickcheck::Arbitrary;
use quickcheck::quickcheck;

#[derive(Copy, Clone, Debug)]
/// quickcheck Arbitrary adaptor - half the size of `T` on average
struct Short<T>(T);

impl<T> Deref for Short<T> {
    type Target = T;
    fn deref(&self) -> &T { &self.0 }
}

impl<T> Arbitrary for Short<T>
    where T: Arbitrary
{
    fn arbitrary<G: qc::Gen>(g: &mut G) -> Self {
        let sz = g.size() / 2;
        Short(T::arbitrary(&mut qc::StdGen::new(g, sz)))
    }

    fn shrink(&self) -> Box<Iterator<Item=Self>> {
        Box::new((**self).shrink().map(Short))
    }
}

macro_attr! {
    #[derive(Clone, Debug, NewtypeDeref!)]
    struct Text(String);
}

static ALPHABET: &'static str = "abñòαβ\u{3c72}";
static SIMPLEALPHABET: &'static str = "ab";

impl Arbitrary for Text {
    fn arbitrary<G: qc::Gen>(g: &mut G) -> Self {
        let len = u16::arbitrary(g);
        let mut s = String::with_capacity(len as usize);
        let alpha_len = ALPHABET.chars().count();
        for _ in 0..len {
            let i = usize::arbitrary(g);
            let i = i % alpha_len;
            s.push(ALPHABET.chars().nth(i).unwrap());
        }
        Text(s)
    }
    fn shrink(&self) -> Box<Iterator<Item=Self>> {
        Box::new(self.0.shrink().map(Text))
    }
}

/// Text from an alphabet of only two letters
macro_attr! {
    #[derive(Clone, Debug, NewtypeDeref!)]
    struct SimpleText(String);
}

impl Arbitrary for SimpleText {
    fn arbitrary<G: qc::Gen>(g: &mut G) -> Self {
        let len = u16::arbitrary(g);
        let mut s = String::with_capacity(len as usize);
        let alpha_len = SIMPLEALPHABET.chars().count();
        for _ in 0..len {
            let i = usize::arbitrary(g);
            let i = i % alpha_len;
            s.push(SIMPLEALPHABET.chars().nth(i).unwrap());
        }
        SimpleText(s)
    }
    fn shrink(&self) -> Box<Iterator<Item=Self>> {
        Box::new(self.0.shrink().map(SimpleText))
    }
}

/// Fibonacci word or composed thereof
///
/// S0 = 0
/// S1 = 01
/// Sn = Sn-1 Sn-2
macro_attr! {
    #[derive(Clone, Debug, NewtypeDeref!)]
    struct FibWord(String);
}

// The letters of the fib alphabet
const F0: char = 'a';
const F1: char = 'b';

const S0: &str = "a";
const S1: &str = "ab";

impl FibWord {
    fn new(n: usize) -> Self {
        let mut f = FibWord(String::new());
        Self::write(n, &mut f.0);
        f
    }

    fn write(n: usize, into: &mut String) {
        match n {
            0 => into.push_str(S0),
            1 => into.push_str(S1),
            other => {
                FibWord::write(other - 1, into);
                FibWord::write(other - 2, into);
            }
        }
    }
}

#[test]
fn test_fib_word() {
    assert_eq!(&*FibWord::new(2), "aba");
    assert_eq!(&*FibWord::new(4), "abaababa");
}

impl Arbitrary for FibWord {
    fn arbitrary<G: qc::Gen>(g: &mut G) -> Self {
        let n = u8::arbitrary(g) % 20;
        let mut a = FibWord::new(n as usize);
        if bool::arbitrary(g) {
            let n = u8::arbitrary(g) % 10;
            FibWord::write(n as usize, &mut a.0);
        }
        a
    }
    fn shrink(&self) -> Box<Iterator<Item=Self>> {
        Box::new(self.0.shrink().map(FibWord))
    }
}

#[derive(Clone, Debug)]
struct ShortText(String);
// Half the length of Text on average
impl Arbitrary for ShortText {
    fn arbitrary<G: qc::Gen>(g: &mut G) -> Self {
        let len = u16::arbitrary(g) / 2;
        let mut s = String::with_capacity(len as usize);
        let alpha_len = ALPHABET.chars().count();
        for _ in 0..len {
            let i = usize::arbitrary(g);
            let i = i % alpha_len;
            s.push(ALPHABET.chars().nth(i).unwrap());
        }
        ShortText(s)
    }
    fn shrink(&self) -> Box<Iterator<Item=Self>> {
        Box::new(self.0.shrink().map(ShortText))
    }
}

pub fn contains(hay: &str, n: &str) -> bool {
    gs_find(hay.as_bytes(), n.as_bytes()).is_some()
}

pub fn find(hay: &str, n: &str) -> Option<usize> {
    gs_find(hay.as_bytes(), n.as_bytes())
}

pub fn contains_rev(hay: &str, n: &str) -> bool {
    let _ = (hay, n);
    unimplemented!()
}

pub fn rfind(hay: &str, n: &str) -> Option<usize> {
    let _ = (hay, n);
    unimplemented!()
}

#[test]
fn test_contains() {
    fn prop(a: Text, b: Short<Text>) -> TestResult {
        let a = &a.0;
        let b = &b[..];
        let truth = a.contains(b);
        TestResult::from_bool(contains(&a, &b) == truth)
    }
    quickcheck(prop as fn(_, _) -> _);
}

#[ignore]
#[test]
fn test_contains_rev() {
    fn prop(a: Text, b: Short<Text>) -> TestResult {
        let a = &a.0;
        let b = &b[..];
        let truth = a.contains(b);
        TestResult::from_bool(contains_rev(&a, &b) == truth)
    }
    quickcheck(prop as fn(_, _) -> _);
}

#[test]
fn test_find_regular_str() {
    fn prop(a: String, b: String) -> TestResult {
        let a = &a[..];
        let b = &b[..];
        let truth = a.find(b);
        TestResult::from_bool(find(&a, &b) == truth)
    }
    quickcheck(prop as fn(_, _) -> _);
}

#[test]
fn test_find_short() {
    fn prop(a: Text, b: Short<Text>) -> TestResult {
        let a = &a.0;
        let b = &b[..];
        let truth = a.find(b);
        TestResult::from_bool(find(&a, &b) == truth)
    }
    quickcheck(prop as fn(_, _) -> _);
}

quickcheck! {
    fn test_find_longer_simple(a: SimpleText, b: SimpleText) -> () {
        // find all
        let mut a = &a[..];
        let b = &b[..];
        let mut n = 10;
        while let Some(i) = a.find(b) {
            assert_eq!(find(&a, &b), Some(i));
            // drop the char at i.
            let mut iter = a[i..].chars();
            iter.next();
            a = iter.as_str();
            n -= 1;
            if n == 0 { return; }
        }
        assert_eq!(find(a, b), None);
    }

    fn test_find_fib_in_simple(a: SimpleText, b: FibWord) -> () {
        let a = &a.0;
        let b = &b[..];
        let truth = a.find(b);
        assert_eq!(find(&a, &b), truth);
    }

    fn test_find_fib_in_fib(a: FibWord, b: FibWord) -> () {
        // find all
        let mut a = &a[..];
        let b = &b[..];
        let mut n = 10;
        while let Some(i) = a.find(b) {
            assert_eq!(find(&a, &b), Some(i));
            // drop the char at i.
            let mut iter = a[i..].chars();
            iter.next();
            a = iter.as_str();
            n -= 1;
            if n == 0 { return; }
        }
        assert_eq!(find(a, b), None);
    }

    fn test_find_simple_in_fib(a: FibWord, b: SimpleText) -> () {
        let a = &a.0;
        let b = &b[..];
        let truth = a.find(b);
        assert_eq!(find(&a, &b), truth);
    }
}

#[ignore]
#[test]
fn test_rfind_str() {
    fn prop(a: Text, b: Short<Text>) -> TestResult {
        let a = &a.0;
        let b = &b[..];
        let truth = a.rfind(b);
        TestResult::from_bool(rfind(&a, &b) == truth)
    }
    quickcheck(prop as fn(_, _) -> _);
}

#[test]
fn test_contains_plus() {
    fn prop(a: Text, b: Short<Text>) -> TestResult {
        let a = &a.0;
        let b = &b[..];
        //let b = &b.0;
        if b.len() == 0 { return TestResult::discard() }
        let truth = a.contains(b);
        TestResult::from_bool(contains(&a, &b) == truth &&
            (!truth || b.substrings().all(|sub| contains(&a, sub))))
    }
    quickcheck(prop as fn(_, _) -> _);
}

quickcheck! {
    fn test_find_substrings_simple(a: SimpleText, b: SimpleText) -> TestResult {
        let a = &a.0;
        let b = &b[..];
        if b.len() == 0 { return TestResult::discard() }
        assert_eq!(a.find(b), find(&a, &b));
        for sub in b.substrings() {
            assert_eq!(a.find(sub), find(&a, sub), "mismatch for a={:?}, {:?}", a, sub);
        }
        TestResult::passed()
    }
}

#[ignore]
#[test]
fn test_contains_rev_plus() {
    fn prop(a: Text, b: Text) -> TestResult {
        let a = &a.0;
        let b = &b[..];
        if b.len() == 0 { return TestResult::discard() }
        let truth = a.contains(b);
        TestResult::from_bool(contains_rev(&a, &b) == truth &&
            (!truth || b.substrings().all(|sub| contains_rev(&a, sub))))
    }
    quickcheck(prop as fn(_, _) -> _);
}

#[test]
fn test_contains_substrings() {
    fn prop(s: (char, char, char, char)) -> bool {
        let mut ss = String::new();
        ss.push(s.0);
        ss.push(s.1);
        ss.push(s.2);
        ss.push(s.3);
        let a = &ss;
        for sub in a.substrings() {
            assert!(a.contains(sub));
            if !contains(a, sub) {
                return false;
            }
        }
        true
    }
    quickcheck(prop as fn(_) -> _);
}

#[ignore]
#[test]
fn test_contains_substrings_rev() {
    fn prop(s: (char, char, char, char)) -> bool {
        let mut ss = String::new();
        ss.push(s.0);
        ss.push(s.1);
        ss.push(s.2);
        ss.push(s.3);
        let a = &ss;
        for sub in a.substrings() {
            assert!(a.contains(sub));
            if !contains_rev(a, sub) {
                return false;
            }
        }
        true
    }
    quickcheck(prop as fn(_) -> _);
}

#[test]
fn test_find_period() {
    fn prop(a: SimpleText, b: Short<SimpleText>) -> TestResult {
        let a = &a.0;
        let b = &b[..];
        let pat = [b, b].concat();
        let truth = a.find(&pat);
        TestResult::from_bool(find(a, &pat) == truth)
    }
    quickcheck(prop as fn(_, _) -> _);
}

#[ignore]
#[test]
fn test_find_rev_period() {
    fn prop(a: SimpleText, b: Short<SimpleText>) -> TestResult {
        let a = &a.0;
        let b = &b[..];
        let pat = [b, b].concat();
        let truth = a.rfind(&pat);
        TestResult::from_bool(rfind(a, &pat) == truth)
    }
    quickcheck(prop as fn(_, _) -> _);
}


/*
#[test]
fn test_find_byte() {
    fn prop(v: Vec<u8>, offset: u8) -> bool {
        use twoway::set::find_byte as memchr;

        // test all pointer alignments
        let uoffset = (offset & 0xF) as usize;
        let data = if uoffset <= v.len() {
            &v[uoffset..]
        } else {
            &v[..]
        };
        for byte in 0..256u32 {
            let byte = byte as u8;
            if memchr(byte, &data) != data.iter().position(|elt| *elt == byte) {
                return false;
            }
        }
        true
    }
    quickcheck(prop as fn(_, _) -> _);
}

#[test]
fn test_rfind_byte() {
    fn prop(v: Vec<u8>, offset: u8) -> bool {
        use twoway::set::rfind_byte as memrchr;

        // test all pointer alignments
        let uoffset = (offset & 0xF) as usize;
        let data = if uoffset <= v.len() {
            &v[uoffset..]
        } else {
            &v[..]
        };
        for byte in 0..256u32 {
            let byte = byte as u8;
            if memrchr(byte, &data) != data.iter().rposition(|elt| *elt == byte) {
                return false;
            }
        }
        true
    }
    quickcheck(prop as fn(_, _) -> _);
}
*/
