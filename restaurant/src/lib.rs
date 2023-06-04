mod front_of_house;

mod back_of_house {
    fn cook_order() {}
    fn fix_order() {
        cook_order();
        super::front_of_house::serving::serve_order();
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub mod customer {
    use crate::back_of_house;
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        let mut meal = back_of_house::Breakfast::summer("rye");
        meal.toast = String::from("wheat");
        println!("I'd like {} toast.", meal.toast);
        // meal.seasonal_fruit = String::from("blueberries");

        hosting::add_to_waitlist();
    }
}

// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
