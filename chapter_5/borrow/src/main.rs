use std::{ptr::addr_of, vec};

fn main() {
    // 5.1
    reference_value();
    // 5.2
    use_reference();

    // 5.4
    share_mut();
}

#[allow(unused_variables, dead_code)]
fn reference_value() {
    use std::collections::HashMap;
    type Table = HashMap<String, Vec<String>>;

    fn show_table(table: &mut Table, edit: bool) {
        for (artist, works) in table {
            println!("works by artist {}:", artist);
            for work in works {
                println!("\t{}", work);
                if edit {
                    work.push_str("!");
                }
            }
        }
    }
    
    let mut table = Table::new();
    table.insert("Giacomo Puccini".to_string(), vec!["Tosca".to_string(), "Madama Butterfly".to_string()]);
    table.insert("Wolfgang Amadeus Mozart".to_string(), vec!["The Marriage of Figaro".to_string()]);

    show_table(&mut table, true);
    println!("");
    show_table(&mut table, false);
}

#[allow(unused_variables, dead_code)]
fn use_reference() {
    println!("\n{}", "use_reference");
    let x:i32= 10;
    let r:&i32 = &x;
    println!("r: {}, addr: 0x{}, 0x{}", r, addr_of!(r) as usize, addr_of!(*r) as usize);
    println!("x: {}, addr: 0x{}", x, addr_of!(x) as usize);
    assert_eq!(*r, x);  // 解引用，*r指向的是x

    let mut v = vec![3, 1, 2];
    v.sort();  // 隐式解引用，与inplace方法不同
    // or
    (&mut v).sort();

    // 对引用变量赋值
    let x = 10;
    let y = 20;
    let mut r = &x;
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now();
    let duration = now.duration_since(UNIX_EPOCH).unwrap();
    if duration.as_millis() % 2 == 0 {
        r = &y;
    }
    assert!(*r == y || *r == x);

    // 对引用进行引用
    struct Point {
        x: i32,
        y: i32,
    }
    let p:Point = Point { x: 10, y: 20 };
    let r:&Point = &p;
    let r2:&&Point = &r;
    let r3:&&&Point = &r2;
    assert!(r3.x == p.x);  // 隐式解引用 / 追踪到最原始的引用来源

    // 比较引用
    let x = 10;
    let y = 10;
    let rx = &x;
    let ry = &y;
    let rrx = &rx;
    let rry = &ry;
    assert!(rrx == rry);

    // 引用永不为空，将空指针使用Option<>代替
}

#[allow(unused_variables, dead_code)]
fn share_mut() {
    // 共享访问是只读访问
    // 可变访问是独占访问

    {
        let v = vec![1, 2, 3];
        let r = &v;
        println!("{}", r[0]);  // 正确
        let asside = v;
        // print!("{}", r[0]);  // 错误，v已经转移，此处v是空
    }

    {
        fn extend(vec: &mut Vec<i32>, slice: &[i32]) {
            for item in slice.iter() {
                vec.push(*item);
            }
        }

        let mut wave = Vec::new();
        extend(&mut wave, &[1, 2, 3]);
        println!("{}", wave[0]);
        // extend(&mut wave, &wave);  // 错误，wave被可变访问了，不能再进行共享访问
    }
}