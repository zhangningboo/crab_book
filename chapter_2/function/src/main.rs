use std::str::FromStr;  // 引入特型，特型是可以由类型实现的方法集合
use std::env;


fn main() {
    println!("Hello, world!");
    let mut numbers = Vec::new();
    for arg in env::args().skip(1) {
        let trans_result = u64::from_str(&arg);
        match trans_result {
            Ok(num) => numbers.push(num),
            Err(e) => println!("{}: {}", arg, e),
        }
        // numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ...");  // 输出到标准错误流
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {  // 所有权发生转移
        d = gcd(d, *m);  // *m: 传递的是引用，解引用取出值
    }

    println!("The gcd of {:?} is {}", numbers, d);
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}