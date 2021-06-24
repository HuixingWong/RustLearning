
pub fn function_test() {

    // let x = 5;
    // let y = {
    //     let x = 3;
    //     x + 1 ;   这里不可以加分号 如果作为返回值的话
    // };
    // println!("{}", y);


    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

}