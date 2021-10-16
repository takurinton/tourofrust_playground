fn main() {
    c2_1();
    c2_2();
    c2_3();
    c2_4();
    c2_5();
    c2_6();
    println!("{:?}", c2_7());
}

// 条件分岐、入門者みたいな式書いてしまった、これくらいしか思いつかん
fn c2_1() {
    let x = 21;
    if x < 20 {
        println!("未成年")
    } else {
        println!("成人")
    }
}

// 繰り返し
// 無限ループ
fn c2_2() {
    // 可変
    let mut x = 0;
    loop {
        x += 1;
        if x == 10 {
            break;
        }
    }
    println!("{}", x);
}

// while
fn c2_3() {
    let mut x = 0;
    while x != 10 {
        x += 1;
    }
    println!("{}", x);
}

// for 
fn c2_4() {
    // 10を含まない(0-9までの繰り返し)
    for x in 0..10 {
        println!("{}", x);
    }

    // 10を含む(0-10までの繰り返し)
    for x in 0..=10 {
        println!("{}", x);
    }
    // println!("{}", 0..10); `std::ops::Range<{integer}>` cannot be formatted with the default formatter
}

// match(switch的な)
fn c2_5() {
    let name = "takurinton";
    match name {
        // 一致
        "takurinton" => {
            println!("{}", name);
        }
        // 複数
        "hoge" | "fuga" => {
            println!("hoge or fuga");
        }
        // else
        _ => {
            println!("other value");
        }
    }

    // string と int でやれることが違ったので2パターン目
    let num = 10;
    match num {
        0 => {
            println!("found zero");
        }
        1 | 2 => {
            println!("found 1 or 2!");
        }
        // 範囲
        3..=9 => {
            println!("found a number 3 to 9 inclusively");
        }
        // マッチした値を変数に入れる
        matched_num @ 10..=100 => {
            println!("found {} number between 10 to 100!", matched_num);
        }
        _ => {
            println!("found something else!");
        }
    }
}

// break に値を持たせることができる
fn c2_6() {
    let mut x = 0;
    let v = loop {
        x += 1;
        if x == 10 {
            break "10";
        };
    };

    println!("戻り値: {}", v);
}

// ブロック式から値を返す
// if や match は値を返してる、そのまま結果を代入することができる
// 終わりに ; がついてないと戻り値として扱われる
fn c2_7() -> i32 {
    let x = 10;
    let v = if x == 10 { "10" } else { "not 10" };
    println!("{}", v);

    let name = "takurinton";
    let res = match name {
        "takurinton" => "I am takurinton", // ; じゃなくて , だと戻り値扱いになる
        "hoge" | "fuga" => "hoge or fuga",
        _ => "else",
    };
    println!("{}", res);

    // ここは関数スコープからの分離
    let v = {
        let a = 1;
        let b = 2;
        a + b // ; がないと戻り値
    };
    println!("{}", v);
    v + 1
}