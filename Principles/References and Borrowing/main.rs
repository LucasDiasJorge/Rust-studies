fn main() {

    let mut s = String::from("hello");
    change_string(&mut s); // Pass a mutable reference to allow modification
    println!("{}", s); // Since we used a mutable reference, "s" is still accessible here

}

fn change_string(some_string: &mut String) {
    some_string.push_str(", world"); // Modify the string by appending ", world"
}
