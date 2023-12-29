// alright, two days left to go, let's do this!
// nice observation: seems like the velocity vectors don't ever
// have a component equal to zero, so we can derive general
// formulas without having to worry about dividing by zero.

// i really hope we don't run into floating point precision bugs.

// ooh, part 2 is very different from what I expected! Isn't this simple,
// though? unless there are FP bugs, of course.
// wait, it's not. could be tricky after all.

// okay, thought about this a bit, and there should be an analytical
// solution using only three rays, which leads to an equation system
// that has 9 variables (px, py, pz, vx, vy, vz, t1, t2 and t2) and 9
// equations. Some manual transformation leads to a linear equation
// system that has all times eliminated, so we could use Gaussian
// elimination. Unfortunately, I'm a little worried about numeric
// stability, given that the input data is already at values up to
// ~ 4 * 10^14. I guess if we ensure proper pivoting, it should
// hopefully be fine.

// random appearance of a cross product, funny. Probably not random,
// but I'm too bad at math to understand why it's not.

// awesome. of course we run into precision bugs. ugh.
// guess I'll have to implement double-double arithmetic if I want
// to continue using this approach? Let's figure out how that works.

// you know what? No. I'm sure learing about that would be interesting,
// but I solved the main problem, so I should not have to deal with this.
// We'll use an external library here.

// scratch everything. turns out if we translate the coordinate system
// towards (0, 0, 0), the numeric error is *just* small enough to be
// okay. Not a pretty solution, but it worked for my input, so I'll
// take it.

mod math;

use std::{str::FromStr, error::Error};
use std::fmt::Debug;

use aoc::{ParseLineError, PuzzlePart};
use math::{Vector, Vec2f, Vec3f, Mat2x3};

use twofloat::TwoFloat;

const XY_PLANE: Mat2x3 = Mat2x3 {
    a00: 1.0,
    a01: 0.0,
    a02: 0.0,
    a10: 0.0,
    a11: 1.0,
    a12: 0.0,
};

const TEST_LOWER: f64 = 200000000000000.0;
const TEST_UPPER: f64 = 400000000000000.0;

#[derive(Debug)]
struct Ray<V: Vector + Debug> {
    start: V,
    velocity: V,
}

impl Ray<Vec2f> {
    fn get_intersection(&self, other: &Self) -> Option<(f64, f64)> {
        if self.velocity.is_parallel(&other.velocity) {
            return None;
        }
        let a = &self.start;
        let b = &other.start;
        let u = &self.velocity;
        let v = &other.velocity;
        let t1 = (v.x * (a.y - b.y) - v.y * (a.x - b.x)) / (u.x * v.y - u.y * v.x);
        let t2 = (u.x * (b.y - a.y) - u.y * (b.x - a.x)) / (v.x * u.y - v.y * u.x);
        Some((t1, t2))
    }
}

impl Ray<Vec3f> {

    fn project(&self, mat: &Mat2x3) -> Ray<Vec2f> {
        Ray {
            start: self.start.project(mat),
            velocity: self.velocity.project(mat),
        }
    }

    fn get_intersection(&self, other: &Self) -> Option<(f64, f64)> {
        let self_projection = self.project(&XY_PLANE);
        let other_projection = other.project(&XY_PLANE);
        let res = self_projection.get_intersection(&other_projection);
        if let Some((t1, t2)) = res {
            let z1 = &self.start + &(&self.velocity * t1);
            let z2 = &other.start + &(&other.velocity * t2);
            if z1 == z2 {
                return res;
            }
        }
        None
    }

}

impl FromStr for Vec3f {
    type Err = ParseLineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(",");
        let val: Vec<f64> = split.filter_map(|v| {
            match v.parse() {
                Ok(i) => Some(i),
                Err(_) => None,
            }
        }).collect();
        if val.len() != 3 {
            return Err(ParseLineError::new("Vec3f", s));
        }
        Ok(Vec3f { x: val[0], y: val[1], z: val[2] })
    }
}

impl FromStr for Ray<Vec3f> {
    type Err = ParseLineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace(" ", "");
        let split: Vec<&str> = s.split("@").collect();
        if split.len() != 2 {
            return Err(ParseLineError::new("Ray", &s));
        }
        let start: Vec3f = split[0].parse()?;
        let velocity: Vec3f = split[1].parse()?;
        Ok(Ray { start, velocity })
    }
}

fn part1(rays: &Vec<Ray<Vec3f>>) {
    let rays: Vec<Ray<Vec2f>> = rays.iter().map(|r| r.project(&XY_PLANE)).collect();
    let mut amount = 0;
    for i in 0..rays.len() {
        for j in i + 1..rays.len() {
            let a = &rays[i];
            let b = &rays[j];
            let intersection = a.get_intersection(b);
            if let Some((t1, t2)) = intersection {
                if t1 < 0.0 || t2 < 0.0 {
                    continue;
                }
                let p = &a.start + &(&a.velocity * t1);
                if p.x >= TEST_LOWER && p.y >= TEST_LOWER && p.x <= TEST_UPPER && p.y <= TEST_UPPER {
                    amount += 1;
                }
            }
        }
    }
    println!("intersections: {}", amount);
}

