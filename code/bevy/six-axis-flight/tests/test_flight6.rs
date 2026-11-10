//! Behaviour tests for the six-axis flight model. Run from this folder: cargo test

use flight6::*;
use std::f32::consts::PI;

const DT: f32 = 1.0 / 60.0;

/// A small deterministic random source (xorshift).
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> f32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 40) as f32 / (1u64 << 24) as f32
    }
    fn pm(&mut self) -> f32 {
        self.next() * 2.0 - 1.0
    }
}

fn wrap(a: f32) -> f32 {
    let mut a = a % (2.0 * PI);
    if a > PI {
        a -= 2.0 * PI;
    }
    if a < -PI {
        a += 2.0 * PI;
    }
    a
}

fn ship() -> (FlyState, Body) {
    let fly = FlyState::new(0.0, &Engine::default(), &Tuning::default());
    let body = Body { pos: V3::ZERO, vel: V3::ZERO, fuel: 100.0 };
    (fly, body)
}

fn tick(fly: &mut FlyState, body: &mut Body, c: Controls) -> FlyOut {
    step6(fly, body, &c, &Ctx::open(DT))
}

fn no() -> Controls {
    Controls::default()
}

#[test]
fn attitude_stays_orthonormal_over_100k_random_ticks() {
    let (mut fly, mut body) = ship();
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    let mut c = no();
    let cap = Tuning::default().speed_cap;
    for i in 0..100_000 {
        if i % 15 == 0 {
            c = Controls {
                pitch: rng.pm(),
                roll: rng.pm(),
                yaw: rng.pm(),
                main: rng.next(),
                retro: rng.next(),
                lift: rng.pm(),
                strafe: rng.pm(),
                boost: rng.next() < 0.2,
                kill_rot: rng.next() < 0.1,
                no_grip: rng.next() < 0.1,
            };
            if rng.next() < 0.05 {
                fly.fa = !fly.fa;
            }
        }
        body.fuel = 100.0;
        tick(&mut fly, &mut body, c);
        let a = fly.att;
        assert!(
            a.f.dot(a.u).abs() < 1e-5 && (a.f.len() - 1.0).abs() < 1e-5 && (a.u.len() - 1.0).abs() < 1e-5,
            "tick {i}: {a:?}"
        );
        let (h, p) = fly.heading_pitch();
        assert!(body.vel.len().is_finite() && h.is_finite() && p.is_finite());
        assert!(fly.charge >= 0.0 && body.vel.len() <= cap + 1e-3);
    }
}

#[test]
fn heading_holds_through_the_vertical() {
    // A pure pitch loop: the heading never turns NaN and only flips by the
    // half turn the nose really makes going over the top and the bottom.
    // While the nose is within the vertical cone the heading is held exactly.
    let mut fly = FlyState::new(0.5, &Engine::default(), &Tuning::default());
    let mut last = 0.5;
    let mut flips = 0;
    let mut held = 0;
    for i in 0..160 {
        let was_vertical = fly.att.f.z.abs() > 0.9995;
        fly.att.rotate(V3::new(0.0, 0.0, 2.5), DT);
        let (h, p) = fly.heading_pitch();
        assert!(h.is_finite() && p.is_finite(), "tick {i}");
        // The heading and pitch always rebuild the nose to within the cone's width.
        let cone = 2.0 * (1.0f32 - 0.9995 * 0.9995).sqrt() + 1e-3;
        assert!(V3::from_angles(h, p).dist(fly.att.f) < cone, "tick {i}");
        let d = wrap(h - last).abs();
        if fly.att.f.z.abs() > 0.9995 {
            assert_eq!(h, last, "held in the cone, tick {i}");
            held += 1;
        } else if d > 1e-3 {
            assert!((d - PI).abs() < 1e-3 && was_vertical, "tick {i}: jump {d}");
            flips += 1;
        }
        last = h;
    }
    assert!(held > 0, "the loop never entered the cone");
    // 160 ticks at 2.5 rad/s is past a full loop: over the top and the bottom.
    assert_eq!(flips, 2);
}

