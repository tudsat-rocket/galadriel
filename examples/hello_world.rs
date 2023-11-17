use galadriel::*;

fn main() {
    let mut sim = Simulation::default();

    while sim.state.flight_phase != FlightPhase::Touchdown {
        sim.tick(0.000001);
        //println!("{:?}", sim.state);
    }
}
