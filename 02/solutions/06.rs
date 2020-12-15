fn main() {
    match is_four(2) {
      Err(why) => panic!("{:?}", why),
      Ok(x) => println!("x is {}", x),
    }
  }
  
  fn is_four(x: i32) -> Result<i32, String> {
    match is_two(x / 2) {
      Err(_) => Err("Variable not equal to 4".to_string()),
      Ok(x) => Ok(x * 2),
    }
  }
  
  fn is_two(x: i32) -> Result<i32, String> {
    match x {
      2 => Ok(x),
      _ => Err("Variable not equal to 2".to_string()),
    }
  }
  
  #[cfg(test)]
  mod tests {
    use super::*;
  
    #[test]
    fn is_four_four() -> Result<(), String> {
      let x = 4;
      assert_eq!(is_four(x), Ok(x));
      Ok(())
    }
  
    #[test]
    fn is_four_two() -> Result<(), String> {
      let x = 2;
      assert_eq!(is_four(x), Err("Variable not equal to 4".to_string()));
      Ok(())
    }
  
    #[test]
    fn is_four_42() -> Result<(), String> {
      let x = 42;
      assert_eq!(is_four(x), Err("Variable not equal to 4".to_string()));
      Ok(())
    }
  }