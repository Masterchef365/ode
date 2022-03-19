// TODO: Extend to N dimensions

pub struct Euler {
    x: f32,
    y: f32,
    step: f32,
}

impl Euler {
    pub fn new(x_min: f32, y_init: f32, step: f32) -> Self {
        Self {
            x: x_min,
            y: y_init,
            step,
        }
    }

    /// Returns the result of Euler's method for a given function
    /// f(x, y) -> dy/dx
    pub fn step(&mut self, f: impl Fn(f32, f32) -> f32) -> f32 {
        self.y = self.y + self.step * f(self.x, self.y);
        self.x += self.step;
        self.y
    }
    
    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }
}
