use criterion::{criterion_group, criterion_main, Criterion};

use galadriel::*;

fn tick(c: &mut Criterion) {
    c.bench_function("tick", |b| {
        let mut sim = Simulation::default();

        while sim.state.flight_phase != FlightPhase::Burn {
            sim.tick(0.01);
        }

        b.iter(|| {
            sim.tick(0.0);
            //sim.state = state.clone();
            // TODO: properly test ticks without updates?
        })
    });
}

fn complete_sim_100hz(c: &mut Criterion) {
    c.bench_function("complete_sim_1000hz", |b| {

        b.iter(|| {
            let mut sim = Simulation::default();
            while sim.state.flight_phase != FlightPhase::Touchdown {
                sim.tick(0.001);
            }
        })
    });
}

criterion_group!(benches, tick, complete_sim_100hz);
criterion_main!(benches);
