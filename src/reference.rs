
pub fn test_reference() {

    let mut str = String::from("Hello");
    test_borrow(&mut str);
    println!("{}", str)
}

fn test_borrow(str: &mut String) {

    str.push_str( ", world");

}