fn main() {
    let t : i32 = -5;
    println!("the number t is {}", t);
    let k : f64 = 23.05;
    println!("the float number k is {}", k);
    let tup : (i32, f64, i32) = (10, 11.3, 20);
    println!("item1 is {}", tup.0);
    println!("item2 is {}", tup.1);
    println!("item3 is {}", tup.2);
    let (x, y, z) = tup;
    println!("x:{}, y:{}, z:{}", x, y, z);
    let arr = [1, 2, 3, 4, 5];
    println!("arr[0]:{}, arr[1]:{}, arr[2]:{}, arr[3]:{}, arr[4]:{}", arr[0], arr[1], arr[2], arr[3], arr[4])
}
