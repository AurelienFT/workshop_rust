use std::fs;

fn main() {
    match fs::read_to_string("04.rs") {
        Err(why) => panic!("{:?}", why),
        Ok(foo) => println!("{}", foo),
    }
}
