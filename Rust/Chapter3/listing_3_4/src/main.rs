#![allow(dead_code)]

#[derive(Debug)]
enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday
}

fn main() {
    let dow = DayOfWeek::Thursday;
    println!("Hello, day of week {:?}!", dow);
}
