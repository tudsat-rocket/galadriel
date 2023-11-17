use crate::drag::*;

#[derive(Default)]
pub enum ParachuteTrigger {
    /// Deploy at apogee
    #[default]
    AtApogee,
    /// Deploy below a certain altitude in m AGL
    BelowAltitude(f32),
    // TODO: SITL
}

pub struct Parachute {
    pub drag_coef: DragCoefficient,
    pub area: f32,
    pub trigger: ParachuteTrigger,
    pub delay: f32,
}

impl Default for Parachute {
    fn default() -> Self {
        Self {
            drag_coef: DragCoefficient::Constant(1.0),
            area: 0.15,
            trigger: ParachuteTrigger::AtApogee,
            delay: 1.0,
        }
    }
}
