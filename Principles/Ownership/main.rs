fn main(){

    // Stack and Heap
    let _var = 1; // created on the stack // "_" because unused variable intentional
    let mut s = "hello".to_string(); // created on the heap
    s.push_str(",world");

    // Move
    let x = vec!["lucas".to_string()]; // init x with "lucas" value
    let y = x; // move (concept) x value to y
    let z = y; // move (concept) y value to z

    println!("{:?}",z); // if you try to print x or y, errors are expected, once you move the value between the owners x->y->z

    // Clone
    let x = vec!["lucas".to_string()]; // init x with "lucas" value
    let y = x.clone(); // clone (concept) x value to y
    let z = y.clone(); // clone (concept) y value to z

    println!("{:?}",x);
    println!("{:?}",y);
    println!("{:?}",z); // Now, without errors. [Expensive operation]

    // Copy
    let x = 1;
    let y = x; // Some types performer a copy instead a move
    println!("x = {}, y = {}",x,y); // output: 1 1

    // More Moves
    let s = String::from("takes"); // Create a variable with string "takes"
    takes_ownership(s); // give ownership to the function
    // println!("s = {}",s); // proof of error

    let num = 1;
    make_copy(num); // Some types performer a copy instead a move
    println!("num = {}",num); // proof of it

    let s2 = give_ownership(); // Init s2 with give_ownership return value
    println!("s2 = {}",s2); // proof of it

    let s3 = take_and_give(s2); //
    // println!("s2 after take_and_give(s2) = {}",s2); s2 owner is s3 now
    println!("s3 = {}",s3);

    if true {
        let _s4 = s3; // also in control statements or loops, ownership works equal
    }
    // println!("s3 = {}",s3); // error

}
// Stack and Heap
// var is dropped, stack pop var
// s is dropped

fn takes_ownership(some_string: String) {
    let strin = some_string;
    println!("strin = {}",strin);
}

fn make_copy(one: i32){
    println!("one = {}",one);
}

fn give_ownership() -> String {
    "give ownership".to_string()
}

fn take_and_give(str3: String) -> String {
    str3 // take and give back
}