#[test]
fn quaternion_maps_the_local_axes() {
    // q turns local +x onto the nose, +z onto the deck's up, +y onto the left wing.
    let rot = |q: [f32; 4], v: V3| {
        let (x, y, z, w) = (q[0], q[1], q[2], q[3]);
        let qv = V3::new(x, y, z);
        let t = qv.cross(v) * 2.0;
        v + t * w + qv.cross(t)
    };
    let mut rng = Rng(42);
    for _ in 0..500 {
        let mut a = Attitude::from_heading_pitch(rng.pm() * PI, rng.pm() * 1.5);
        a.rotate(V3::new(rng.pm() * PI, 0.0, 0.0), 1.0);
        let q = a.quat();
        let n = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + q[3] * q[3]).sqrt();
        assert!((n - 1.0).abs() < 1e-4, "not unit: {q:?}");
        assert!(rot(q, V3::new(1.0, 0.0, 0.0)).dist(a.f) < 1e-4);
        assert!(rot(q, V3::new(0.0, 0.0, 1.0)).dist(a.u) < 1e-4);
        assert!(rot(q, V3::new(0.0, 1.0, 0.0)).dist(-a.r()) < 1e-4);
    }
}

/// Delivered body acceleration on a test stand (velocity reset each tick), FA OFF.
fn stand(c: Controls, ticks: usize) -> (V3, Spool) {
    let (mut fly, mut body) = ship();
    fly.fa = false;
    for _ in 0..ticks {
        body.vel = V3::ZERO;
        tick(&mut fly, &mut body, c);
    }
    let a = body.vel * (1.0 / DT);
    let at = fly.att;
    (V3::new(a.dot(at.f), a.dot(at.u), a.dot(at.r())), fly.spool)
}

#[test]
fn thrust_table_and_spool() {
    let e = Engine::default();
    // The main spool (tau 0.15 s) reaches 63% at 0.15 s.
    let (_, s) = stand(Controls { main: 1.0, ..no() }, 9);
    let frac = s.x / e.main;
    assert!((frac - 0.63).abs() <= 0.03, "main spool {frac}");
    // Each thruster within 1% of the table after 0.7 s; boost with a
    // translation held jinks, doubling that thruster.
    let cases = [
        (Controls { main: 1.0, ..no() }, V3::new(e.main, 0.0, 0.0)),
        (Controls { retro: 1.0, ..no() }, V3::new(-e.retro, 0.0, 0.0)),
        (Controls { lift: 1.0, ..no() }, V3::new(0.0, e.lift, 0.0)),
        (Controls { lift: -1.0, ..no() }, V3::new(0.0, -e.drop, 0.0)),
        (Controls { strafe: 1.0, ..no() }, V3::new(0.0, 0.0, e.lateral)),
        (Controls { strafe: -1.0, ..no() }, V3::new(0.0, 0.0, -e.lateral)),
        (Controls { boost: true, ..no() }, V3::new(e.boost, 0.0, 0.0)),
        (Controls { lift: 1.0, boost: true, ..no() }, V3::new(0.0, 2.0 * e.lift, 0.0)),
        (Controls { lift: -1.0, boost: true, ..no() }, V3::new(0.0, -2.0 * e.drop, 0.0)),
        (Controls { strafe: 1.0, boost: true, ..no() }, V3::new(0.0, 0.0, 2.0 * e.lateral)),
        (Controls { retro: 1.0, boost: true, ..no() }, V3::new(-2.0 * e.retro, 0.0, 0.0)),
        (Controls { main: 1.0, lift: 1.0, ..no() }, V3::new(e.main, e.lift, 0.0)),
    ];
    for (c, want) in cases {
        let (got, _) = stand(c, 42);
        assert!(got.dist(want) <= 0.01 * want.len(), "{c:?}: {got:?} vs {want:?}");
    }
}

