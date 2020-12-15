fn is_two(x: i32) -> Result<i32, String> {
    if x == 2 {
        Ok(x)
    } else {
        Err("Parameter is not a 2.".to_string())
    }
}

fn is_four(x: i32) -> Result<i32, String> {
    Ok(is_two(x / 2)? * 2) 
}

fn main() {
    println!("{:#?}", is_four(4));
}