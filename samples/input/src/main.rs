use std::io;

fn convert_to_int(my_string: &String) -> i32 {
    //let myNumber: i32 = myString.trim().parse<i32>.unwrap();
    let my_number: i32 = my_string.trim().parse().unwrap();
    my_number
}

fn main() {
    let mut input = String::new();
    println!("Enter a number: \n");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line 1");

    let mut input2 = String::new();
    println!("Enter a number 2: \n");
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read line");

    if convert_to_int(&input) > convert_to_int(&input2) {
        println!("{} is greater than {}", input, input2);
    } else {
        println!("{} is greater than {}", input2, input);
    }
}
