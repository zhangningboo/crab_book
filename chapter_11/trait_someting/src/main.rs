fn main() {

    // 11.2
    define_trait();
}

#[allow(unused_variables, dead_code)]
fn define_trait() {

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
            true
        }
    }
}