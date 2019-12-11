#![allow(dead_code)]

mod dow {
    #[derive(Debug)]
    pub enum DayOfWeek {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday
    }

    impl DayOfWeek {
        pub fn day_of_week(d: u8) -> Option<DayOfWeek> {
            match d {
                1 => Some(DayOfWeek::Monday),
                2 => Some(DayOfWeek::Tuesday),
                3 => Some(DayOfWeek::Wednesday),
                4 => Some(DayOfWeek::Thursday),
                5 => Some(DayOfWeek::Friday),
                6 => Some(DayOfWeek::Saturday),
                7 => Some(DayOfWeek::Sunday),
                _ => None
            }
        }

        pub fn to_string(&self) -> String {
            match self {
                DayOfWeek::Monday    => String::from("Monday"),
                DayOfWeek::Tuesday   => String::from("Tuesday"),
                DayOfWeek::Wednesday => String::from("Wednesday"),
                DayOfWeek::Thursday  => String::from("Thursday"),
                DayOfWeek::Friday    => String::from("Friday"),
                DayOfWeek::Saturday  => String::from("Saturday"),
                DayOfWeek::Sunday    => String::from("Sunday"),
            }
        }
    }
}

fn main() {
    use dow::*;

    let dow = DayOfWeek::Thursday;
    println!("Hello, day of week {}!", dow.to_string());

    let dow2 = DayOfWeek::day_of_week(2).unwrap();
    println!("Hello, day of week 2 {}!", dow2.to_string());

    match DayOfWeek::day_of_week(8) {
        Some(dow) => println!("Hello, day of week 3 {}!", dow.to_string()),
        None      => println!("Hello, no day of week!")
    }
}
