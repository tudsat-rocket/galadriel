use nalgebra::*;

pub mod drag;
pub mod environment;
pub mod motor;
pub mod parachute;
pub mod rocket;

use environment::*;
use rocket::*;

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub enum FlightPhase {
    #[default]
    PreLaunch,
    LaunchRail,
    Burn,
    Coast,
    Control,
    Descent,
    Touchdown,
}

#[derive(Clone, Default, Debug)]
#[non_exhaustive]
pub struct SimulationState {
    pub time: f32,
    pub flight_phase: FlightPhase,
    pub phase_start_time: f32,
    pub position: Vector3<f32>,
    pub velocity: Vector3<f32>,
    pub acceleration: Vector3<f32>,
    // TODO: state of motor/parachute?
}

#[derive(Default)]
#[non_exhaustive]
pub struct Simulation {
    rocket: Rocket,
    environment: Environment,
    pub state: SimulationState,
}

impl Simulation {
    pub fn tick(&mut self, delta_time: f32) {
        let t = self.state.time;

        match self.state.flight_phase {
            FlightPhase::PreLaunch if t > 1.0 => {
                self.rocket.motor.ignite(self.state.time); // TODO: avoid passing time?
                self.state.flight_phase = FlightPhase::LaunchRail;
            }
            FlightPhase::LaunchRail
                if self.state.position.z > self.environment.launch_rail_length =>
            {
                // TODO: trigonometry
                self.state.flight_phase = FlightPhase::Burn;
            }
            FlightPhase::Burn if self.rocket.motor.burned_out(t) => {
                self.state.flight_phase = FlightPhase::Coast;
            }
            FlightPhase::Coast if self.state.velocity.z < 0.0 => {
                self.state.flight_phase = FlightPhase::Descent;
            }
            FlightPhase::Descent if self.state.position.z <= 0.0 => {
                self.state.flight_phase = FlightPhase::Touchdown;
                self.state.position.z = 0.0;
                self.state.velocity = Vector3::new(0.0, 0.0, 0.0);
                self.state.acceleration = Vector3::new(0.0, 0.0, 0.0);
            }
            _ => {}
        }

        // calculate new acceleration
        let thrust = Vector3::new(0.0, 0.0, self.rocket.motor.thrust(t));
        let drag = Vector3::new(0.0, 0.0, 0.0);
        let wind = Vector3::new(0.0, 0.0, 0.0);

        let forces = thrust + drag + wind;
        let mass = self.rocket.dry_mass + self.rocket.motor.mass(t);
        self.state.acceleration = forces / mass;
        if self.state.flight_phase != FlightPhase::PreLaunch
            && self.state.flight_phase != FlightPhase::Touchdown
        {
            self.state.acceleration.z -= 10.0;
        }

        self.state.velocity += self.state.acceleration * delta_time;
        self.state.position += self.state.velocity * delta_time;

        // TODO: trigger parachutes

        self.state.time += delta_time;
    }
}