fn gauss_eliminate(input: &mut Vec<Vec<f64>>) -> Vec<f64> {
    let mut i = 0;
    let mut backwards: Vec<Vec<f64>> = Vec::new();
    while input.len() > 1 {
        input.sort_by(|a, b| a[i].abs().total_cmp(&b[i].abs()));
        //println!("{:?}", input);
        let cur = input.pop().unwrap();
        input.iter_mut().for_each(|v| {
            let f = v[i] / cur[i];
            for j in i..v.len() {
                v[j] -= f * cur[j];
            }
        });
        backwards.push(cur);
        i += 1;
    }
    backwards.push(input.pop().unwrap());
    //println!("{:?}", backwards);
    let mut solution: Vec<f64> = vec![0.0; i + 1];
    while let Some(mut v) = backwards.pop() {
        let mut res = v.pop().unwrap();
        for k in 0..v.len() {
            res -= v[k] * solution[k];
        }
        solution[i] = res / v[i];
        if i > 0 {
            i -= 1;
        }
    }
    solution
}

fn part2(input: &Vec<Ray<Vec3f>>) {
    let ray1 = &input[0];
    let ray2 = &input[1];
    let ray3 = &input[2];
    let pp1 = &ray1.start;
    let pp2 = &ray2.start;
    let pp3 = &ray3.start;
    let min_x = pp1.x.min(pp2.x.min(pp3.x));
    let min_y = pp1.y.min(pp2.y.min(pp3.y));
    let min_z = pp1.z.min(pp2.z.min(pp3.z));
    let new_origin = Vec3f{ x: min_x, y: min_y, z: min_z };
    //let new_origin = Vec3f{ x: 0.0, y: 0.0, z: 0.0 };
    println!("{:?}", new_origin);
    let p1 = &(pp1 - &new_origin);
    let p2 = &(pp2 - &new_origin);
    let p3 = &(pp3 - &new_origin);
    let q1 = &ray1.velocity;
    let q2 = &ray2.velocity;
    let q3 = &ray3.velocity;
    let cr1 = p1.cross(&q1);
    let cr2 = p2.cross(&q2);
    let cr3 = p3.cross(&q3);
    let a1 = p1 - p3;
    let a2 = p2 - p3;
    let b1 = q1 - q3;
    let b2 = q2 - q3;
    let c1 = &cr1 - &cr3;
    let c2 = &cr2 - &cr3;
    let mut input = vec![
        vec![b1.y, -b1.x, 0.0, -a1.y, a1.x, 0.0, c1.z],
        vec![b2.y, -b2.x, 0.0, -a2.y, a2.x, 0.0, c2.z],
        vec![0.0, b1.z, -b1.y, 0.0, -a1.z, a1.y, c1.x],
        vec![0.0, b2.z, -b2.y, 0.0, -a2.z, a2.y, c2.x],
        vec![-b1.z, 0.0, b1.x, a1.z, 0.0, -a1.x, c1.y],
        vec![-b2.z, 0.0, b2.x, a2.z, 0.0, -a2.x, c2.y],
    ];
    let res = gauss_eliminate(&mut input);
    /* let mut input_128: Vec<Vec<TwoFloat>> = input.iter().map(|v| {
        v.iter().map(|i| TwoFloat::from(*i)).collect()
    }).collect();
    let cr1z = TwoFloat::from(p1.x) * TwoFloat::from(q1.y) - TwoFloat::from(p1.y) * TwoFloat::from(q1.x);
    let cr1x = TwoFloat::from(p1.y) * TwoFloat::from(q1.z) - TwoFloat::from(p1.z) * TwoFloat::from(q1.y);
    let cr1y = TwoFloat::from(p1.z) * TwoFloat::from(q1.x) - TwoFloat::from(p1.x) * TwoFloat::from(q1.z);
    let cr2z = TwoFloat::from(p2.x) * TwoFloat::from(q2.y) - TwoFloat::from(p2.y) * TwoFloat::from(q2.x);
    let cr2x = TwoFloat::from(p2.y) * TwoFloat::from(q2.z) - TwoFloat::from(p2.z) * TwoFloat::from(q2.y);
    let cr2y = TwoFloat::from(p2.z) * TwoFloat::from(q2.x) - TwoFloat::from(p2.x) * TwoFloat::from(q2.z);
    let cr3z = TwoFloat::from(p3.x) * TwoFloat::from(q3.y) - TwoFloat::from(p3.y) * TwoFloat::from(q3.x);
    let cr3x = TwoFloat::from(p3.y) * TwoFloat::from(q3.z) - TwoFloat::from(p3.z) * TwoFloat::from(q3.y);
    let cr3y = TwoFloat::from(p3.z) * TwoFloat::from(q3.x) - TwoFloat::from(p3.x) * TwoFloat::from(q3.z);
    input_128[0][6] = cr1z - cr3z;
    input_128[1][6] = cr2z - cr3z;
    input_128[2][6] = cr1x - cr3x;
    input_128[3][6] = cr2x - cr3x;
    input_128[4][6] = cr1y - cr3y;
    input_128[5][6] = cr2y - cr3y;
    let res = 1; */
    //let res = gauss_eliminate_128(&mut input_128);
    println!("{:?}", res);
    let res_vec = Vec3f { x: res[0], y: res[1], z: res[2] };
    let total = &res_vec + &new_origin;
    println!("{:?}, {}", total, total.x + total.y + total.z);
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = aoc::read_input()?;
    let test: f64 = -2.25;
    println!("{}, {}", test.trunc(), test.fract());
    let rays: Vec<Ray<Vec3f>> = input.iter().filter_map(|line| {
        match line.parse() {
            Ok(r) => Some(r),
            Err(_) => None,
        }
    }).collect();
    //println!("{:?}", rays);
    match aoc::puzzle_part() {
        PuzzlePart::PartOne => part1(&rays),
        PuzzlePart::PartTwo => part2(&rays),
    }
    Ok(())
}
