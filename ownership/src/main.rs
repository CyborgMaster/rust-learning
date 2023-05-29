fn main() {
    let mut s1 = String::from("hello");

    println!("s1 {s1}");

    let l = calculate_length(&s1);

    // s1.push_str(", hi");
    let s2 = &mut s1;
    let s3 = &s1;

    // println!("s1: {s1}, len: {l}");
    println!("s2: {s2}");
    println!("s3: {s3}");
}

fn take_ownership(s: String) {
    println!("owned: {s}");
}

fn calculate_length(s: &String) -> usize {
    let len = s.len();
    len
}

fn change(s: &mut String) {
    s.push_str(", there");
}

fn give_ref() -> &String {
    let s = String::from("from_ref");
    &s
}
