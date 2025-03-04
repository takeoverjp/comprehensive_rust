/// `values`において、`offset`だけ離れた要素間の差を計算します。
/// なお、`values`の末尾要素の次は先頭へ戻ることとします。
///
/// 結果の要素 `n` は `values[(n+offset)%len] - values[n]` です。
fn offset_differences(offset: usize, values: Vec<i32>) -> Vec<i32> {
  let len = values.len();
  if len == 0 {
    return values;
  }
  let offset = offset % len;
  let mut buf = Vec::with_capacity(len);
  buf.extend_from_slice(&values[offset..]);
  buf.extend_from_slice(&values[..offset]);
  for (idx, v) in values.iter().enumerate() {
    buf[idx] -= *v;
  }
  buf
}

#[test]
fn test_offset_one() {
  assert_eq!(offset_differences(1, vec![1, 3, 5, 7]), vec![2, 2, 2, -6]);
  assert_eq!(offset_differences(1, vec![1, 3, 5]), vec![2, 2, -4]);
  assert_eq!(offset_differences(1, vec![1, 3]), vec![2, -2]);
}

#[test]
fn test_larger_offsets() {
  assert_eq!(offset_differences(2, vec![1, 3, 5, 7]), vec![4, 4, -4, -4]);
  assert_eq!(offset_differences(3, vec![1, 3, 5, 7]), vec![6, -2, -2, -2]);
  assert_eq!(offset_differences(4, vec![1, 3, 5, 7]), vec![0, 0, 0, 0]);
  assert_eq!(offset_differences(5, vec![1, 3, 5, 7]), vec![2, 2, 2, -6]);
}

#[test]
fn test_degenerate_cases() {
  assert_eq!(offset_differences(1, vec![0]), vec![0]);
  assert_eq!(offset_differences(1, vec![1]), vec![0]);
  let empty: Vec<i32> = vec![];
  assert_eq!(offset_differences(1, empty), vec![]);
}
