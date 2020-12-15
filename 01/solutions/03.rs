fn main() {
    let mut s1 = String::from("Hello ");
    let s2 = String::from("world");

    s1.push_str(&s2);
    println!("s1 is: {}", s1);
}