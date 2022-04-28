use std::f32::consts::PI;

use rand::random;

#[inline]
fn r2(x: f32, y: f32) -> f32 {
    x*x + y*y
}
#[inline]
fn r(x: f32, y: f32) -> f32 {
    r2(x, y).sqrt()
}
#[inline]
fn theta(x: f32, y: f32) -> f32 {
    x.atan2(y)
}
#[inline]
fn phi(x: f32, y: f32) -> f32 {
    y.atan2(x)
}
#[inline]
fn O() -> f32 {
    if random() {0.} else {PI}
}
#[inline]
fn L() -> f32 {
    if random() {-1.} else {1.}
}
#[inline]
fn P() -> f32 {
    random()
}

pub fn v0(x: f32, y: f32) -> (f32, f32) {
    (x, y)
}

pub fn v1(x: f32, y: f32) -> (f32, f32) {
    (x.sin(), y.sin())
}

pub fn v2(x: f32, y: f32) -> (f32, f32) {
    let inv_r2 = 1. / r2(x, y);

    (inv_r2*x, inv_r2*y)
}

pub fn v3(x: f32, y: f32) -> (f32, f32) {
    let (r2_sin, r2_cos) = r2(x, y).sin_cos();

    (x*r2_sin - y*r2_cos, x*r2_cos + y*r2_sin)
}

pub fn v4(x: f32, y: f32) -> (f32, f32) {
    let inv_r = 1./r(x, y);

    (inv_r*(x - y)*(x + y), 2.*inv_r*x*y)
}

pub fn v5(x: f32, y: f32) -> (f32, f32) {
    (theta(x, y)/PI, r(x,y) - 1.)
}

pub fn v6(x: f32, y: f32) -> (f32, f32) {
    let r = r(x, y);
    let theta = theta(x, y);

    ((theta + r).sin() * r, (theta - r).cos() * r)
}

pub fn v7(x: f32, y: f32) -> (f32, f32) {
    let r = r(x, y);
    let theta = theta(x, y);
    let (r_theta_sin, r_theta_cos) = (r * theta).sin_cos();

    (r*r_theta_sin, -r*r_theta_cos)
}

pub fn v8(x: f32, y: f32) -> (f32, f32) {
    let r = r(x, y);
    let theta_over_pi = theta(x, y) / PI;
    let (pi_r_sin, pi_r_cos) = (r * PI).sin_cos();

    (theta_over_pi*pi_r_sin, theta_over_pi*pi_r_cos)
}

pub fn v9(x: f32, y: f32) -> (f32, f32) {
    let r = r(x, y);
    let inv_r = 1./r;
    let (theta_sin, theta_cos) = theta(x, y).sin_cos();
    let (r_sin, r_cos) = r.sin_cos();

    (inv_r *(theta_cos + r_sin), inv_r * (theta_sin - r_cos))
}

pub fn v10(x: f32, y: f32) -> (f32, f32) {
    let r = r(x, y);
    let (theta_sin, theta_cos) = theta(x, y).sin_cos();

    (theta_sin / r, r * theta_cos)
}

pub fn v11(x: f32, y: f32) -> (f32, f32) {
    let (theta_sin, theta_cos) = theta(x, y).sin_cos();
    let (r_sin, r_cos) = r(x,y).sin_cos();

    (theta_sin * r_cos, theta_cos * r_sin)
}


