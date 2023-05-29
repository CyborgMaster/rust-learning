fn main() {
    let s = String::from("my first name");

    let first = first_word(&s);
    println!("s: {s}; first_word: {first}");
}

fn first_word(s: &String) -> &str {
    let b = s.as_bytes();
    for (i, &item) in b.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    s
}
