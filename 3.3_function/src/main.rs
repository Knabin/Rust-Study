fn main() {
    println!("Hello, world!");

    another_function();
    another_function_param(5);
    another_function_params(10, 11);

    let y = 6;              // 구문
    //let x = (let z = 6);  // error!! 구문은 값을 반환하지 않는다.

    let a = 5;
    let b = {
        let a = 3;
        a + 1
    };

    println!("The value of y is: {}", b);

    let f = five();

    let c = plus_one(10);
    println!("The value of c is: {}", c);
}

fn another_function() {
    println!("Another function.");
}

fn another_function_param(x: i32) {
    println!("The value of x is: {}", x);
}

fn another_function_params(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}