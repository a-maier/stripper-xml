use hepmc2::event::{Vertex, EnergyUnit, LengthUnit};
use particle_id::ParticleID;

use crate::{SubEvent, Status, Particle, Momentum, Id};

const HEPMC_INCOMING_STATUS: i32 = 4;
const HEPMC_OUTGOING_STATUS: i32 = 1;

// Id of auxiliary vertex.
//
// According to the HepMC standard, it is supposed to be negative
const VTX_ID: i32 = -1;

impl From<&SubEvent> for hepmc2::Event {
    fn from(ev: &SubEvent) -> Self {
        let mut incoming = Vec::with_capacity(2);
        let mut outgoing = Vec::with_capacity(
            std::cmp::max(ev.particles.len(), 2) - 2
        );
        for particle in &ev.particles {
            match particle.id.status {
                Status::Outgoing => {
                    outgoing.push(particle.into());
                }
                Status::Incoming => {
                    let mut p = hepmc2::event::Particle::from(particle);
                    p.end_vtx = VTX_ID;
                    incoming.push(p);
                }
            };
        }
        let vertices = vec![Vertex {
            particles_in: incoming,
            particles_out: outgoing,
            barcode: VTX_ID,
            ..Default::default()
        }];
        Self {
            scale: ev.mu_r,
            vertices,
            weights: vec![ev.weight],
            energy_unit: EnergyUnit::GEV,
            length_unit: LengthUnit::MM,
            ..Default::default()
        }
    }
}

impl From<&hepmc2::Event> for SubEvent {
    fn from(ev: &hepmc2::Event) -> Self {
        let particles = ev.vertices.iter().flat_map(|vx| {
            vx.particles_in
                .iter()
                .filter(|p| p.status == HEPMC_INCOMING_STATUS)
                .chain(
                    vx.particles_out
                        .iter()
                        .filter(|p| p.status == HEPMC_OUTGOING_STATUS)
                )
        }).map(Into::into)
            .collect();

        Self {
            weight: ev.weights.first().copied().unwrap_or_default(),
            mu_r: ev.scale,
            particles,
            ..Default::default()
        }
    }
}

impl From<&hepmc2::event::Particle> for Particle {
    fn from(p: &hepmc2::event::Particle) -> Self {
        let status = match p.status {
            HEPMC_INCOMING_STATUS => Status::Incoming,
            HEPMC_OUTGOING_STATUS => Status::Outgoing,
            _ => panic!("Can only convert incoming or outgoing particles"),
        };
        Self {
            id: Id {
                status,
                pdg_id: ParticleID::new(p.id),
            },
            momentum: Momentum(p.p.0),
        }
    }
}

impl From<&Particle> for hepmc2::event::Particle {
    fn from(particle: &Particle) -> Self {
        let status = match particle.id.status {
            Status::Incoming => HEPMC_INCOMING_STATUS,
            Status::Outgoing => HEPMC_OUTGOING_STATUS
        };
        let p = particle.momentum.0;
        Self {
            id: particle.id.pdg_id.id(),
            p: hepmc2::event::FourVector(p),
            status,
            theta: theta(p),
            phi: phi(p),
            ..Default::default()
        }
    }
}

fn phi(p: [f64; 4]) -> f64 {
    p[1].atan2(p[2])
}

fn theta(p: [f64; 4]) -> f64 {
    pt(p).atan2(p[3])
}

fn pt2(p: [f64; 4]) -> f64 {
    p[1] * p[1] + p[2] * p[2]
}

fn pt(p: [f64; 4]) -> f64 {
    pt2(p).sqrt()
}
