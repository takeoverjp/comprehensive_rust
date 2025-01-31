fn fib(n: u32) -> u32 {
    if n < 2 {
        // ベースケース。
        n
    } else {
        // 再帰的なケース。
        fib(n - 1) + fib(n - 2)
    }
}

fn main() {
    let n = 20;
    println!("fib({n}) = {}", fib(n));
}
