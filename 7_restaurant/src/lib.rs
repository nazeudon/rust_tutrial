#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}

//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}

//         fn serve_order() {}

//         fn take_payment() {}

//         mod back_of_house {
//             fn fix_incorrect_order() {
//                 cook_order();
//                 super::serve_order();
//             }

//             fn cook_order() {}
//         }
//     }
// }
// 中身をfront_of_houseに移した
mod front_of_house;

mod back_of_house {
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

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    //絶対パス
    crate::front_of_house::hosting::add_to_waitlist();
    //相対パス
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // enumは要素が全てpubになる
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    //useを使えば, 全てのパスを書かなくてもよくなる
    hosting::add_to_waitlist();
}
