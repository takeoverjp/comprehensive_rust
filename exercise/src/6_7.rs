/// `n` から始まるコラッツ数列の長さを決定。
fn collatz_length(n: i32) -> u32 {
  if n == 1 {
    return 1;
  }

  if n % 2 == 0 {
    return 1 + collatz_length(n / 2);
  }

  return 1 + collatz_length(3 * n + 1);
}

#[test]
fn test_collatz_length() {
    assert_eq!(collatz_length(11), 15);
}

fn main() {
    println!("Length: {}", collatz_length(11));
}