#[test]
fn rotation_rates() {
    let k = Tuning::default();
    // FA ON stops each axis within 0.25 s of release.
    for i in 0..3 {
        let (mut fly, mut body) = ship();
        fly.w = [V3::new(k.rate_max.x, 0.0, 0.0), V3::new(0.0, k.rate_max.y, 0.0), V3::new(0.0, 0.0, k.rate_max.z)][i];
        let rate = |w: V3| [w.x, w.y, w.z][i];
        let mut n = 0;
        while rate(fly.w).abs() > 1e-6 && n < 120 {
            tick(&mut fly, &mut body, no());
            n += 1;
        }
        assert!(n as f32 * DT <= 0.25, "axis {i}: {n} ticks");
    }
    // FA OFF: a released 2.5 rad/s pitch spin coasts about 1.19 s
    // (decay at 15% of the 14 rad/s² pitch acceleration).
    let (mut fly, mut body) = ship();
    fly.fa = false;
    fly.w = V3::new(0.0, 0.0, 2.5);
    let mut n = 0;
    while fly.w.z > 0.0 && n < 600 {
        tick(&mut fly, &mut body, no());
        n += 1;
    }
    assert!((n as f32 * DT - 1.19).abs() <= 0.05, "coast {n} ticks");
    // kill_rot stops it at full authority: 2.5 / 14 s, about 11 ticks.
    fly.w = V3::new(0.0, 0.0, 2.5);
    let mut n = 0;
    while fly.w.z > 0.0 && n < 600 {
        tick(&mut fly, &mut body, Controls { kill_rot: true, ..no() });
        n += 1;
    }
    assert!(n <= 12, "kill_rot {n} ticks");
    assert_eq!(fly.w.z, 0.0);
    // A full pitch stick holds the top rate.
    for _ in 0..60 {
        tick(&mut fly, &mut body, Controls { pitch: 1.0, ..no() });
    }
    assert!((fly.w.z - k.rate_max.z).abs() < 1e-5);
}

#[test]
fn speed_caps() {
    let k = Tuning::default();
    // 60 s of main, lift and strafe from rest stays at the arm speed, FA ON or OFF.
    for fa in [true, false] {
        let (mut fly, mut body) = ship();
        fly.fa = fa;
        let mut top: f32 = 0.0;
        for _ in 0..3600 {
            body.fuel = 100.0;
            tick(&mut fly, &mut body, Controls { main: 1.0, lift: 1.0, strafe: 1.0, ..no() });
            top = top.max(body.vel.len());
        }
        assert!(top <= k.arm_speed + 0.5 && top > 200.0, "fa {fa}: top {top}");
    }
    // Main plus boost reaches nearly the top speed and never passes it.
    let (mut fly, mut body) = ship();
    for _ in 0..1800 {
        body.fuel = 100.0;
        tick(&mut fly, &mut body, Controls { main: 1.0, boost: true, ..no() });
    }
    let s = body.vel.len();
    assert!((k.speed_cap - 8.0..=k.speed_cap).contains(&s), "boost {s}");
    // The gear caps speed at the hover speed.
    let (mut fly, mut body) = ship();
    body.vel = V3::new(200.0, 0.0, 0.0);
    let cx = Ctx { gear: true, ..Ctx::open(DT) };
    step6(&mut fly, &mut body, &no(), &cx);
    assert!(body.vel.len() <= k.hover_speed + 1e-3);
}

#[test]
fn stopping_from_the_arm_speed() {
    let start = |fa: bool| {
        let (mut fly, mut body) = ship();
        body.vel = V3::new(224.0, 0.0, 0.0);
        fly.fa = fa;
        (fly, body)
    };
    // FA ON: retro is the 56 m/s² brake: 4 s and about 448 m.
    let (mut fly, mut body) = start(true);
    let mut t = 0.0;
    while body.vel.len() > 1e-3 && t < 10.0 {
        tick(&mut fly, &mut body, Controls { retro: 1.0, ..no() });
        t += DT;
    }
    let d = body.pos.x;
    assert!((t - 4.0).abs() <= 0.1 && (d - 448.0).abs() <= 15.0, "brake {t} s {d} m");
    // FA OFF: the 30 m/s² retro thruster, about 7.5 s.
    let (mut fly, mut body) = start(false);
    let mut t = 0.0;
    while body.vel.x > 0.0 && t < 20.0 {
        tick(&mut fly, &mut body, Controls { retro: 1.0, ..no() });
        t += DT;
    }
    assert!((t - 7.5).abs() <= 0.2, "retro {t} s");
}

