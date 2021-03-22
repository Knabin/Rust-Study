fn main_1() {
    let mut s = String::from("Hello world");
    let word = first_word(&s);  // word == 5

    s.clear();  // s == ""

    // word는 5를 가지고 있겠지만, 5라는 값을 의미 있게 쓸 수 있는 String이 존재하지 않는다.
}   // word는 유효하지 않다.

fn first_word(s: &String) -> usize {
    // String을 요소별로 보면서 공백인지 확인해야 하므로 byte 배열로 전환한다.
    let bytes = s.as_bytes();

    // iter - 컬렉션의 각 요소를 반환하는 함수
    // enumerate - iter의 결과값을 튜플의 일부로 만들어 반환, (인덱스, 참조값)
    for (i, &item) in bytes.iter().enumerate() {
        // 공백을 찾았다면 위치를 반환한다.
        if item == b' ' {
            return i;
        }
    }

    // 공백을 찾지 못했다면 String의 길이값을 반환한다.
    s.len()
}

fn string_slice() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    // slice1 == slice2
    let slice1 = &s[0..2];
    let slice2 = &s[..2];

    // slice3 == slice4
    let len = s.len();
    let slice3 = &s[3..len];
    let slice4 = &s[3..];

    // slice5 == slice6
    let slice5 = &s[0..len];
    let slice6 = &s[..];
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}

fn main_2() {
    let mut s = String::from("hello world");
    let word = first_word_slice(&s);

    //s.clear();    //error!!

    println!("the first word is: {}", word);
}

fn first_word_sig(s: &str) -> &str {
    // do something...
    &s[..]
}

fn main() {
    let my_string = String::from("hello world");

    // first_word_sig가 'String'의 슬라이스로 동작한다.
    let word = first_word_sig(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word_sig가 스트링 리터럴의 슬라이스로 동작한다.
    let word = first_word_sig(&my_string_literal[..]);

    // 스트링 리터럴은 *또한* 스트링 슬라이스이기 때문에, 슬라이스 문법 없이도 동작한다.
    let word = first_word_sig(my_string_literal);
}