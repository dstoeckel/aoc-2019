use regex::Regex;
use std::io::BufRead;

#[test]
fn test_simulate() {
    let mut bodies = vec![
        Body::new(Vector::new(-8, -10, 0)),
        Body::new(Vector::new(5, 5, 10)),
        Body::new(Vector::new(2, -7, 3)),
        Body::new(Vector::new(9, -8, -3)),
    ];

    assert_eq!(1940, simulate(&mut bodies, 100))
}

fn test_simulate2() {
    let mut bodies = vec![
        Body::new(Vector::new(-1, 7, 3)),
        Body::new(Vector::new(12, 2, -13)),
        Body::new(Vector::new(14, 18, -8)),
        Body::new(Vector::new(17, 4, -4)),
    ];

    assert_eq!(7077, simulate(&mut bodies, 1000));
}

#[derive(Copy, Clone)]
struct Vector {
    x: isize,
    y: isize,
    z: isize,
}

impl Vector {
    fn new(x: isize, y: isize, z: isize) -> Vector {
        Vector { x, y, z }
    }

    fn zero() -> Vector {
        Vector::new(0, 0, 0)
    }
}

struct Body {
    p: Vector,
    v: Vector,
}

impl Body {
    fn new(p: Vector) -> Body {
        Body {
            p,
            v: Vector::zero(),
        }
    }
}

fn parse_position(x: &str) -> Body {
    let re = Regex::new(r"<x=(-?[0-9]+), y=(-?[0-9]+), z=(-?[0-9]+)>").unwrap();
    let caps = re.captures(x).unwrap();

    Body::new(Vector::new(
        caps.get(1).unwrap().as_str().parse().unwrap(),
        caps.get(2).unwrap().as_str().parse().unwrap(),
        caps.get(3).unwrap().as_str().parse().unwrap(),
    ))
}

impl std::ops::Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl std::ops::AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl std::ops::Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl std::ops::SubAssign for Vector {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl std::ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector::new(-self.x, -self.y, -self.z)
    }
}

impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<x={}, y={}, z={}>", self.x, self.y, self.z)
    }
}

fn simulate(bodies: &mut Vec<Body>, steps: usize) -> isize {
    let n_body = bodies.len();
    let mut potential_energy = 0;
    let mut kinetic_energy = 0;
    let mut total_energy = 0;

    for n in 0..steps {
        for i in 0..(n_body - 1) {
            for j in (i + 1)..n_body {
                let diff = bodies[i].p - bodies[j].p;

                let delta_v = Vector::new(diff.x.signum(), diff.y.signum(), diff.z.signum());

                bodies[i].v -= delta_v;
                bodies[j].v += delta_v;
            }
        }

        for a in bodies.iter_mut() {
            a.p += a.v;
            println!("pos = {}, vel = {}>", a.p, a.v);
        }

        potential_energy = 0;
        kinetic_energy = 0;
        total_energy = 0;
        for a in bodies.iter() {
            let pot = a.p.x.abs() + a.p.y.abs() + a.p.z.abs();
            let kin = a.v.x.abs() + a.v.y.abs() + a.v.z.abs();
            potential_energy += pot;
            kinetic_energy += kin;
            total_energy += pot * kin;
        }

        println!(
            "Step {}: E_pot = {}, E_kin = {}, E_tot = {}",
            n, potential_energy, kinetic_energy, total_energy
        );
    }

    total_energy
}

fn main() {
    let path = std::env::args().nth(1).unwrap();

    let file = std::fs::File::open(path).unwrap();

    let mut bodies: Vec<_> = std::io::BufReader::new(file)
        .lines()
        .map(|l| parse_position(&l.unwrap()))
        .collect();

    simulate(&mut bodies, 1000);
}