#[test]
fn fuel_rules() {
    let e = Engine::default();
    // 10 s of rotation costs nothing (FA OFF exactly; FA ON at rest too).
    for fa in [false, true] {
        let (mut fly, mut body) = ship();
        fly.fa = fa;
        let mut rng = Rng(7);
        for _ in 0..600 {
            tick(&mut fly, &mut body, Controls { pitch: rng.pm(), roll: rng.pm(), yaw: rng.pm(), ..no() });
        }
        let used = 100.0 - body.fuel;
        assert!(if fa { used < 1e-4 } else { used == 0.0 }, "fa {fa}: {used}");
        assert_eq!(fly.charge, e.charge_cap);
    }
    // 1 s of main from rest, and its spool-down, costs about 5.4.
    let (mut fly, mut body) = ship();
    fly.fa = false;
    let mut tank = 0.0;
    for i in 0..120 {
        let main = if i < 60 { 1.0 } else { 0.0 };
        tank += tick(&mut fly, &mut body, Controls { main, ..no() }).fuel_used;
    }
    assert!((tank - 5.4).abs() <= 0.2, "1 s of main: {tank}");
    assert!((100.0 - body.fuel - tank).abs() < 1e-3);
    // A 0.4 s sideways lift pulse at 224 m/s is paid from the charge, with
    // next to nothing from the tank.
    let (mut fly, mut body) = ship();
    body.vel = V3::new(224.0, 0.0, 0.0);
    fly.fa = false;
    let (mut tank, mut charge) = (0.0, 0.0);
    for i in 0..120 {
        let lift = if i < 24 { 1.0 } else { 0.0 };
        let out = tick(&mut fly, &mut body, Controls { lift, ..no() });
        tank += out.fuel_used;
        charge += out.charge_used;
        assert!(body.vel.len() <= 224.0 + 1e-3);
    }
    assert!(tank < 0.05 && (charge - 12.0).abs() <= 0.5, "pulse: tank {tank} charge {charge}");
    // Retro along the velocity, fast, pays the tank instead of the charge.
    let (mut fly, mut body) = ship();
    body.vel = V3::new(100.0, 0.0, 0.0);
    fly.fa = false;
    let (mut tank, mut charge) = (0.0, 0.0);
    for _ in 0..30 {
        let out = tick(&mut fly, &mut body, Controls { retro: 1.0, ..no() });
        tank += out.fuel_used;
        charge += out.charge_used;
    }
    let dv = 100.0 - body.vel.x;
    assert!(charge < 1e-4 && (tank - 0.1 * dv).abs() < 0.02, "retro: tank {tank} charge {charge} dv {dv}");
    // Holding the lift drains the charge toward 0, never below; then the tank pays.
    let (mut fly, mut body) = ship();
    fly.fa = false;
    let mut late_tank = 0.0;
    for i in 0..1200 {
        body.vel = V3::ZERO;
        let out = tick(&mut fly, &mut body, Controls { lift: 1.0, ..no() });
        assert!(fly.charge >= 0.0);
        if i > 600 {
            late_tank += out.fuel_used;
        }
    }
    // At the end the charge only holds one tick's refill.
    assert!(fly.charge <= e.charge_refill * DT + 1e-4 && late_tank > 1.0, "charge {} tank {late_tank}", fly.charge);
    // The charge refills while there is fuel, and not with a dry tank.
    let (mut fly, mut body) = ship();
    fly.charge = 0.0;
    tick(&mut fly, &mut body, no());
    assert!((fly.charge - e.charge_refill * DT).abs() < 1e-6, "refill {}", fly.charge);
    fly.charge = 0.0;
    body.fuel = 0.0;
    tick(&mut fly, &mut body, no());
    assert_eq!(fly.charge, 0.0);
    // Boost: the speed it adds, plus its burn per second.
    let k = Tuning::default();
    let (mut fly, mut body) = ship();
    fly.fa = false;
    let out = tick(&mut fly, &mut body, Controls { boost: true, ..no() });
    let want = e.boost * DT * k.fuel_per_dv + k.boost_burn * DT;
    assert!((out.fuel_used - want).abs() < 1e-5, "boost {} vs {want}", out.fuel_used);
    // Going faster than the arm speed burns fuel just to keep going; at it, none.
    for (v, want) in [(k.arm_speed, 0.0), (336.0, 0.5 * k.over_burn * DT)] {
        let (mut fly, mut body) = ship();
        fly.fa = false;
        body.vel = V3::new(v, 0.0, 0.0);
        let out = tick(&mut fly, &mut body, no());
        assert!((out.fuel_used - want).abs() < 1e-6, "at {v}: {} vs {want}", out.fuel_used);
    }
}

/// A pad 50 m down with a 9.8 m/s² field pulling down, no other gravity.
fn pad_ctx(h: f32, gear: bool) -> Ctx {
    Ctx {
        field: HoverField { accel: V3::new(0.0, 0.0, -9.8), fade: 1.0, h },
        gear,
        ..Ctx::open(DT)
    }
}

