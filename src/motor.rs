pub struct Motor {
    dry_mass: f32,
    wet_mass: f32,
    thrust_curve: Vec<(f32, f32)>,
    ignition_time: Option<f32>,
}

impl Default for Motor {
    fn default() -> Self {
        Self {
            dry_mass: 3.656,
            wet_mass: 8.108,
            thrust_curve: vec![(0.0, 0.0), (0.01, 3415.0), (2.93, 3415.0), (2.94, 0.0)],
            ignition_time: None,
        }
    }
}

impl Motor {
    fn burn_time(&self) -> f32 {
        self.thrust_curve.last().unwrap().0
    }

    fn thrust_at_time(&self, time_since_ignition: f32) -> f32 {
        // TODO
        let average_thrust = self.thrust_curve[2].1;

        if time_since_ignition < self.burn_time() {
            average_thrust
        } else {
            0.0
        }
    }

    pub fn thrust(&self, time: f32) -> f32 {
        match self.ignition_time {
            None => 0.0,
            Some(t) if time > t => self.thrust_at_time(time - t),
            _ => 0.0,
        }
    }

    pub fn ignite(&mut self, time: f32) {
        self.ignition_time = Some(time);
    }

    pub fn burned_out(&self, time: f32) -> bool {
        match self.ignition_time {
            None => false,
            Some(t) => time > (t + self.burn_time()),
        }
    }

    pub fn mass(&self, time: f32) -> f32 {
        match self.ignition_time {
            None => self.wet_mass,
            Some(t) => self.dry_mass, // TODO
        }
    }
}
