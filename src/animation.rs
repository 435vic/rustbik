use std::f32::consts::PI;


///
/// Smooth out an arbitrary value using second order dynamics.
///
/// Based on t3ssel8r's video, this class implements a second order
/// differential equation to smooth out, dampen or otherwise interpolate
/// a discrete value.
///
/// You can use it to make movement look more natural,
/// smooth out rotations or zooming, and even more uses I can't think
/// of right now.
///
pub struct SecondOrderSystem {
    x_prev: f32,
    y: f32,
    dy: f32,
    k1: f32,
    k2: f32,
    k3: f32,
}

///
/// Initial parameters for the second order system.
/// As a rule of thumb:
/// - If you want to change how the system settles after some movement, change **zeta**.
/// - If you want to change the initial response of the system, change **r**.
/// - If you want to speed up or slow down the system as a whole, change **freq**.
///
pub struct SecondOrderSystemParameters {
    /// The natural frequency of the system in cycles per second.
    /// it controls the general speed of the system.
    pub freq: f32,
    /// The damping factor of the system.
    /// It dictates how quickly or slowly a movement 'dies out' after a sudden change.
    /// When `zeta < 1`, the system is *underdamped*, and will oscillate when reaching the
    /// current value, like a spring. When it's greater than 1, the system is *overdamped*
    /// and will not oscillate, instead slowly settling to the value.
    pub zeta: f32,
    /// The response time of the system.
    /// It controls how quickly the system moves after its input moves, or its reaction time.
    /// When `r > 1` the system will overshoot the value, and when `r < 0` it will anticipate
    /// the value, going the opposite direction of the movement for a little bit.
    pub r: f32,
}

impl SecondOrderSystem {
    pub fn new(params: SecondOrderSystemParameters, x_initial: f32) -> Self {
        let k1 = params.zeta / (PI * params.freq);
        let k2 = 1.0 / ((2.0 * PI * params.freq) * (2.0 * PI * params.freq));
        let k3 = params.r * params.zeta / (2.0 * PI * params.freq);
        SecondOrderSystem {
            x_prev: x_initial,
            y: x_initial,
            dy: 0.0,
            k1,
            k2,
            k3,
        }
    }

    /// Update the system with a new value. The speed will be approximated using
    /// historical values.
    pub fn update(&mut self, timestep: f32, x: f32) {
        let dx = if timestep > 0.0 {
            (x - self.x_prev) / timestep
        } else {
            0.0
        };
        self.x_prev = x;
        self.update_with_speed(timestep, x, dx);
    }

    /// Update the system with a new value, specifying its current speed.
    pub fn update_with_speed(&mut self, timestep: f32, x: f32, dx: f32) {
        if (x - self.y).abs() < 0.0001 {
            self.y = x;
            return;
        }
        self.y = self.y + self.dy * timestep;
        self.dy = (self.dy * self.k2 + (x + dx * self.k3 - self.y) * timestep)
            / (self.k2 + timestep * self.k1);
    }

    /// Get the current state of the system.
    pub fn value(&self) -> f32 {
        self.y
    }
}