#[test]
fn hover_assist_holds_still_hands_off() {
    let (mut fly, mut body) = ship();
    body.pos = V3::new(0.0, 0.0, 50.0);
    let fuel0 = body.fuel;
    for i in 0..3600 {
        let cx = pad_ctx(body.pos.z, false);
        let out = step6(&mut fly, &mut body, &no(), &cx);
        assert!(out.hover_assist, "tick {i}");
    }
    let drift = body.pos.dist(V3::new(0.0, 0.0, 50.0));
    assert!(drift <= 0.5, "drift {drift}");
    assert!(fly.att.tilt_from(V3::UP) < 0.2);
    assert!(fuel0 - body.fuel < 1.0, "fuel {}", fuel0 - body.fuel);
}

#[test]
fn hover_stick_sets_a_ground_speed() {
    // Pitch forward (stick down) flies forward at up to hover_vh.
    let k = Tuning::default();
    let (mut fly, mut body) = ship();
    body.pos = V3::new(0.0, 0.0, 50.0);
    for _ in 0..600 {
        let cx = pad_ctx(body.pos.z, false);
        step6(&mut fly, &mut body, &Controls { pitch: -1.0, ..no() }, &cx);
    }
    let vh = V3::new(body.vel.x, body.vel.y, 0.0);
    assert!((vh.len() - k.hover_vh).abs() < 1.5 && body.vel.x > 0.0, "{:?}", body.vel);
    assert!(body.vel.z.abs() < 1.0, "height drifts {}", body.vel.z);
    assert!(fly.att.tilt_from(V3::UP) <= k.hover_tilt + 0.05);
}

#[test]
fn soft_touch_slows_the_last_metres() {
    // Gear down, full sink from 15 m: the descent is held to max(soft_min, h/5).
    // It is a target, not a wall: coming in fast from higher, it brakes over a few metres.
    let k = Tuning::default();
    let (mut fly, mut body) = ship();
    body.pos = V3::new(0.0, 0.0, 15.0);
    for _ in 0..1200 {
        let h = body.pos.z;
        if h <= 0.5 {
            break;
        }
        step6(&mut fly, &mut body, &Controls { lift: -1.0, ..no() }, &pad_ctx(h, true));
        let most = k.soft_min.max(h / 5.0) + 0.6;
        assert!(-body.vel.z <= most, "h {h}: sinking {}", -body.vel.z);
    }
    assert!(body.pos.z <= 0.5, "never got down: {}", body.pos.z);
}

#[test]
fn main_thrust_punches_out_of_the_hover() {
    let k = Tuning::default();
    let (mut fly, mut body) = ship();
    body.pos = V3::new(0.0, 0.0, 50.0);
    let mut left = false;
    for _ in 0..600 {
        let cx = pad_ctx(body.pos.z, false);
        let out = step6(&mut fly, &mut body, &Controls { main: 1.0, ..no() }, &cx);
        left |= !out.hover_assist;
    }
    assert!(left && body.vel.len() > 2.0 * k.hover_speed, "speed {}", body.vel.len());
    // Below punch_min_h, main thrust stays in the hover.
    let (mut fly, mut body) = ship();
    body.pos = V3::new(0.0, 0.0, 5.0);
    for _ in 0..30 {
        let out = step6(&mut fly, &mut body, &Controls { main: 1.0, ..no() }, &pad_ctx(5.0, false));
        assert!(out.hover_assist);
    }
}

#[test]
fn takeover_in_a_field_holds_height() {
    // Taking the stick 200 m over a pad (a 5 m/s² field) with the thrusters
    // cold: Flight Assist on loses at most 1 m in 2 s, since the hover assist
    // starts the lift at what holds the ship; off, with the lever at the
    // mark, at most 3 m while it spools up.
    for (fa, most) in [(true, 1.0), (false, 3.0)] {
        let (mut fly, mut body) = ship();
        fly.fa = fa;
        body.pos = V3::new(0.0, 0.0, 200.0);
        let cx = Ctx { field: HoverField { accel: V3::new(0.0, 0.0, -5.0), fade: 1.0, h: 200.0 }, ..Ctx::open(DT) };
        for _ in 0..120 {
            step6(&mut fly, &mut body, &no(), &cx);
        }
        let dh = (body.pos.z - 200.0).abs();
        assert!(dh <= most, "fa {fa}: {dh} m");
    }
}

