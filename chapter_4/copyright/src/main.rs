#[allow(unused_variables)]
fn main() {
    // 4.1
    reference();
    // 4.2
    move_case();
    // 4.3
    copy_case();
    // 4.4
    share_copyright();
}

fn print_padovan() {
    let mut padovan = vec![1, 1, 1];  // 分配: 指针，容量，长度，实际数据存在堆上
    for i in 3..10 {
        let next = padovan[i - 3] + padovan[i - 2];
        padovan.push(next);
    }
    println!("P(1..10) = {:?}", padovan);
}  // 丢弃 padovan

fn reference() {
    print_padovan();

    {
        let point = Box::new((0.625, 3.14));  // 分配point
        let label = format!("{:?}", point);  // 分配label
        assert_eq!(label, "(0.625, 3.14)");
    }  // 丢弃
}

struct Person {
    name: Option<String>,
    birth: i32,
}

#[allow(unused_variables)]
fn move_case() {
    let s = vec![
        "udon".to_string(),
        "ramen".to_string(),
        "soba".to_string(),
    ];

    let t = s.clone();  // 发生转移
    let u = s;  // 失败: ^ value used here after move

    let mut v = Vec::new();
    for i in 101..106 {
        v.push(Person {
            name: Some(format!("Person {}", i)),
            birth: i,
        });
    }

    // 移除vec的某一个元素
    let fifth = v.pop().expect("vec is empty");
    assert_eq!(fifth.birth, 105);

    let second = v.swap_remove(1);
    assert_eq!(second.birth, 102);

    let first_name = v[0].name.take();  // 取走，发生转移，剩下的为None
    assert_eq!(first_name, Some("Person 101".to_string()));
    assert_eq!( v[0].name, None);
    println!("{}", first_name.unwrap());
}

struct NormalStruct {
    num: i32,
    name: String,
}

#[derive(Clone, Copy, Debug)]  // Debug 是为了可以在println!中打印
struct CopyableStruct {
    num: i32,
    cnt: u8,
}

#[allow(unused_variables)]
fn copy_case() {
    // 可以通过简单的 复制位 复制其值的类型才能作为Copy类型
    // 任何在丢弃时，需要一些特殊操作的类型都不是Copy类型
    // 默认struct不是Copy类型，如果所有字段都是Copy类型，那么可以加#[derive(Copy, Clone)]

    let copyable = CopyableStruct { num: 1, cnt: 2 };
    let normal = NormalStruct { num: 1, name: "test".to_string() };
    let mut c1 = copyable;
    c1.cnt = 1;
    let c2 = c1;
    println!("{:?}", copyable);
    println!("{:?}", c2);
}

#[allow(unused_variables)]
fn share_copyright() {
    // Arc: 原子引用计数，线程安全
    // Rc: 引用计数，线程不安全
    use std::rc::Rc;

    // Rc要求的内容不可变
    let s: Rc<String> = Rc::new("hello".to_string());
    let t = s.clone();  // 克隆操作，不会克隆具体的值，只是创建一个指向它的指针并递增引用计数，类似Python的引用计数

}