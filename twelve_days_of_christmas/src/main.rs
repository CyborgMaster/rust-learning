fn main() {
    for day in 1..=12 {
        verse(day)
    }
}

fn verse(day: u8) {
    println!(
        "On the {day}{} day of Christmas my true love gave to me:",
        number_ending(day.into())
    );
    for gift_count in (1..=day).rev() {
        if gift_count == 1 {
            if day == 1 {
                print!("A ");
            } else {
                print!("And a ");
            }
        } else {
            print!("{gift_count} ");
        }
        print!("{}", gift(gift_count));
        if gift_count == 1 {
            println!(".");
        } else {
            println!(",");
        }
    }
    println!("");
}

fn gift(day: u8) -> &'static str {
    match day {
        1 => "partridge in a pear tree",
        2 => "turtle doves",
        3 => "french hens",
        4 => "calling birds",
        5 => "GOLDEN RINGS",
        6 => "geese a laying",
        7 => "swans a swimming",
        8 => "maids a milking",
        10 => "ladies dancing",
        9 => "lords a leaping",
        11 => "pipers piping",
        12 => "drummers drumming",
        _ => panic!("bad gift day!"),
    }
}

fn number_ending(num: u32) -> &'static str {
    let num = num % 100;
    if num > 10 && num < 20 {
        return "th";
    }
    let last = num % 10;
    if last == 1 {
        "st"
    } else if last == 2 {
        "nd"
    } else if last == 3 {
        "rd"
    } else {
        "th"
    }
}
