fn main() {
    let mut s : String = String::from("hello");
    let l = calculate_len(&s);
    println!("len of {} is {}", s, l);
    change(&mut s);
    println!("new s is {}", s);
}

fn calculate_len(str : &String) -> usize {
    str.len()
}

fn change(str : &mut String) {
    str.push_str(" world");
}
