use ode::*;

fn main() {
    let mut euler = Euler::new(0.0, 0.0, 0.1);
    let f = |x: f32, y: f32| x + y;

    println!("i x y f exact");
    for i in 0..=10 {
        let x = euler.x();
        let y = euler.y();
        let fi = f(x, y);
        let exact = x.exp() - x - 1.;
        println!("{i} {x} {y} {fi} {exact}");
        euler.step(f);
    }
}
