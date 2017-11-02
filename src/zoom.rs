
type ZoomInt = u64;

fn read_zoom<'a, T>(p: &'a [T], i: usize, zoom: &mut ZoomInt) -> &'a [T] {
    if p.len() == 1 {
        &[]
    } else {
        let last_bit = *zoom & 1;
        *zoom >>= 1;
        let len = p.len() * 3 / 4;
        if last_bit != 0 {
            &p[p.len() - len ..]
        } else {
            &p[.. len]
        }
    }
}

#[test]
fn test_read_zoom() {
    let mut p: &[_] = b"aaaaaaaaaaaabbbbb";
    let mut zoom = 0b0000101;
    
    let mut i = 0;
    while p.len() != 0 {
        //println!("{:?}", p);
        p = read_zoom(p, i, &mut zoom);
    }
}

pub fn notes_on_zooming(_text: &[u8], _pattern: &[u8]) -> Option<usize>
{
    // The zooming sequence example
    // a^12 b^5
    let p = "aaaaaaaaaaaabbbbb";
    // Compressed representation is 1010000
    let zooms = [p, "aaaaaaabbbbb", "aaaaaaabb", "aaaabb", "aaaa", "aaa", "aa", "a"];

    // nonperiodic P
    // ZoomSeq(P) is (P1, P2, .., Pk) where P1 = P, |Pk| = 1
    // each Pj+1 is a prefix or suffix of Pj, length 3/4 of |Pj|
    //  (if both suffix and prefix are nonperiodic, we take the prefix)
    //
    // The jth bit is 0 iff Pj+1 is the prefix of Pj and the jth bit is 1
    // if Pj+1 is the suffix of Pj.
    //
    // ZoomSeq(P) is stored as one integer this way.
    //
    // per(P) is the shortest period of P
    // P is periodic iff per(P) <= 1/6 |P|
    //
    // quasiper(P) is per(P) iff it is periodic, else it is |P|
    //
    None
}