#[test]
fn the_field_pulls_by_its_fade() {
    let k = Tuning::default();
    let field = |fade: f32| Ctx { field: HoverField { accel: V3::new(0.0, 0.0, -8.0), fade, h: 300.0 }, ..Ctx::open(DT) };
    // FA off, lever down: one tick of pull is accel × fade.
    let (mut fly, mut body) = ship();
    fly.fa = false;
    fly.lever = 0.0;
    step6(&mut fly, &mut body, &no(), &field(0.5));
    assert!((body.vel.z + 8.0 * 0.5 * DT).abs() < 1e-6, "{}", body.vel.z);
    // FA off, lever at the mark: holds height at half strength too.
    let (mut fly, mut body) = ship();
    fly.fa = false;
    for _ in 0..120 {
        step6(&mut fly, &mut body, &no(), &field(0.5));
    }
    assert!(body.pos.z.abs() < 2.0, "lever at fade 0.5: {}", body.pos.z);
    // The hover assist starts only above hover_fade; under it FA on cancels the pull.
    let (mut fly, mut body) = ship();
    assert!(step6(&mut fly, &mut body, &no(), &field(k.hover_fade + 0.1)).hover_assist);
    let (mut fly, mut body) = ship();
    for _ in 0..60 {
        assert!(!step6(&mut fly, &mut body, &no(), &field(k.hover_fade - 0.1)).hover_assist);
    }
    assert!(body.vel.len() < 0.1, "open-space FA on should cancel the field: {:?}", body.vel);
}

#[test]
fn hover_lean_is_capped() {
    // With a small hover_tilt, a full push leans no further than the cap.
    let mut cx = pad_ctx(50.0, false);
    cx.tuning.hover_tilt = 20f32.to_radians();
    let (mut fly, mut body) = ship();
    body.pos = V3::new(0.0, 0.0, 50.0);
    let mut most: f32 = 0.0;
    for _ in 0..300 {
        cx.field.h = body.pos.z;
        step6(&mut fly, &mut body, &Controls { pitch: -1.0, ..no() }, &cx);
        most = most.max(fly.att.tilt_from(V3::UP));
    }
    assert!(most <= 20f32.to_radians() + 0.03, "leaned {most} rad");
    assert!(most > 15f32.to_radians(), "never leaned: {most} rad");
}

#[test]
fn grip_waits_after_a_translation() {
    // FA on: the grip lets go while strafing and for grip_hold_s after.
    let k = Tuning::default();
    let (mut fly, mut body) = ship();
    body.vel = V3::new(112.0, 0.0, 0.0);
    assert!(!tick(&mut fly, &mut body, Controls { strafe: 1.0, ..no() }).grip_on);
    let hold = (k.grip_hold_s / DT).round() as usize;
    for i in 0..hold - 1 {
        assert!(!tick(&mut fly, &mut body, no()).grip_on, "tick {i}");
    }
    for _ in 0..3 {
        tick(&mut fly, &mut body, no());
    }
    assert!(tick(&mut fly, &mut body, no()).grip_on);
}

#[test]
fn a_jink_can_pass_the_arm_speed() {
    // A strafe jink isn't held to the arm speed; a plain strafe is.
    let k = Tuning::default();
    for (boost, faster) in [(true, true), (false, false)] {
        let (mut fly, mut body) = ship();
        fly.fa = false;
        body.vel = V3::new(k.arm_speed, 0.0, 0.0);
        for _ in 0..60 {
            tick(&mut fly, &mut body, Controls { strafe: 1.0, boost, ..no() });
        }
        let s = body.vel.len();
        assert_eq!(s > k.arm_speed + 1.0, faster, "boost {boost}: {s}");
    }
}

