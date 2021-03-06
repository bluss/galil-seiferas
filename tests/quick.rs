
extern crate galil_seiferas;

#[macro_use] extern crate quickcheck;
extern crate odds;
#[macro_use] extern crate macro_attr;
#[macro_use] extern crate newtype_derive;

use std::ops::Deref;

use galil_seiferas::gs_find;

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

// The letters of the fib alphabet are a and b
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

// Write a fuzz dictionary -- need to use --ignored to run these
quickcheck! {
    #[ignore]
    fn generate_dict_fibwords(n: usize) -> () {
        let word = FibWord::new(n % 10);
        writeln_to_file(format!("{:?}", word.as_str()), "dict-fibwords");
    }

    #[ignore]
    fn generate_dict_lsys(words: Vec<usize>, repeats: Vec<usize>) -> () {
        let mut s = String::new();
        for (w, repeat) in words.into_iter().zip(repeats) {
            for _ in 0..((repeat % 5) + 1) {
                s.push_str(&LSys1::new(w % 6));
            }
        }

        if !s.is_empty() {
            writeln_to_file(format!("\"{}\"", s), "dict-lsys");
        }
    }
}

fn writeln_to_file(s: String, name: &str) {
    use std::io::Write;
    use std::fs::OpenOptions;
    let mut f = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(name).unwrap();
    writeln!(f, "{}", s).unwrap();
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

macro_attr! {
    #[derive(Clone, Debug, NewtypeDeref!, NewtypeDerefMut!)]
    struct LSys1(String);
}

// L-System where we rewrite a string in generations
/// Starting state
const LSTART: &str = "0";

/// Rewrite rules
const LRULES: &[(u8, &str)] = &[
    (b'0', "100"),
    (b'1', "11"),
];

impl LSys1 {
    fn new(n: usize) -> Self {
        let mut f = LSys1(String::from(LSTART));
        let mut g = LSys1(String::new());
        for _ in 0..n {
            f.next_into(&mut g);
            std::mem::swap(&mut f, &mut g);
        }
        f
    }

    fn next_into(&self, s: &mut String) {
        s.clear();
        'bytes: for byte in self.bytes() {
            for &(rule, replace) in LRULES {
                if byte == rule {
                    s.push_str(replace);
                    continue 'bytes;
                }
            }
            // else it is a constant
            s.push(byte as char);
        }
    }
}

pub fn contains(hay: &str, n: &str) -> bool {
    gs_find(hay.as_bytes(), n.as_bytes()).is_some()
}

pub fn find(hay: &str, n: &str) -> Option<usize> {
    gs_find(hay.as_bytes(), n.as_bytes())
}

quickcheck! {
    fn test_contains(a: Text, b: Short<Text>) -> bool {
        let a = &a.0;
        let b = &b[..];
        let truth = a.contains(b);
        contains(&a, &b) == truth
    }

    fn test_find_regular_str(a: String, b: Short<String>) -> bool {
        let a = &a[..];
        let b = &b[..];
        let truth = a.find(b);
        find(&a, &b) == truth
    }

    fn test_find_short(a: Text, b: Short<Text>) -> bool {
        let a = &a.0;
        let b = &b[..];
        let truth = a.find(b);
        find(&a, &b) == truth
    }

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

    fn test_contains_plus(a: Text, b: Short<Text>) -> TestResult {
        let a = &a.0;
        let b = &b[..];
        //let b = &b.0;
        if b.len() == 0 { return TestResult::discard() }
        let truth = a.contains(b);
        TestResult::from_bool(contains(&a, &b) == truth &&
            (!truth || b.substrings().all(|sub| contains(&a, sub))))
    }

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

    fn test_contains_substrings(s: Short<String>) -> () {
        let a = &s[..];
        for sub in a.substrings() {
            assert!(a.contains(sub));
        }
    }

    fn test_find_period(a: SimpleText, b: Short<SimpleText>) -> bool {
        let mut a = a.0;
        let b = &b[..];
        if 3 * b.len() > a.len() {
            a = a.repeat(6 * b.len() / (a.len() + 1));
        }
        let a = &a;
        let pat = b.repeat(3);
        let truth = a.find(&pat);
        find(a, &pat) == truth
    }
}
