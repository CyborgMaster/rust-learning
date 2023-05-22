fn main() {
    println!("fib: {}", fibonacci(50));
}

fn fibonacci(n: u32) -> u64 {
    let mut x0 = 1;
    let mut x1 = 1;

    for _ in 0..n {
        println!("{x1}");
        (x0, x1) = (x1, x0 + x1);
    }
    x1
}
