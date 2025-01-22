pub struct Fern {
    pub size: f64,
    pub growth_rate: f64,
}

impl Fern {
    pub fn grow(&mut self) {
        self.size *= 1.0 + self.growth_rate;
    }
}

#[no_mangle]  // 保证编译后函数名不变
pub fn run_simulation(fern: &mut Fern, days: usize) -> u32 {
    for _ in 0..days {
        fern.grow();
    }
    1 as u32
}

#[no_mangle]  // 保证编译后函数名不变
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}