fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    //try_change(&s1);

    let mut s2 = String::from("hello");
    change(&mut s2);

    let r1 = &mut s2;
    //let r2 = &mut s2;   // error!
    //println!("{}, {}", r1, r2);
}

fn calculate_length(s: &String) -> usize {  // s는 String의 참조자이다.
    s.len()
}   // 여기서 s는 스코프 밖으로 벗어났다. 
    // 하지만 가리키고 있는 값에 대한 소유권이 없어, 아무런 일도 발생하지 않는다.

fn try_change(some_string: &String) {
    //some_string.push_str(", world");  // error!
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn scope() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    }   // 여기서 r1은 스코프 밖으로 벗어났으므로,

    let r2 = &mut s;    // 새로운 참조자를 만들 수 있다.
}

fn mut_ref() {
    let mut s = String::from("hello");

    let r1 = &s;        // 문제 없음
    let r2 = &s;        // 문제 없음
    //let r3 = &mut s;    // error!!

    //println!("{}, {}, and {}", r1, r2, r3);
}
/*
fn dangle() -> &String {    // dangle은 String의 참조자를 반환한다.
    let s = String::from("hello");  // s는 새로운 String.

    &s; // String s의 참조자를 반환한다.
}   // 여기서 s는 스코프를 벗어나서 메모리가 해제된다. 위험!
*/

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}