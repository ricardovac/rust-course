mod enums {
    pub fn example() {
        enum Day {
            Monday,
            Tuesday,
            Wednesday,
            Thursday,
            Friday,
            Saturday,
            Sunday,
        }

        impl Day {
            fn is_weekend(&self) -> bool {
                match self {
                    Day::Saturday | Day::Sunday => true,
                    _ => false,
                }
            }

            fn is_monday(&self) -> bool {
                match self {
                    Day::Monday => true,
                    _ => false,
                }
            }
        }
        let today: Day = Day::Monday;
        match today {
            Day::Monday => println!("today is monday"),
            Day::Tuesday => println!("fuck Tuesday"),
            Day::Wednesday => println!("fuck Wednesday"),
            Day::Thursday => println!("fuck Thursday"),
            Day::Friday => println!("fuck Friday"),
            Day::Saturday => println!("fuck Saturday"),
            Day::Sunday => println!("fuck Sunday"),
        }

        println!("Is today the weekend {}", today.is_weekend());
        println!("today is monday? {}", today.is_monday());
    }
}

pub fn enums_example() {
    crate::enums::enums::example();
}
