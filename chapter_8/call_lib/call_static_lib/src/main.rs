use libloading::{Library, Symbol};
use std::path::Path;

pub struct Fern {
    pub size: f64,
    pub growth_rate: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lib_path = Path::new("/home/zhangningboo/workspace/workspace-rust/crab_book/chapter_8/call_lib/lib_demo/target/release/liblib_demo.so");

    // let lib = match unsafe { Library::new(lib_path) } {
    //     Ok(lib) => lib,
    //     Err(e) => {
    //         panic!("Failed to load library: {}", e);
    //     }
    // };

    unsafe {
        let lib = Library::new(lib_path)?;
        // 定义函数指针类型
        type RunSimulation = unsafe extern fn(&mut Fern, usize) -> u32;
        // 加载函数
        let run_simulation: Symbol<RunSimulation> =  lib.get(b"run_simulation")?;
        // 调用函数
        let mut fern = Fern {
            size: 0.1,
            growth_rate: 0.05,
        };

        run_simulation(&mut fern, 10);
        println!("Result of adding 1 and 2: {}", fern.size);
    }

    unsafe {
        let lib = Library::new(lib_path).unwrap();
        // 定义函数指针类型
        type Add = extern fn(i32, i32) -> i32;
        // 加载函数
        let add: Symbol<Add> = lib.get(b"add").unwrap();
        // 调用函数
        let result = add(1, 2);
        println!("Result of adding 1 and 2: {}", result);
    }

    Ok(())
}
