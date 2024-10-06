struct MyString<'a> {
    text: &'a str
}

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

    // Other example
    let str1 = String::from("This is my string"); 
    let x = MyString{text: str1.as_str()}; // if str1 get dropped, MyString needs to "know", otherwise, x (MyString) has no reference for text

    let s: &'static str = "I have a static lifetime";
    
}

/*/
fn invalid_reference() -> &str {
    let string = String::from("Hello");
    &string
} // Error: Invalid reference, because string was dropped
*/

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}