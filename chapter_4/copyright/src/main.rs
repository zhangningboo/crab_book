#[allow(unused_variables)]

fn main() {
    print_padovan();

    {
        let point = Box::new((0.625, 3.14));  // 分配point
        let label = format!("{:?}", point);  // 分配label
        assert_eq!(label, "(0.625, 3.14)");
    }  // 丢弃

    let s = vec![
        "udon".to_string(),
        "ramen".to_string(),
        "soba".to_string(),
    ];

    let t = s.clone();  // 发生转移
    let u = s;  // 失败: ^ value used here after move

    
}

struct Person {
    name:
}

fn print_padovan() {
    let mut padovan = vec![1, 1, 1];  // 分配: 指针，容量，长度，实际数据存在堆上
    for i in 3..10 {
        let next = padovan[i - 3] + padovan[i - 2];
        padovan.push(next);
    }
    println!("P(1..10) = {:?}", padovan);
}  // 丢弃 padovan