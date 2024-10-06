fn main(){

    /*
    
    let r;

    {
        let x = 5;
        r = &x;
    } // x is dropped here

    println!("r: {}", r); // r refernce was x, x lifetime is done
    
    */

    let string1 = String::from("long string is long");
    let string2 = String::from("short");  

    let result = longest(&string1, &string2);

    println!("The longest string is: {}", result);

}

fn invalid_reference() -> &str {
    let string = String::from("Hello");
    &string
} // Error: Invalid reference, because string was dropped

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}