#[test]
fn half_turn_flip_takes_about_1_44_s() {
    // FA off, at rest: a bang-bang 180° pitch flip (top rate 2.5, 14 rad/s²).
    let k = Tuning::default();
    let (mut fly, mut body) = ship();
    fly.fa = false;
    let f0 = fly.att.f;
    let accel = k.rate_accel.z;
    let (mut t, mut braking) = (0.0, false);
    loop {
        let ang = fly.att.f.angle_to(f0);
        let wz = fly.w.z;
        let stick = if !braking {
            if PI - ang <= wz * wz / (2.0 * accel) + wz * DT * 0.5 {
                braking = true;
                -1.0
            } else {
                1.0
            }
        } else if wz > 0.0 {
            -1.0
        } else {
            0.0
        };
        if braking && wz <= 0.0 {
            assert!((t - 1.44f32).abs() <= 0.05 && (ang - PI).abs() < 0.05, "flip {t} s, {ang} rad");
            break;
        }
        assert!(t < 10.0);
        tick(&mut fly, &mut body, Controls { pitch: stick, ..no() });
        t += DT;
    }
}

#[test]
fn level_out_keeps_the_heading_near_vertical() {
    // Fly on a heading, then point the nose almost straight up: level-out
    // comes back down on the heading the ship was flying.
    let k = Tuning::default();
    let h = -1.467;
    let (mut fly, mut body) = ship();
    fly.att = Attitude::from_heading_pitch(h, 1.0);
    tick(&mut fly, &mut body, no());
    fly.att = Attitude::from_heading_pitch(h, std::f32::consts::FRAC_PI_2 - 0.5f32.to_radians());
    tick(&mut fly, &mut body, no());
    let mut n = 0;
    while !level_out_step(&mut fly, &mut body, V3::ZERO, &k, DT) {
        n += 1;
        assert!(n < 200);
    }
    let (h1, _) = fly.heading_pitch();
    assert!(wrap(h1 - h).abs() < 0.01, "ended on {h1}, flew {h}");
}

#[test]
fn a_dry_tank_turns_but_doesnt_thrust() {
    let (mut fly, mut body) = ship();
    body.fuel = 0.0;
    body.vel = V3::new(10.0, 0.0, 0.0);
    let f0 = fly.att.f;
    let g = V3::new(0.0, 0.0, -3.0);
    let cx = Ctx { gravity: g, ..pad_ctx(50.0, false) };
    for _ in 0..60 {
        let v0 = body.vel;
        let out = step6(&mut fly, &mut body, &Controls { main: 1.0, lift: 1.0, pitch: 1.0, boost: true, ..no() }, &cx);
        // Only gravity acts; the field and every thruster are off.
        assert!((body.vel - v0 - g * DT).len() < 1e-5);
        assert!(!out.hover_assist && !out.boosting && out.fuel_used == 0.0);
    }
    assert!(fly.att.f.angle_to(f0) > 0.3);
}

#[test]
fn fa_on_flies_like_an_arcade_ship() {
    // 3 s of main adds over 50 m/s; 1 s of right yaw lowers the heading by over 0.3 rad.
    let (mut fly, mut body) = ship();
    for _ in 0..180 {
        tick(&mut fly, &mut body, Controls { main: 1.0, ..no() });
    }
    assert!(body.vel.len() > 50.0, "{}", body.vel.len());
    let (h0, _) = fly.heading_pitch();
    for _ in 0..60 {
        tick(&mut fly, &mut body, Controls { yaw: 1.0, ..no() });
    }
    let (h1, _) = fly.heading_pitch();
    assert!(wrap(h0 - h1) > 0.3, "a right yaw lowers the heading");
    // Carving at 112 m/s keeps the course on the nose.
    let (mut fly, mut body) = ship();
    body.vel = V3::new(112.0, 0.0, 0.0);
    for _ in 0..180 {
        tick(&mut fly, &mut body, Controls { yaw: 0.25, ..no() });
    }
    let off = body.vel.angle_to(fly.att.f);
    assert!(off < 5f32.to_radians(), "course {off} rad off the nose");
    // With the grip off, the same carve slides.
    let (mut fly, mut body) = ship();
    body.vel = V3::new(112.0, 0.0, 0.0);
    for _ in 0..180 {
        tick(&mut fly, &mut body, Controls { yaw: 0.25, no_grip: true, ..no() });
    }
    assert!(body.vel.angle_to(fly.att.f) > 0.3);
    // A banked ship coordinates (yaws toward the low wing) and levels itself.
    let (mut fly, mut body) = ship();
    for _ in 0..20 {
        tick(&mut fly, &mut body, Controls { roll: 1.0, ..no() });
    }
    assert!(fly.att.bank() > 0.5);
    for _ in 0..30 {
        tick(&mut fly, &mut body, no());
    }
    assert!(fly.w.y > 0.1);
    for _ in 0..300 {
        tick(&mut fly, &mut body, no());
    }
    assert!(fly.att.bank().abs() < 0.01);
}

