fn add(x : i32, y : i32) -> i32 {
    return x + y
}

fn print_something() {
    println!("something")
}

fn print_product(a : i32, b : i32) {
    println!("product of {}, {} is {}", a, b, a * b)
}

fn get_division_of_10_2() -> i32 {
    return 10 / 2
}

fn use_block_to_get_sum(a : i32, b : i32) -> i32 {
    let y = {
        let x = a;
        x + b
    };
    return y;
}

fn main() {
    println!("addition of 10 and 20 is {}", add(10, 20));
    print_something();
    print_product(30, 15);
    println!("division of 10 and 2 is {}", get_division_of_10_2());
    println!("using block to get sum of 10, 5: {}", use_block_to_get_sum(10, 5));
}
