use regex::Regex;
use std::collections::HashMap;
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

#[derive(Clone)]
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

#[derive(Hash, PartialEq, Eq, Clone)]
struct Body1d {
    p: isize,
    v: isize,
}

impl Body1d {
    fn new(p: isize) -> Body1d {
        Body1d { p, v: 0 }
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
    }

    let mut potential_energy = 0;
    let mut kinetic_energy = 0;
    let mut total_energy = 0;

    for a in bodies.iter() {
        let pot = a.p.x.abs() + a.p.y.abs() + a.p.z.abs();
        let kin = a.v.x.abs() + a.v.y.abs() + a.v.z.abs();
        potential_energy += pot;
        kinetic_energy += kin;
        total_energy += pot * kin;
    }

    println!(
        "E_pot = {}, E_kin = {}, E_tot = {}",
        potential_energy, kinetic_energy, total_energy
    );

    total_energy
}

fn simulate_1d(bodies: &mut Vec<Body1d>, max_steps: u64) -> Option<u64> {
    let n_body = bodies.len();
    let mut states = HashMap::new();

    states.insert(bodies.clone(), 0);

    for n in 0..max_steps {
        for i in 0..(n_body - 1) {
            for j in (i + 1)..n_body {
                let delta_v = (bodies[i].p - bodies[j].p).signum();
                bodies[i].v -= delta_v;
                bodies[j].v += delta_v;
            }
        }

        for b in bodies.iter_mut() {
            b.p += b.v;
        }

        if states.contains_key(bodies) {
            let idx = states.get(bodies).unwrap();
            return Some(n - idx + 1);
        }

        states.insert(bodies.clone(), n + 1);
    }

    return None;
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b > 0 {
        a %= b;
        let tmp = b;
        b = a;
        a = tmp;
    }

    a
}

fn main() {
    let path = std::env::args().nth(1).unwrap();

    let file = std::fs::File::open(path).unwrap();

    let mut bodies: Vec<_> = std::io::BufReader::new(file)
        .lines()
        .map(|l| parse_position(&l.unwrap()))
        .collect();

    let mut body_x: Vec<_> = bodies.iter().map(|b| Body1d::new(b.p.x)).collect();
    let mut body_y: Vec<_> = bodies.iter().map(|b| Body1d::new(b.p.y)).collect();
    let mut body_z: Vec<_> = bodies.iter().map(|b| Body1d::new(b.p.z)).collect();

    simulate(&mut bodies, 1000);

    let period_x = simulate_1d(&mut body_x, 1000000u64);
    let period_y = simulate_1d(&mut body_y, 1000000u64);
    let period_z = simulate_1d(&mut body_z, 1000000u64);

    if let (Some(period_x), Some(period_y), Some(period_z)) = (period_x, period_y, period_z) {
        println!(
            "Got periods x = {}, y = {}, z = {}",
            period_x, period_y, period_z
        );

        let tmp = period_x * (period_y / gcd(period_x, period_y));
        let result = tmp * (period_z / gcd(period_z, tmp));

        println!("Common period is {}", result);
    } else {
        println!("Could not identify all periods. Consider increasing max step size.")
    }
}
