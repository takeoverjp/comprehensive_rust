use std::cmp::Ordering;

fn min<T: Ord>(a: T, b: T) -> T {
  match a.cmp(&b) {
    Ordering::Less => a,
    Ordering::Greater => b,
    Ordering::Equal => a,
  }
}

fn main() {
    assert_eq!(min(0, 10), 0);
    assert_eq!(min(0, 0), 0);
    assert_eq!(min(500, 123), 123);

    assert_eq!(min('a', 'z'), 'a');
    assert_eq!(min('7', '1'), '1');

    assert_eq!(min("hello", "goodbye"), "goodbye");
    assert_eq!(min("bat", "armadillo"), "armadillo");
}
