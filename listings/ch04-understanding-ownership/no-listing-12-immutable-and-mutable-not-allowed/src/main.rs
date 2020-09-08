fn main() {
    // ANCHOR: here
    let mut s = String::from("hello");

    let r1 = &s; // 沒問題
    let r2 = &s; // 沒問題
    let r3 = &mut s; // 很有問題！

    println!("{}, {}, and {}", r1, r2, r3);
    // ANCHOR_END: here
}
