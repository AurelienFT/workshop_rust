fn main() {
    let x = 4;

    if x < 2 {
      println!("Échec");
    } else {
      println!("Réussi");
    }

    let mut test: String = String::new();
    
    let result = loop {
        test.push('a');
    
        if test == "aaaa".to_string() {
            break "Salut";
        }
    };
    
    println!("{}", result);
}
