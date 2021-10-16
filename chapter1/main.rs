fn main() {
    c1_1();
    c1_2();
    c1_3();
    c1_4();
    c1_5();
    c1_6();
    println!("{}", c1_7(1,2));
    c1_7_2("hello world");

    // 受け取り方は変数として受け取るか、タプルとして受け取るかの2択がある
    let (x, y) = c1_8(1, 2);
    println!("{}, {}", x,y);
    let res = c1_8(1, 2);
    println!("{}, {}", res.0, res.1);

    // 空の戻り値はタプルを返す
    println!("{:?}", c1_9());
    println!("{:?}", c1_9_2());

}

// 型定義
fn c1_1() {
    // 型推論してくれる
    let x = 13;
    println!("{}", x);

    // 明示
    let y: f64 = 3.1415;
    println!("{}", y);

    // 先に宣言だけ
    let z;
    z = 100;
    println!("{}", z);
}

// 可変と不変
fn c1_2() {
    // mut は不変値を示す
    #[allow(unused_mut)]
    let mut x = 42;
    println!("{}", x);

    // 再代入でエラー
    // x = 13;
}

// 型
fn c1_3() {
    let x = 12; // デフォルト i32
    let a = 12u8;
    let b = 4.3; // デフォルト f64
    let c = 4.3f32;
    let bv = true;
    let t = (13, false);
    let sentence = "hello world!";
    println!(
        "{} {} {} {} {} {} {} {}",
        x, a, b, c, bv, t.0, t.1, sentence
    );
}

// u8 と u32 は区別されるけど、as で変換可能
fn c1_4() {
    let a = 13u8;
    let b = 7u32;
    let c = a as u32 + b; // as を指定すると変換することができる
    println!("{}", c);

    let t = true;
    println!("{}", t as u8);
}

// 定数
fn c1_5() {
    const PI: f32 = 3.14159;
    println!(
        "PI: {}",
        PI
    );
}

// 配列
fn c1_6() {
    // [type; length]、lengthはコンパイル時にメモリが確保されるサイズ
    let nums: [i32; 3] = [1, 2, 3];
    // {:要素数} で format する、わからない場合は ? を使うと不変になる
    println!("{:?}", nums);
    println!("{}", nums[1]); // 参照は普通の言語と同じ
}

// 関数
// 戻り値あり、int
fn c1_7(x: i32, y: i32) -> i32 {
    return x+y;
}

// 戻り値なし、str
fn c1_7_2(message: &str) {
    println!("{}", message);
}

// 複数の戻り値
fn c1_8(x: i32, y: i32) -> (i32, i32) {
    return (y, x);
}

// 空の戻り値
fn c1_9() -> () {
    return ();
}

fn c1_9_2() {
}