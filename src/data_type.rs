

pub fn data_type() {

    let x = 52u32;
    println!("{}", x);

    //奇怪的类型 ，，，，
    let tup: (i32, f64, u8, String) = (500, 6.4, 1, "h哈哈".to_string());
    let (x, y, z, f) = tup;
    println!("The value of y is: {}", y);
    println!("The value of f is: {}", f);

    //array
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    //创建一个有5个三的数组
    let a = [3;5];
    for x in a.iter() {
        println!("{}", x)
    }


}