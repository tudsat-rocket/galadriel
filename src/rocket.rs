use crate::drag::*;
use crate::motor::*;
use crate::parachute::*;

pub struct Rocket {
    ///
    pub motor: Motor,
    ///
    pub parachutes: Vec<Parachute>,
    /// Mass without motor casing [kg]
    pub dry_mass: f32,
    drag_coef: DragCoefficient,
    area: f32,
}

impl Default for Rocket {
    fn default() -> Self {
        Self {
            motor: Motor::default(),
            parachutes: vec![
                Parachute::default(),
                Parachute {
                    trigger: ParachuteTrigger::BelowAltitude(300.0),
                    area: 1.0,
                    ..Default::default()
                },
            ],
            dry_mass: 15.0,
            drag_coef: DragCoefficient::Constant(0.2),
            area: 0.02,
        }
    }
}
