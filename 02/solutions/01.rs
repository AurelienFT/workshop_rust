fn is_two(x: i32) -> Result<i32, String> {
    if x == 2 {
        Ok(x)
    } else {
        Err("Parameter is not a 2.".to_string())
    }
}

fn main() {
    println!("{:#?}", is_two(23));
}