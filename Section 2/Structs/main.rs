// Named field struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Tuple Struct
struct Coordinates(i32, i32, i32);

struct UnitStruct;

fn main() {

    let mut user = User {username: String::from("Lucas"), email: String::from("lucas@domain.com"), sign_in_count: 1, active: true};
    print_user(&mut user);
    user.username = String::from("Lucas Jorge"); // Example of owner change

    let mut user2 = build_user("lucasjorge@domain2.com".to_string(), "lucasjorge".to_string());
    print_user(&mut user2);

    let coord = Coordinates(0, 0, 0);
    print_coordinates(&coord);

    // 1..5, Range {start 1, end 5} UnitStruct later

}

fn print_coordinates(coordinates: &Coordinates) {
    println!("coordinates: ({}, {}, {})", coordinates.0, coordinates.1, coordinates.2);
}

fn build_user(email: String, username: String) -> User {
    User {email, username, sign_in_count: 1, active: true}
}

fn print_user(user: &mut User){
    if user.active {
        println!("Email: {}", user.email);
        println!("Sign in Count: {}", user.sign_in_count);
        println!("Username: {}", user.username);
    } else{
        println!("User is not active.");
    }
}
