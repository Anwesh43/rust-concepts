fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("number is {}", number);
    if number == 6 {
        println!("hello 5");
    } else if number % 2 == 0 {
        println!("divisible by 2");
    } else if number % 3 == 0 {
        println!("divisible by 3");
    } else if number % 5 == 0 {
        println!("divisible by 5");
    }
}
