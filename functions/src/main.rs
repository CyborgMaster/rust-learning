fn main() {
    println!("main");

    let y = a_function(10);
    println!("returned {y}");
}

fn a_function(x: i32) -> i32 {
    println!("a_function {x}");
    x * 2
}
