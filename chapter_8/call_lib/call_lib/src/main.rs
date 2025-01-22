use lib_demo::Fern;

fn main() {
    let mut fern = Fern {
        size: 0.1,
        growth_rate: 0.05,
    };

    lib_demo::run_simulation(&mut fern, 10);
    println!("Fern size: {}", fern.size);
    
}