#[test]
fn level_out_from_upside_down_and_steep() {
    let k = Tuning::default();
    let (mut fly, mut body) = ship();
    body.vel = V3::new(30.0, 0.0, 0.0);
    let mut a = Attitude::from_heading_pitch(0.3, 1.5);
    a.rotate(V3::new(PI, 0.0, 0.0), 1.0);
    fly.att = a;
    assert!(!fly.att.is_level(k.level_max_pitch) && fly.att.bank().abs() > 3.0);
    let mut n = 0;
    while !level_out_step(&mut fly, &mut body, V3::ZERO, &k, DT) {
        n += 1;
        assert!(n < 200);
    }
    let secs = (n + 1) as f32 * DT;
    // Finished on its own (no snap) within 1.5 s.
    assert!(secs <= 1.5 && fly.level_t < k.level_snap_s, "{secs} s");
    assert!(fly.att.is_level(k.level_max_pitch) && !fly.levelling && fly.w == V3::ZERO);
    let (_, p) = fly.heading_pitch();
    assert!((p - k.level_max_pitch).abs() < 1e-4 && fly.att.bank().abs() < 1e-4, "pitch {p}");
    // The ship coasted.
    assert!((body.vel - V3::new(30.0, 0.0, 0.0)).len() < 1e-3);
    // Past the time limit it snaps level.
    fly.att.rotate(V3::new(1.0, 0.0, 0.0), 1.0);
    fly.level_t = k.level_snap_s - 0.5 * DT;
    assert!(level_out_step(&mut fly, &mut body, V3::ZERO, &k, DT));
    assert!(fly.att.is_level(k.level_max_pitch) && fly.level_t >= k.level_snap_s);
}

#[test]
fn helpers() {
    let k = Tuning::default();
    let x = V3::new(1.0, 0.0, 0.0);
    assert!((arm(V3::new(112.0, 0.0, 0.0), x, k.arm_speed) - 0.5).abs() < 1e-6);
    assert_eq!(arm(V3::new(-50.0, 0.0, 0.0), x, k.arm_speed), 1.0);
    assert_eq!(arm(V3::new(300.0, 0.0, 0.0), x, k.arm_speed), 0.0);
    assert_eq!(over_burn(k.arm_speed, &k), 0.0);
    assert!((over_burn(k.speed_cap, &k) - k.over_burn).abs() < 1e-6);
    assert_eq!(field_fade(5.0, 10.0, 40.0), 1.0);
    assert_eq!(field_fade(50.0, 10.0, 40.0), 0.0);
    assert!((field_fade(25.0, 10.0, 40.0) - 0.5).abs() < 1e-6);
    // A zero dt changes nothing.
    let (mut fly, mut body) = ship();
    let before = (fly, body);
    step6(&mut fly, &mut body, &Controls { main: 1.0, ..no() }, &Ctx::open(0.0));
    assert_eq!((fly, body), before);
}

#[test]
fn impact_point_on_the_pad_plane() {
    let g = V3::new(0.0, 0.0, -5.0);
    let p = impact_point(V3::new(10.0, 0.0, 100.0), V3::new(5.0, 0.0, -10.0), g).unwrap();
    let t = (-10.0 + (100.0f32 + 1000.0).sqrt()) / 5.0;
    assert!((p.x - (10.0 + 5.0 * t)).abs() < 1e-3 && p.z == 0.0);
    // Straight-line fall, no acceleration.
    let p = impact_point(V3::new(0.0, 4.0, 20.0), V3::new(0.0, 1.0, -2.0), V3::ZERO).unwrap();
    assert!((p.y - 14.0).abs() < 1e-4);
    // Rising away with nothing pulling down: none; below the plane: none.
    assert!(impact_point(V3::new(0.0, 0.0, 20.0), V3::new(0.0, 0.0, 3.0), V3::ZERO).is_none());
    assert!(impact_point(V3::new(0.0, 0.0, -1.0), V3::ZERO, g).is_none());
    // Climbing now, but the field brings it down.
    assert!(impact_point(V3::new(0.0, 0.0, 20.0), V3::new(0.0, 0.0, 3.0), g).is_some());
}
