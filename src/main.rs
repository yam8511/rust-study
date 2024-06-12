mod person;

use person::Person;

fn main() {
    let mut p = Person::new("zuolar".to_string(), 30); // instance
    p.change_name("ryan".to_string()); // call
    p.say_hi();
    println!("{}", p);
}
