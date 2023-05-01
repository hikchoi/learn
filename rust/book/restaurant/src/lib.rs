
fn deliver_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

mod front_of_house;
pub use crate::front_of_house::hosting;

mod customer {
    // idiomatic to bring the function's parent module into scope
    // ot make it clear that it's not been locally defined

    // bringing in structs, enums and other items, it is idiomatic to
    // specify the full path
    // no reason. just convention.
    // exception is when two same name has to be brought from different parent modules
    // or you can use the as keyword to provide new names
    use crate::back_of_house::Breakfast;
    use crate::back_of_house::Appetizer;
    use crate::hosting;
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    
        // order a breakfast in the summer with rye toast
        let mut meal = Breakfast::summer("Rye");
        // change our minde about what bread we'd like
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);
        // the next line won't compile if we uncomment it;
        // meal.seasonal_fruit = String::from("blueberries");
    
        let _order1 = Appetizer::Soup;
        let _order2 = Appetizer::Salad;
    
    }
}