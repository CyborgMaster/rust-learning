use lazy_static::lazy_static;
use onig::Regex;

fn pig_latin(s: &str) -> String {
    lazy_static! {
        // Split at word boundaries, but don't count single quotes as a boundary.
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

    let first_vowel_pos = word.find(|c| is_vowel(c)).unwrap();
    if first_vowel_pos == 0 {
        return word.to_string() + "-hay";
    }
    let (mut first, mut rest) = word.split_at(first_vowel_pos);

    let first_lowercase: String;
    let rest_uppercase: String;
    if first_char.is_uppercase() {
        first_lowercase = first.to_lowercase();
        first = &first_lowercase;
        rest_uppercase = upcase_first_char(rest);
        rest = &rest_uppercase;
    }

    format!("{}-{}ay", rest, first)
}

fn is_vowel(c: char) -> bool {
    match c.to_ascii_lowercase() {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
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
