// ===== 변수의 스코프 =====
fn scope() {
    // s는 선언 전이기 때문에 유효하지 않다.
    let s = "hello";    // s는 이 지점부터 유효하다.
    
    // s를 이용한 동작을 구현한다.
}   // end of scope. s는 더 이상 유효하지 않다.
// ========================

// ===== String 타입 =====
fn string_type() {
    let s = String::from("hello");

    let mut ms = String::from("hello");
    ms.push_str(", world!");    // push_str() - 해당 스트링 리터럴을 스트링에 붙여 준다.alloc
    println!("{}", ms);
}
// ========================

// ====== 이동(move) ======
fn variables_move() {
    // 두 개의 변수를 사용할 수 있다. 복사본을 만들어 변수 y에 바인딩한다.
    let x = 5;
    let y = x;

    // 한 개의 변수만 사용할 수 있다. s1이 s2로 이동(move)된다.
    let s1 = String::from("hello");
    let s2 = s1;

    //println!("{}, world!", s1);   // error!
}
// ========================

// ====== 클론(clone) ======
fn variables_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
// ========================

// ====== 복사(copy) ======
fn stack_copy() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}
// ========================

// ===== 소유권과 함수 =====
fn main_1() {
    let s = String::from("hello");  // s가 스코프 안으로 들어왔다.
    
    takes_ownership(s);             // s의 값이 함수 안으로 이동했다...
                                    // ... 그리고 이제 더 이상 유효하지 않다.
    let x = 5;                      // x가 스코프 안으로 들어왔다.

    makes_copy(x);                  // x의 값이 함수 안으로 이동했다.
                                    // i32는 copy가 되므로, 이후에도 x를 계속 사용해도 된다.

    //println!("{}", s);              // error!
    println!("{}", x);
}   // x는 스코프 밖으로 나가고, 그 후 s도 나간다.
    // 그러나 s는 이미 이동되었으므로 별다른 일이 발생하지 않는다.

fn takes_ownership(some_string: String) {   // some_string이 스코프 안으로 들어왔다.
    println!("{}", some_string);
}   // some_string이 스코프 밖으로 벗어났고, 'drop'이 호출된다. some_string의 메모리 해제.

fn makes_copy(some_integer: i32) {  // some_integer가 스코프 안으로 들어왔다.
    println!("{}", some_integer);
}   // some_integer가 스코프 밖으로 벗어났다. 별다른 일이 발생하지 않는다.
// ========================

// ==== 반환값과 스코프 ====
fn main() {
    let s1 = gives_ownership();         // gives_ownership은 반환값을 s1에게 이동시킨다.
    let s2 = String::from("hello");     // s2가 스코프 안으로 들어왔다.
    let s3 = takes_and_gives_back(s2);  // s2는 takes_and_gives_back 안으로 이동되었고, 이 함수가 반환값을 s3으로도 이동시켰다.

    let s4 = String::from("hello");
    let (s5, len) = calculate_length(s4);

    println!("The length of '{}' is {}.", s5, len);
}   // 여기서 s3은 스코프 밖으로 벗어났으며, drop이 호출된다.
    // s2는 스코프 밖으로 벗어났지만 이동되었으므로 아무 일도 일어나지 않는다.
    // s1은 스코프 밖으로 벗어나서 drop이 호출된다.

fn gives_ownership() -> String {        // gives_ownership 함수가 반환값을 호출한 쪽으로 이동시킨다.
    let some_string = String::from("hello");    // some_string이 스코프 안에 들어왔다.

    some_string                         // some_string이 반환되고, 호출한 쪽의 함수로 이동된다.
}

// takes_and_gives_back 함수는 String을 하나 받아서, 다른 하나를 반환한다.
fn takes_and_gives_back(a_string: String) -> String {   // a_string이 스코프 안으로 들어왔다.
    a_string                            // a_string은 반환되고, 호출한 쪽의 함수로 이동된다.
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();   // len() - 문자열의 길이를 반환한다.

    (s, length)
}
// ========================