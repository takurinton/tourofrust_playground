fn main() {
    c3_2();
}

// 構造体
// c3_1 にしてたら命名でエラー出た。
// struct Chapter31Struct {
//     // String は構造体
//     name: String,
//     age: i32, 
// }

// メソッド呼び出し
// static method は :: で呼び出す
// instance method は . で呼び出す
fn c3_2() {
    // String instance を生成、Java の String みたいな感覚かな
    // シンタックスはPHP的な？
    let s = String::from("hello world");
    println!("{}, {}", s, s.len()); // こっちで読んでるのは instance method
}