fn main() {
    //loop example
    let mut number : i32 = 3;
    loop {
        if number == 0 {
            break
        }
        println!("hello loop {}", number);
        number = number - 1;
    }

    //while loop
    let mut i : i32 = 0;
    while i != 3 {
        println!("hello while {}", i);
        i = i + 1;
    }

    //for loop
    for num in 0..3 {
        println!("hello for {}", num);
    }

    //iterating over an array
    let arr = [10, 11, 12, 13, 14];
    for a in arr.iter() {
        println!("element in arr is {}", a);
    }
}
