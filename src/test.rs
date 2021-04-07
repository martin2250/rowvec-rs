use super::*;

#[test]
fn common_ops() {
    let mut r = RowVec::new(2);
    for c in 0..10 {
        r.push(&[2 * c, 2 * c + 1]);
    }
    assert!(r[1][0] == 2);
    assert!(r[1][1] == 3);
    assert!(r[(1, 0)] == 2);
    assert!(r[(1, 1)] == 3);
    let s = r.slice().range(1, 2);
    assert!(s[0][0] == 2);
    assert!(s[0][1] == 3);
    r.remove_range(0..1);
    assert!(r[0][0] == 2);
    assert!(r[0][1] == 3);
}