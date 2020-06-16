struct Triangle {
    base: u32,
    height: u32,
}

impl Triangle {
    fn area(&self) -> u32 {
        let a = &self.base * &self.height / 2;
        a
    }
    fn info() {
        println!("A triangle has 3 sides. All its angles must make 360 degrees.");
    }
}

pub fn runner() {
    let triangle = Triangle {
        base: 10,
        height: 20
    };
    Triangle::info();
    println!("{:?}", triangle.area());
}