fn is_two(x: i32) -> Result<i32, String> {
    if x == 2 {
        Ok(x)
    } else {
        Err("Parameter is not a 2.".to_string())
    }
}

fn is_four(x: i32) -> Result<i32, String> {
    match is_two(x / 2) {
        Ok(x) => Ok(x * 2),
        Err(_) => Err("Parameter is not a 4.".to_string())
    }
}

fn main() {
    println!("{:#?}", is_four(6));
}