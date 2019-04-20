fn take_ownership(str1 : String) {
    println!("str1 = {}", str1);
}
fn main() {
    let s1 = String::from("Hello");
    let s2 = s1;
    println!("{}", s2);
    let s3 = String::from("World");
    let s4 = s3.clone();
    println!("s3 = {}, s4 = {}", s3, s4);
    let s5 = String::from("Won't work after move");
    take_ownership(s5);
}
