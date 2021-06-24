pub fn owner_ship() {
    let s = String::from("hello");
    // test_1()
    test_3()
}

fn test_1() {
    let s = String::from("Hello");
    let s2 = s;

    // println!("{}", s) s已经无效
    println!("{}", s2);

    let s3 = s2.clone(); // 深拷贝
    println!("{}", s2);
    println!("{}", s3);
}

fn test_2() {
    let str = String::from("fuck u");
    let result = cal_length(str);
    println!("{}", result);
    //不能这么使用， 当把str 传递出去之后，
    // 相应的ownership也被传递出去了就不能继续被使用了
    // println!("{}",str);
}

fn cal_length(str: String) -> usize {
    str.len()
}


fn test_3() {
    let str = String::from("fuck u");
    let result = cal_length2(&str);
    println!("{}", result);
    //这个地方就可以引用了， 因为传递出去的只是引用， ownership还在
    println!("{}", str);
}

fn cal_length2(str: &String) -> usize {
    str.len()
}



