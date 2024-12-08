fn main() {
    let s = String::from("hello_devil");

    let s1 = &s[0..5];
    let s2 = &s[5..10];

    println!("-------the value of s1={s1}---------");
    println!("-------the value of s1={s2}---------");
}
