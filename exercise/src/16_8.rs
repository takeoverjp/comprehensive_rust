use std::collections::HashMap;
use std::hash::Hash;
/// カウンタは型 T の各値が確認された回数をカウントします。
struct Counter<T> {
    values: HashMap<T, u64>,
}

impl<T: Eq + Hash> Counter<T> {
    /// 新しいカウンタを作成します。
    fn new() -> Self {
        Counter {
            values: HashMap::new(),
        }
    }

    /// 指定された値の発生をカウントします。
    fn count(&mut self, value: T) {
        let count = self.values.entry(value).or_insert(0);
        *count += 1;
    }

    /// 指定された値が確認された回数を返します。
    fn times_seen(&self, value: T) -> u64 {
        self.values.get(&value).copied().unwrap_or_default()
    }
}

fn main() {
    let mut ctr = Counter::new();
    ctr.count(13);
    ctr.count(14);
    ctr.count(16);
    ctr.count(14);
    ctr.count(14);
    ctr.count(11);

    for i in 10..20 {
        println!("saw {} values equal to {}", ctr.times_seen(i), i);
    }

    let mut strctr = Counter::new();
    strctr.count("apple");
    strctr.count("orange");
    strctr.count("apple");
    println!("got {} apples", strctr.times_seen("apple"));
}
