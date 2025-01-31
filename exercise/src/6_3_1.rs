fn main() {
    let s = [[5, 6, 7], [8, 9, 10], [11, 12, 13]];
    let mut elements_searched = 0;
    let target_value = 10;
    'outer: for i in 0..=2 {
        for j in 0..=2 {
            elements_searched += 1;
            if s[i][j] == target_value {
                println!("Found at s[{i}][{j}]");
                break 'outer;
            }
        }
    }
}
