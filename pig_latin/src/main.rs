use lazy_static::lazy_static;
use onig::Regex;

fn pig_latin(s: &str) -> String {
    lazy_static! {
        static ref WORDS: Regex = Regex::new(r"(?<!')\b(?!')").unwrap();
    }
    WORDS.split(s).map(pig_word).collect()
}

fn pig_word(word: &str) -> String {
    if word.len() == 0 {
        return "".to_string();
    }

    let first_char = word.chars().next().unwrap();
    if !first_char.is_alphabetic() {
        return word.to_string();
    }

    // dbg!(word);

    let mut first = first_char.to_string();
    let mut rest = &word[first_char.len_utf8()..];

    let upcase_rest: String;
    match first_char.to_ascii_lowercase() {
        'a' | 'e' | 'i' | 'o' | 'u' => {
            first = "h".to_string();
            rest = word;
        }
        _ if first_char.is_uppercase() => {
            first = first_char.to_lowercase().collect();
            upcase_rest = upcase_first_char(rest);
            rest = &upcase_rest;
        }
        _ => (),
    }

    format!("{}-{}ay", rest, first)
}

fn upcase_first_char(s: &str) -> String {
    let mut c = s.chars();
    let f = c.next().unwrap();
    f.to_uppercase().collect::<String>() + c.as_str()
}

fn main() {
    let s = pig_latin("Jeremy loves apples!  However, not bananas. Here's a string.");
    println!("{}", s);
}
