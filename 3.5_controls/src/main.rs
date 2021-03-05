fn main() {
    // 조건문
    let number = 3;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let con_number = if condition {
        5
    } else {
        //"six" // error!! 값 타입을 통일해야 한다.
        6
    };

    println!("The value of con_number is: {}", con_number);


    // loop 반복문
    // loop {
    //     println!("again");
    // }

    // while 반복문
    let mut while_num = 3;

    while while_num != 0 {
        println!("{}!", while_num);

        while_num = while_num - 1;
    }

    println!("while문 탈출!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // for with Range
    for number in (1..4).rev() {    // 범위에 4를 포함시키려면 1..=4처럼 작성하면 된다.
        println!("{}!", number);
    }
    println!("for문 탈출!");
}
