mod fast_tuple;
use crate::fast_tuple::FastTuple;

pub struct Projectile {
    pub position: FastTuple,
    pub velocity: FastTuple,
}

#[derive(Clone, Copy)]
pub struct Environment {
    pub gravity: FastTuple,
    pub wind: FastTuple,
}

fn tick(env: Environment, proj: Projectile) -> Projectile {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;
    Projectile {
        position: position,
        velocity: velocity,
    }
}

fn main() {
    let mut t = 0;
    let mut p = Projectile {
        position: FastTuple::point(0.0, 1.0, 0.0),
        velocity: FastTuple::normalize(FastTuple::vector(1.0, 1.0, 0.0)),
    };
    let e = Environment {
        gravity: FastTuple::vector(0.0, -0.1, 0.0),
        wind: FastTuple::vector(-0.01, 0.0, 0.0),
    };

    while p.position.y >= 0.0 {
        println!(
            "Projectile position at time t={}: {}, {}, {}",
            t, p.position.x, p.position.y, p.position.z
        );
        p = tick(e, p);
        t += 1;
    }

    println!(
        "Projectile position at time t={}: {}, {}, {}",
        t, p.position.x, p.position.y, p.position.z
    );
}
