use crate::geometry2d::{StaticTriangle2d, Triangle2d};

mod geometry2d;

mod primitives;

fn main() {
    let p1 = (2.0, 0.0).into();
    let p3 = (3.0, 0.0).into();
    let p2 = (2.0, 1.0).into();
    let triangle2d = StaticTriangle2d::new(p1, p2, p3);
    println!("T: {triangle2d:?}, {}", triangle2d.area());
}
