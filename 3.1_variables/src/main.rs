fn main1() {
    //let x = 5;    //error!!
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // OUTPUT : The value of x is: 5
    //          The value of x is: 6
}

fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("The value of x is: {}", x);
    
    // OUTPUT : The value of x is: 12
}