fn main() {

    let mut val: Vec<i8> = Vec::new();
    val.push(1);
    val.push(3);
    val.push(5);
    val.push(7);

    function_check(&mut val);

    val.push(15);

    for i in 0..val.len() {
        println!("{}", val[i]);
    }

    let num = 10;
    add_two(num); // Copy concept, this is why no reference here.
    println!("{:?}",num);

}

fn function_check(val: &mut Vec<i8>) -> bool{
    if val[0] == 1 {
        return true;
    }
    return false;
}

fn add_two(val: i8){
    println!("val + 2 = {}", val + 2); // We just made a copy
}