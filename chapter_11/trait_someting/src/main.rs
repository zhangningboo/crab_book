fn main() {
    // 11.1
    generics_info();

    // 11.2
    define_trait();

    // 11.3
    completely_limit_call();
}

#[allow(unused_variables, dead_code)]
fn  generics_info() {
    use std::io::Write;

    // fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {  // 普通函数版本
    //     out.write_all(b"Hello World!\n")?;
    //     out.flush()
    // }
    
    fn say_hello<W>(out: &mut W) -> std::io::Result<()> 
        where W: Write 
    {  // 泛型函数版本
        out.write_all(b"Hello World!\n")?;
        out.flush()
    }
    // TODO FIX: 第二次执行会报错：called `Result::unwrap()` on an `Err` value: Os { code: 9, kind: Uncategorized, message: "Bad file descriptor" }
    // use std::fs::File;
    // let mut file1 = match File::open("file1.txt") {
    //     Ok(file) => file,
    //     Err(_) => File::create("file1.txt").unwrap()
    // };
    // say_hello(&mut file1).unwrap();

    use std::fs::OpenOptions;
    let mut file2 = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("file2.txt").unwrap();
    say_hello(&mut file2).unwrap(); 

    let mut bytes = vec![];
    say_hello(&mut bytes).unwrap();
    assert_eq!(bytes, b"Hello World!\n");

    fn min<T: Ord>(v1: T, v2: T) -> T {  // T： 实现Ord特型的任意类型
        if v1 <= v2 {
            v1
        } else {
            v2
        }
    }

    
}

#[allow(unused_variables, dead_code)]
fn define_trait() {
    // 定义特型，实现特型

    struct Canvas{

    }

    impl Canvas {
        fn write_at(&mut self, x: i32, y: i32, c: char) {

        }
    }

    struct Broom{
        x: i32,
        y: i32,
        height: i32,
    }

    impl Broom {  // Broom 对象都有的方法
        
    }

    trait Visiable {
        fn draw(&self, canvas: &mut Canvas);

        fn hit_test(&self, x: i32, y: i32) -> bool;
    }

    impl Visiable for Broom {  // 为自己的工程Broom添加实现的方法
        fn draw(&self, canvas: &mut Canvas) {
            for y in self.y - self.height - 1 ..self.y {
                canvas.write_at(self.x, y, '#');
            }
            canvas.write_at(self.x, self.y, '#');
        }

        fn hit_test(&self, x: i32, y: i32) -> bool {
            self.x == x && self.y - self.height - 1 <= y && y <= self.y
        }
    }

    // 默认方法
    pub struct Sink;
    use std::io::{Write, Result};
    impl Write for Sink {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }
    let mut out = Sink;
    out.write_all(b"hello").unwrap();  // Write特型默认实现了write_all，所以Sink未实现依旧可以使用

    use std::fs::OpenOptions;
    use std::io::{BufReader, Read};
    let hashmap = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .append(true)
        .open("hashmap.txt").unwrap();
    let mut content = String::new();
    let mut reader = BufReader::new(&hashmap);
    let _ = reader.read_to_string(&mut content);
    println!("{:?}", content);

    use std::collections::HashMap;
    let mut config: HashMap::<String, String> = HashMap::new();

    use chrono::{DateTime, Local};
    let now: DateTime<Local> = Local::now();
    let formatted = now.format("%Y-%m-%d %H:%M:%S").to_string();
    config.insert(formatted.clone(), formatted);
    use serde::Serialize;
    use serde_json;
    let mut serializer = serde_json::Serializer::new(hashmap);
    config.serialize(&mut serializer).unwrap();  // 特型实现的方法，如果想使用，该方法必须在作用域内，.serialize是serde_json为rust数据类型实现的序列化方法

}

#[allow(unused_variables, dead_code)]
fn completely_limit_call() {
    let hell0 = "hello";
    let h1 = hell0.to_string();  // 限定方法调用
    let h2 = str::to_string(hell0);  // 限定方法调用
    let h3 = ToString::to_string(hell0);  // 限定方法调用
    let h4 = <str as ToString>::to_string(hell0);  // 完全限定调用
    assert!(h1 == h2 && h2 == h3 && h3 == h4);  // 四种方法完全一致
}