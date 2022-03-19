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

pub struct RungeKutta {
    x: f32,
    y: f32,
    step: f32,
}

impl RungeKutta {
    pub fn new(x_min: f32, y_init: f32, step: f32) -> Self {
        Self {
            x: x_min,
            y: y_init,
            step,
        }
    }

    /// Returns the result of fourth-order Runge-Kutta method for a given function
    /// f(x, y) -> dy/dx
    pub fn step(&mut self, f: impl Fn(f32, f32) -> f32) -> f32 {
        let k1 = self.step * f(self.x, self.y);
        let k2 = self.step * f(self.x + self.step / 2., self.y + k1 / 2.);
        let k3 = self.step * f(self.x + self.step / 2., self.y + k2 / 2.);
        let k4 = self.step * f(self.x + self.step, self.y + k3);
        self.y += (k1 + 2. * k2 + 2. * k3 + k4) / 6.;
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
