use libloading::{Library, Symbol};
use std::path::Path;

pub struct Fern {
    pub size: f64,
    pub growth_rate: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let dynamic_path = "../cpp_lib/build/libcpp_lib_test_dynamic.so";
    let static_path = "../cpp_lib/build/libcpp_lib_test_static.a";

    // unsafe {
    //     let lib = Library::new(lib_path)?;
    //     // 定义函数指针类型
    //     type RunSimulation = unsafe extern fn(&mut Fern, usize) -> u32;
    //     // 加载函数
    //     let run_simulation: Symbol<RunSimulation> =  lib.get(b"run_simulation")?;
    //     // 调用函数
    //     let mut fern = Fern {
    //         size: 0.1,
    //         growth_rate: 0.05,
    //     };

    //     run_simulation(&mut fern, 10);
    //     println!("Result of adding 1 and 2: {}", fern.size);
    // }

    unsafe {
        let lib_path = Path::new(dynamic_path);
        let lib = Library::new(lib_path).unwrap();
        // 定义函数指针类型
        type Add = extern fn(i32, i32) -> i32;
        // 加载函数
        let add: Symbol<Add> = lib.get(b"cpp_add").unwrap();
        // 调用函数
        let result = add(1, 2);
        println!("Result of adding 1 and 2: {}", result);
    }

    unsafe {
        let lib_path = Path::new(static_path);
        let lib = Library::new(lib_path).unwrap();
        // 定义函数指针类型
        type Add = extern fn(i32, i32) -> i32;
        // 加载函数
        let add: Symbol<Add> = lib.get(b"cpp_add").unwrap();
        // 调用函数
        let result = add(1, 2);
        println!("Result of adding 1 and 2: {}", result);
    }

    Ok(())
}
