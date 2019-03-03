fn main() {
    let a = 23;
    println!("value of a is {}", a);
    //a = 24; //not possible
    let mut b = 20;
    println!("value of b is {}", b);
    b = 36;
    println!("value of b is {}", b);
    let a = a + 10;
    println!("new val of a is {}", a); //shadowing concepts
}
