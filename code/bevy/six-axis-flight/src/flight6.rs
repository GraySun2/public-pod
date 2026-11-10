//! Six-axis flight: a ship that pitches, rolls and yaws freely and
//! thrusts along all three of its axes, with Flight Assist.
//!
//! - **Flight Assist ON, open space:** flies like an arcade ship. The velocity
//!   is gripped onto the nose, drag and a brake slow it, a held bank turns the
//!   ship (auto-coordination), and the wings level themselves when the roll
//!   stick is let go.
//! - **Flight Assist ON, in a hover field:** a drone-style hover. The stick
//!   sets a speed over the ground, lift sets a climb or sink, and the ship
//!   leans into the push and holds still with the stick centred.
//! - **Flight Assist OFF:** pure momentum. Every thruster pushes as it is, a
//!   spin keeps going when the stick is let go (slowly decaying), and a
//!   collective lever holds the ship against a hover field.
//!
//! Plus a boost along the nose, jinks (a doubled small-thruster push), a
//! rechargeable charge for the small thrusters, fuel paid by speed change,
//! a level-out helper and a ballistic impact point for a landing HUD.
//!
//! Engine-agnostic and dependency-free (Rust std only). Call [`step6`] once a
//! fixed tick from your game loop (a Bevy `FixedUpdate` system, for example)
//! and copy the result onto your transform.
//!
//! Conventions:
//! - World axes: z is up. Heading turns about z, pitch tilts the nose up.
//! - Body rates `w = V3 { x: roll, y: yaw, z: pitch }` in rad/s; positive is
//!   right wing down, nose right, nose up.
//! - Ship-local axes: x forward (`f`), y up (`u`), z right (`r = f × u`).
//!   World angular velocity is `ω = f·roll − u·yaw + r·pitch`. Each tick `f`
//!   and `u` turn by `ω·dt` (Rodrigues) and are re-orthonormalised, so they
//!   never drift apart.
//! - Bank `φ = atan2(−r.z, u.z)`.
//!
//! Every number in [`Tuning`] and [`Engine`] is a first guess from one
//! prototype; tune them for your game.

use std::f32::consts::{FRAC_PI_2, PI};
use std::ops::{Add, Mul, Neg, Sub};

// ---------------------------------------------------------------------------
// Vectors.

/// A 3D vector (z up).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct V3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl V3 {
    pub const ZERO: V3 = V3::new(0.0, 0.0, 0.0);
    pub const UP: V3 = V3::new(0.0, 0.0, 1.0);

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        V3 { x, y, z }
    }
    /// The unit vector at `heading` (about z, from +x) and `pitch` (up from
    /// the xy plane).
    pub fn from_angles(heading: f32, pitch: f32) -> Self {
        V3::new(
            pitch.cos() * heading.cos(),
            pitch.cos() * heading.sin(),
            pitch.sin(),
        )
    }
    pub fn dot(self, o: V3) -> f32 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }
    pub fn cross(self, o: V3) -> V3 {
        V3::new(
            self.y * o.z - self.z * o.y,
            self.z * o.x - self.x * o.z,
            self.x * o.y - self.y * o.x,
        )
    }
    pub fn len(self) -> f32 {
        self.dot(self).sqrt()
    }
    /// The unit vector along `self`, or zero for a zero vector.
    pub fn norm(self) -> V3 {
        let l = self.len();
        if l > 1e-6 { self * (1.0 / l) } else { V3::ZERO }
    }
    pub fn dist(self, o: V3) -> f32 {
        (self - o).len()
    }
    /// The angle between two vectors (rad), exact near 0.
    pub fn angle_to(self, o: V3) -> f32 {
        let (a, b) = (self.norm(), o.norm());
        a.cross(b).len().atan2(a.dot(b))
    }
}

impl Add for V3 {
    type Output = V3;
    fn add(self, o: V3) -> V3 {
        V3::new(self.x + o.x, self.y + o.y, self.z + o.z)
    }
}
impl Sub for V3 {
    type Output = V3;
    fn sub(self, o: V3) -> V3 {
        V3::new(self.x - o.x, self.y - o.y, self.z - o.z)
    }
}
impl Mul<f32> for V3 {
    type Output = V3;
    fn mul(self, k: f32) -> V3 {
        V3::new(self.x * k, self.y * k, self.z * k)
    }
}
impl Neg for V3 {
    type Output = V3;
    fn neg(self) -> V3 {
        self * -1.0
    }
}

// ---------------------------------------------------------------------------
// Numbers.

/// The ship's thrusters (m/s²) and its small-thruster charge.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Engine {
    /// Main engine, along the nose.
    pub main: f32,
    /// Retro thruster, against the nose (FA OFF; FA ON uses it as the brake).
    pub retro: f32,
    /// Lift (up the deck) and drop (down the deck).
    pub lift: f32,
    pub drop: f32,
    /// Lateral thrusters, each way.
    pub lateral: f32,
    /// The boost along the nose.
    pub boost: f32,
    /// The small-thruster charge, in m/s of speed change: its size and its
    /// refill per second (only while there is fuel).
    pub charge_cap: f32,
    pub charge_refill: f32,
}

impl Default for Engine {
    fn default() -> Self {
        Engine {
            main: 60.0,
            retro: 30.0,
            lift: 30.0,
            drop: 12.0,
            lateral: 15.0,
            boost: 70.0,
            charge_cap: 40.0,
            charge_refill: 12.0,
        }
    }
}

/// Every feel number, in one place. `Tuning::default()` is the prototype's.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tuning {
    /// Thrust (other than the boost and a jink) fades along each axis as the
    /// speed that way nears this, and never raises the speed past it.
    pub arm_speed: f32,
    /// Top speed; only the boost and jinks go past `arm_speed` toward it.
    pub speed_cap: f32,
    /// Fuel per second just for going `speed_cap`; none at `arm_speed`, in a
    /// straight line between.
    pub over_burn: f32,
    /// Fuel per m/s of speed change the engine makes.
    pub fuel_per_dv: f32,
    /// Spool time constants (s): main and lift spool slowly, retro, drop and
    /// lateral quickly.
    pub tau_slow: f32,
    pub tau_fast: f32,
    /// Top body rates (rad/s) and angular accelerations (rad/s²), as `w`
    /// (x roll, y yaw, z pitch).
    pub rate_max: V3,
    pub rate_accel: V3,
    /// FA OFF, stick centred: the rate decays at this share of the axis's α.
    pub hands_off_decay: f32,
    /// Dead zone on every analog input.
    pub dead: f32,
    /// FA ON open space: how fast the velocity swings onto the nose (m/s²),
    /// the brake (m/s²), and drag: `glide_decel + glide_drag·v²`.
    pub grip: f32,
    pub brake: f32,
    pub glide_decel: f32,
    pub glide_drag: f32,
    /// Translation input suspends the grip for this long after it ends (s).
    pub grip_hold_s: f32,
    /// FA ON open space: yaw added per sin(bank) (rad/s), faded out as the
    /// roll stick nears full, so a held bank turns and a full-stick roll stays
    /// a roll.
    pub coord: f32,
    /// FA ON: auto-level after the roll stick has been centred this long (s),
    /// at `−gain·φ`, capped.
    pub auto_level_after: f32,
    pub auto_level_gain: f32,
    pub auto_level_max: f32,
    /// Hover assist: on above this field fade, at or under `hover_speed` (or
    /// with the gear down, which also caps the speed there).
    pub hover_fade: f32,
    pub hover_speed: f32,
    /// Height above the pad from which main thrust, gear up, leaves the hover.
    pub punch_min_h: f32,
    /// Hover assist: top speed over the ground, its gain and top acceleration;
    /// climb and sink speeds and the vertical gain; the most lean and the
    /// attitude gain.
    pub hover_vh: f32,
    pub hover_kv: f32,
    pub hover_ah: f32,
    pub hover_climb: f32,
    pub hover_sink: f32,
    pub hover_kz: f32,
    pub hover_tilt: f32,
    pub att_gain: f32,
    /// Soft touch (gear down, under `soft_h`): descent at most
    /// max(`soft_min`, h/5) m/s.
    pub soft_h: f32,
    pub soft_min: f32,
    /// The collective lever (FA OFF in a field): 0..`lever_max`; `hover_mark`
    /// holds the ship against the field.
    pub lever_max: f32,
    pub hover_mark: f32,
    /// Boost and jink: fuel a second on top of the speed gained.
    pub boost_burn: f32,
    /// Above this speed, the small thrusters' push along the velocity is paid
    /// from the tank, not the charge.
    pub small_tank_speed: f32,
    /// Level-out: roll to 0 and |pitch| to at most `level_max_pitch`, gain
    /// `level_gain`; done under `level_done` rad, snapped after `level_snap_s`.
    pub level_max_pitch: f32,
    pub level_gain: f32,
    pub level_done: f32,
    pub level_snap_s: f32,
}

impl Default for Tuning {
    fn default() -> Self {
        Tuning {
            arm_speed: 224.0,
            speed_cap: 448.0,
            over_burn: 0.25,
            fuel_per_dv: 0.1,
            tau_slow: 0.15,
            tau_fast: 0.05,
            rate_max: V3::new(3.5, 2.0, 2.5),
            rate_accel: V3::new(18.0, 10.0, 14.0),
            hands_off_decay: 0.15,
            dead: 0.05,
            grip: 64.0,
            brake: 56.0,
            glide_decel: 0.02,
            glide_drag: 0.0,
            grip_hold_s: 0.6,
            coord: 0.8,
            auto_level_after: 0.3,
            auto_level_gain: 1.5,
            auto_level_max: 1.0,
            hover_fade: 0.5,
            hover_speed: 40.0,
            punch_min_h: 15.0,
            hover_vh: 30.0,
            hover_kv: 2.0,
            hover_ah: 8.0,
            hover_climb: 10.0,
            hover_sink: 16.0,
            hover_kz: 1.5,
            hover_tilt: 40.0 * PI / 180.0,
            att_gain: 6.0,
            soft_h: 20.0,
            soft_min: 1.5,
            lever_max: 2.0,
            hover_mark: 1.0,
            boost_burn: 0.6,
            small_tank_speed: 20.0,
            level_max_pitch: 1.15,
            level_gain: 6.0,
            level_done: 0.01,
            level_snap_s: 1.5,
        }
    }
}

// ---------------------------------------------------------------------------
// Attitude.

/// The ship's attitude as two unit vectors: the nose `f` and the deck's up `u`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Attitude {
    pub f: V3,
    pub u: V3,
}

impl Attitude {
    /// Level (no pitch, no roll) on `heading`.
    pub fn level(heading: f32) -> Self {
        Attitude::from_heading_pitch(heading, 0.0)
    }
    /// Roll 0: f = from_angles(h, p), u = from_angles(h, p + π/2).
    pub fn from_heading_pitch(h: f32, p: f32) -> Self {
        Attitude {
            f: V3::from_angles(h, p),
            u: V3::from_angles(h, p + FRAC_PI_2),
        }
    }
    /// The right wing: f × u.
    pub fn r(&self) -> V3 {
        self.f.cross(self.u)
    }
    /// Bank angle φ: positive is right wing down, ±π upside down.
    pub fn bank(&self) -> f32 {
        (-self.r().z).atan2(self.u.z)
    }
    /// Tilt of the deck from a surface normal (rad), exact near 0.
    pub fn tilt_from(&self, n: V3) -> f32 {
        let n = n.norm();
        self.u.cross(n).len().atan2(self.u.dot(n))
    }
    /// (heading, pitch) of the nose. Keeps `last_heading` while the nose is
    /// within about 1.8° of vertical, so it never jitters or turns NaN there.
    pub fn heading_pitch(&self, last_heading: f32) -> (f32, f32) {
        let f = self.f;
        let p = f.z.atan2(f.x.hypot(f.y));
        let h = if f.z.abs() > 0.9995 {
            last_heading
        } else {
            f.y.atan2(f.x)
        };
        (h, p)
    }
    /// Rotate by body rates `w` for `dt` (Rodrigues), then re-orthonormalise.
    pub fn rotate(&mut self, w: V3, dt: f32) {
        let r = self.r();
        let om = self.f * w.x - self.u * w.y + r * w.z;
        let rate = om.len();
        if rate > 1e-9 && dt > 0.0 {
            let k = om * (1.0 / rate);
            let (s, c) = (rate * dt).sin_cos();
            self.f = rodrigues(self.f, k, c, s);
            self.u = rodrigues(self.u, k, c, s);
        }
        self.ortho();
    }
    /// Roll under 0.01 rad and |pitch| within `max_pitch` (+0.01).
    pub fn is_level(&self, max_pitch: f32) -> bool {
        let (_, p) = self.heading_pitch(0.0);
        self.bank().abs() < 0.01 && p.abs() <= max_pitch + 0.01
    }
    /// The attitude as a rotation from the world axes, as a unit quaternion
    /// `[x, y, z, w]` that takes the ship's local axes (+x nose, +z up,
    /// −y right, i.e. the same handedness as the world) onto `f`, `u`, `r`.
    /// Feed it to your engine's quaternion type.
    pub fn quat(&self) -> [f32; 4] {
        // Columns: local x -> f, local y -> −r (left), local z -> u.
        let (x, y, z) = (self.f, -self.r(), self.u);
        let (m00, m11, m22) = (x.x, y.y, z.z);
        let tr = m00 + m11 + m22;
        if tr > 0.0 {
            let s = (tr + 1.0).sqrt() * 2.0;
            [(y.z - z.y) / s, (z.x - x.z) / s, (x.y - y.x) / s, 0.25 * s]
        } else if m00 > m11 && m00 > m22 {
            let s = (1.0 + m00 - m11 - m22).sqrt() * 2.0;
            [0.25 * s, (y.x + x.y) / s, (z.x + x.z) / s, (y.z - z.y) / s]
        } else if m11 > m22 {
            let s = (1.0 + m11 - m00 - m22).sqrt() * 2.0;
            [(y.x + x.y) / s, 0.25 * s, (z.y + y.z) / s, (z.x - x.z) / s]
        } else {
            let s = (1.0 + m22 - m00 - m11).sqrt() * 2.0;
            [(z.x + x.z) / s, (z.y + y.z) / s, 0.25 * s, (x.y - y.x) / s]
        }
    }
    /// Unit `f`, and `u` made square to it (Gram–Schmidt).
    fn ortho(&mut self) {
        let f = self.f.norm();
        let f = if f.len() > 0.5 { f } else { V3::new(1.0, 0.0, 0.0) };
        let u = (self.u - f * f.dot(self.u)).norm();
        self.u = if u.len() > 0.5 { u } else { any_square(f) };
        self.f = f;
    }
}

/// `v` turned about the unit axis `k` by the angle with cosine `c`, sine `s`.
fn rodrigues(v: V3, k: V3, c: f32, s: f32) -> V3 {
    v * c + k.cross(v) * s + k * (k.dot(v) * (1.0 - c))
}

/// Some unit vector square to the unit vector `f`.
fn any_square(f: V3) -> V3 {
    let a = if f.z.abs() < 0.9 { V3::UP } else { V3::new(1.0, 0.0, 0.0) };
    (a - f * f.dot(a)).norm()
}

/// Body errors (roll, pitch) that tilt the deck's up `u` onto `target`, with
/// the full error even upside down.
fn steer_up(a: &Attitude, target: V3) -> (f32, f32) {
    let c = a.u.cross(target);
    let sn = c.len();
    let cs = a.u.dot(target);
    let axis = if sn > 1e-6 {
        c * (1.0 / sn)
    } else if cs < 0.0 {
        a.f
    } else {
        V3::ZERO
    };
    let e = axis * sn.atan2(cs);
    (e.dot(a.f), e.dot(a.r()))
}

/// Errors from `a` to the target `t`, split as a ship flies it: the nose's
/// swing onto `t.f`, then the roll about the nose that puts the deck on `t.u`.
/// Returns the body errors as `w`, the swing angle and the roll angle.
fn attitude_error(a: &Attitude, t: &Attitude) -> (V3, f32, f32) {
    let c = a.f.cross(t.f);
    let sn = c.len();
    let cs = a.f.dot(t.f);
    let swing = sn.atan2(cs);
    let axis = if sn > 1e-6 {
        c * (1.0 / sn)
    } else if cs < 0.0 {
        a.r()
    } else {
        V3::ZERO
    };
    let (s, co) = swing.sin_cos();
    let u1 = rodrigues(a.u, axis, co, s);
    let roll = u1.cross(t.u).dot(t.f).atan2(u1.dot(t.u));
    let e = axis * swing;
    (V3::new(roll, -e.dot(a.u), e.dot(a.r())), swing, roll)
}

// ---------------------------------------------------------------------------
// Inputs, state and outputs.

/// The pilot's inputs this tick. Sticks are -1..1, triggers 0..1.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Controls {
    /// Cyclic: + = nose up, + = right wing down. Rudder: + = nose right.
    pub pitch: f32,
    pub roll: f32,
    pub yaw: f32,
    /// Main engine, 0..1.
    pub main: f32,
    /// Retro, 0..1 (FA ON open space: the brake).
    pub retro: f32,
    /// Lift, -1..1: + up the deck, − down.
    pub lift: f32,
    /// Strafe, -1..1: + right.
    pub strafe: f32,
    /// Boost held: along the nose, or a jink with a translation held.
    pub boost: bool,
    /// FA OFF: stop rotating, at full authority.
    pub kill_rot: bool,
    /// Grip off (for example while aiming), so the ship can slide.
    pub no_grip: bool,
}

/// The ship's body as the flight model sees it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Body {
    pub pos: V3,
    /// Velocity, relative to whatever frame the game flies in (a moving
    /// pad's body, for example).
    pub vel: V3,
    /// Fuel in the tank. At 0 the engine can't thrust; the ship still turns.
    pub fuel: f32,
}

/// A hover field at the ship, for example near a landing pad: a pull along
/// `accel` that the hover assist and the lever fly against. Pass
/// `HoverField::default()` (none) in open space. The hover assist assumes the
/// pad is level, with world +z up from it. The prototype used a 5 m/s² pull,
/// full strength up to 450 m above the pad and none from 600 m.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HoverField {
    /// The field's full-strength acceleration (m/s², straight down for a level pad).
    pub accel: V3,
    /// 0..1: how strong it is here (use [`field_fade`]).
    pub fade: f32,
    /// Height above the pad top (m).
    pub h: f32,
}

/// What a tick needs besides the state, the body and the controls.
#[derive(Clone, Copy, Debug)]
pub struct Ctx {
    pub dt: f32,
    /// Gravity at the ship (m/s²).
    pub gravity: V3,
    pub engine: Engine,
    pub tuning: Tuning,
    pub field: HoverField,
    /// Landing gear down. With Flight Assist on it caps the speed at
    /// `hover_speed` (in open space too); in a field it keeps the hover assist
    /// on at any speed, main thrust can't punch out, and soft touch holds the
    /// descent near the pad.
    pub gear: bool,
}

impl Ctx {
    /// Open space, no gravity, default engine and tuning.
    pub fn open(dt: f32) -> Self {
        Ctx {
            dt,
            gravity: V3::ZERO,
            engine: Engine::default(),
            tuning: Tuning::default(),
            field: HoverField::default(),
            gear: false,
        }
    }
}

/// Thrust actually delivered after spool-up, body axes (m/s²).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Spool {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// What one tick did, for the HUD and the sound.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FlyOut {
    pub boosting: bool,
    pub jinking: bool,
    pub hover_assist: bool,
    pub grip_on: bool,
    /// 0..1, for sound and engine glow.
    pub main_frac: f32,
    pub lift_frac: f32,
    pub small_frac: f32,
    /// Felt acceleration (thrust, boost and assists; not gravity or the field).
    pub accel: V3,
    pub charge_used: f32,
    pub fuel_used: f32,
}

/// Flight state kept between ticks.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlyState {
    pub att: Attitude,
    /// Body rates (x roll, y yaw, z pitch).
    pub w: V3,
    pub spool: Spool,
    pub charge: f32,
    /// Flight Assist on.
    pub fa: bool,
    /// The collective lever, 0..`lever_max`.
    pub lever: f32,
    /// True while [`level_out_step`] is turning the ship level, and its time.
    pub levelling: bool,
    pub level_t: f32,
    grip_hold: f32,
    roll_idle: f32,
    last_heading: f32,
    pub last: FlyOut,
}

impl FlyState {
    /// Level on `heading`, at rest, full charge, FA on, lever at the hover mark.
    pub fn new(heading: f32, engine: &Engine, tuning: &Tuning) -> Self {
        FlyState {
            att: Attitude::level(heading),
            w: V3::ZERO,
            spool: Spool::default(),
            charge: engine.charge_cap,
            fa: true,
            lever: tuning.hover_mark,
            levelling: false,
            level_t: 0.0,
            grip_hold: 0.0,
            roll_idle: 0.0,
            last_heading: heading,
            last: FlyOut::default(),
        }
    }
    /// (heading, pitch) of the nose, holding the last heading near vertical.
    pub fn heading_pitch(&mut self) -> (f32, f32) {
        let (h, p) = self.att.heading_pitch(self.last_heading);
        self.last_heading = h;
        (h, p)
    }
}

// ---------------------------------------------------------------------------
// Flying.

fn shape(x: f32, lo: f32, dead: f32) -> f32 {
    let x = x.clamp(lo, 1.0);
    if x.abs() < dead { 0.0 } else { x }
}

fn clamp_len(v: V3, max: f32) -> V3 {
    let l = v.len();
    if l > max { v * (max / l) } else { v }
}

/// One spool step toward `cmd`, with the time constant of the thruster now
/// firing, so a thruster spools down as slowly as it spooled up.
fn spool_to(s: f32, cmd: f32, tau_pos: f32, tau_neg: f32, dt: f32) -> f32 {
    let side = if s.abs() > 1e-3 { s } else { cmd };
    let tau = if side > 0.0 { tau_pos } else { tau_neg };
    s + (cmd - s) * (1.0 - (-dt / tau).exp())
}

/// One axis's rate toward `cmd` (clamped to ±`max`), at most `accel·dt` a tick.
fn drive_rate(w: f32, cmd: f32, max: f32, accel: f32, dt: f32) -> f32 {
    let cmd = cmd.clamp(-max, max);
    w + (cmd - w).clamp(-accel * dt, accel * dt)
}

/// `t` as a deck-up target, leaning at most `tilt` from +z.
fn tilt_target(t: V3, tilt: f32) -> V3 {
    let flat = V3::new(t.x, t.y, 0.0);
    if t.len() < 1e-4 {
        return V3::UP;
    }
    let lean = flat.len().atan2(t.z);
    if lean <= tilt {
        t.norm()
    } else if flat.len() > 1e-6 {
        V3::UP * tilt.cos() + flat.norm() * tilt.sin()
    } else {
        V3::UP
    }
}

/// Thrust fade along world direction `d`: 1 at rest, 0 at `arm_speed` that way.
pub fn arm(v: V3, d: V3, arm_speed: f32) -> f32 {
    (1.0 - v.dot(d).max(0.0) / arm_speed).clamp(0.0, 1.0)
}

/// Fuel per second just for going `speed` (none at or under `arm_speed`).
pub fn over_burn(speed: f32, t: &Tuning) -> f32 {
    ((speed - t.arm_speed) / (t.speed_cap - t.arm_speed)).max(0.0) * t.over_burn
}

/// Smoothstep fade of a hover field with distance: 1 at or under `full`,
/// 0 at or over `zero` (`zero` should be past `full`; if not, it is a hard
/// edge at `full`). Multiply the fades for height and for distance from the
/// pad's axis.
pub fn field_fade(d: f32, full: f32, zero: f32) -> f32 {
    if zero <= full {
        return if d <= full { 1.0 } else { 0.0 };
    }
    let s = ((zero - d) / (zero - full)).clamp(0.0, 1.0);
    s * s * (3.0 - 2.0 * s)
}

/// One tick of hand flying. Updates `body.vel`, `body.pos` (by vel·dt) and
/// `body.fuel`, and the state.
pub fn step6(fly: &mut FlyState, body: &mut Body, c: &Controls, cx: &Ctx) -> FlyOut {
    let dt = cx.dt;
    if dt <= 0.0 {
        return fly.last;
    }
    let (e, k) = (cx.engine, cx.tuning);
    fly.levelling = false;
    fly.level_t = 0.0;

    // 1. Setup.
    let ok = body.fuel > 0.0;
    let v0 = body.vel;
    let speed0 = v0.len();
    let att = fly.att;
    let (f, u, r) = (att.f, att.u, att.r());
    let g_f = if ok { cx.field.accel * cx.field.fade } else { V3::ZERO };
    let punch = c.main >= k.dead && cx.field.h >= k.punch_min_h;
    let hover = fly.fa
        && ok
        && cx.field.fade > k.hover_fade
        && (cx.gear || (speed0 <= k.hover_speed && !punch));
    let pitch = shape(c.pitch, -1.0, k.dead);
    let roll = shape(c.roll, -1.0, k.dead);
    let yaw = shape(c.yaw, -1.0, k.dead);
    let main = shape(c.main, 0.0, k.dead);
    let retro = shape(c.retro, 0.0, k.dead);
    let lift = shape(c.lift, -1.0, k.dead);
    let strafe = shape(c.strafe, -1.0, k.dead);
    let lift_held = lift != 0.0;
    let strafe_held = strafe != 0.0;
    let retro_held = !fly.fa && retro != 0.0;
    let held = lift_held || strafe_held || retro_held;
    fly.grip_hold = if held { k.grip_hold_s } else { (fly.grip_hold - dt).max(0.0) };
    let grip_suspended = c.no_grip || fly.grip_hold > 0.0;
    if roll == 0.0 {
        fly.roll_idle += dt;
    } else {
        fly.roll_idle = 0.0;
    }
    // Boost with a translation held jinks (that thruster doubled, no fade);
    // without one it boosts along the nose.
    let jink = c.boost && ok && held;
    let boosting = c.boost && ok && !held;
    let kk = |h: bool| if jink && h { 2.0 } else { 1.0 };
    let fade = |h: bool, d: V3| if jink && h { 1.0 } else { arm(v0, d, k.arm_speed) };
    let (kx, ky, kz) = (kk(retro_held), kk(lift_held), kk(strafe_held));

    // 2. Translation demand, body axes x = f, y = u, z = r.
    let lift_part = if lift >= 0.0 {
        lift * e.lift * ky * fade(lift_held, u)
    } else {
        lift * e.drop * ky * fade(lift_held, -u)
    };
    let side = if strafe >= 0.0 { r } else { -r };
    let side_part = strafe * e.lateral * kz * fade(strafe_held, side);
    let mut field_cancel = V3::ZERO;
    let mut up_target = V3::UP;
    let (cmd_x, cmd_y, cmd_z) = if hover {
        // Drone-style: the stick sets a velocity over the ground, lift a climb
        // or sink, and the ship leans into the push.
        let fh = V3::new(f.x, f.y, 0.0);
        let uh = V3::new(u.x, u.y, 0.0) * -f.z.signum();
        let hf = if fh.len() > 1e-3 {
            fh.norm()
        } else if uh.len() > 1e-3 {
            uh.norm()
        } else {
            V3::new(1.0, 0.0, 0.0)
        };
        let hr = hf.cross(V3::UP);
        let v_tgt = clamp_len(
            (hf * (-pitch + main - retro) + hr * (roll + strafe)) * k.hover_vh,
            k.hover_vh,
        );
        let mut vz_tgt = if lift > 0.0 { k.hover_climb * lift } else { k.hover_sink * lift };
        if cx.gear && cx.field.h < k.soft_h {
            vz_tgt = vz_tgt.max(-k.soft_min.max(cx.field.h / 5.0));
        }
        let a_h = clamp_len((v_tgt - V3::new(v0.x, v0.y, 0.0)) * k.hover_kv, k.hover_ah);
        let a_z = k.hover_kz * (vz_tgt - v0.z);
        // Held against the field and gravity, so it holds still hands off.
        let t = V3::new(a_h.x, a_h.y, a_z) - g_f - cx.gravity;
        up_target = tilt_target(t, k.hover_tilt);
        let mut y = t.dot(u);
        let mut z = t.dot(r);
        if jink {
            if lift_held {
                y += lift_part;
            }
            if strafe_held {
                z += side_part;
            }
        }
        (t.dot(f), y, z)
    } else if fly.fa {
        // Open space, FA ON: retro brakes (step 5); the field is cancelled.
        field_cancel = -g_f;
        (main * e.main * arm(v0, f, k.arm_speed), lift_part, side_part)
    } else {
        // FA OFF: every thruster as it is; the lever holds against the field.
        let x = main * e.main * arm(v0, f, k.arm_speed)
            - retro * e.retro * kx * fade(retro_held, -f);
        let lever = if cx.field.fade > 0.0 {
            fly.lever.clamp(0.0, k.lever_max) * g_f.len()
        } else {
            0.0
        };
        (x, lever + lift_part, side_part)
    };
    let cmd_x = cmd_x.clamp(-e.retro * kx, e.main);
    let cmd_y = cmd_y.clamp(-e.drop * ky, e.lift * ky);
    let cmd_z = cmd_z.clamp(-e.lateral * kz, e.lateral * kz);

    // 3. Spool, then thrust in the world frame. Taking over, the hover assist
    // starts the lift at what holds the ship up, so it doesn't sag.
    if hover && !fly.last.hover_assist {
        let hold = (-(g_f + cx.gravity)).dot(u);
        fly.spool.y = fly.spool.y.max(hold.clamp(0.0, e.lift));
    }
    if ok {
        let s = &mut fly.spool;
        s.x = spool_to(s.x, cmd_x, k.tau_slow, k.tau_fast, dt);
        s.y = spool_to(s.y, cmd_y, k.tau_slow, k.tau_fast, dt);
        s.z = spool_to(s.z, cmd_z, k.tau_fast, k.tau_fast, dt);
    } else {
        fly.spool = Spool::default();
    }
    let s = fly.spool;
    let mut a_t = f * s.x + u * s.y + r * s.z;
    if !jink {
        // Thrust turns the course freely but can't add speed past the arm
        // speed (or push a faster ship faster).
        let lim = speed0.max(k.arm_speed);
        let v1 = v0 + a_t * dt;
        let s1 = v1.len();
        if s1 > lim {
            a_t = (v1 * (lim / s1) - v0) * (1.0 / dt);
        }
    }

    // 4. Thrust, gravity, the field (and its free cancel).
    let mut v = v0 + (a_t + cx.gravity + g_f + field_cancel) * dt;
    let mut felt = a_t;
    // The hover assist's main thrust is paid as the small thrusters (step 8).
    let mut tank_dv = if hover { 0.0 } else { s.x.max(0.0) * dt };

    // 5. Boost: along the nose, not spooled, up to the top speed.
    if boosting {
        let along = v.dot(f).max(0.0);
        let gain = (e.boost * dt).min((k.speed_cap - along).max(0.0));
        v = v + f * gain;
        felt = felt + f * (gain / dt);
        tank_dv += gain;
    }

    // 6. Assists (FA ON, open space): drag, the brake and the grip.
    let mut grip_on = false;
    if fly.fa && !hover && ok {
        let before = v;
        let sp = v.len();
        if sp > 1e-4 {
            let drag = (k.glide_decel + k.glide_drag * sp * sp) * dt;
            let brake = (k.brake * retro * dt).min((sp - drag).max(0.0));
            tank_dv += brake;
            v = v * (1.0 - (drag + brake).min(sp) / sp);
        }
        let sp = v.len();
        if !grip_suspended {
            grip_on = true;
            if sp > 1e-4 {
                let start = v;
                let along = v.dot(f);
                let lat = v - f * along;
                if along > 0.0 {
                    let off = v.angle_to(f);
                    let turn = off.min(k.grip * dt / sp);
                    let vh = v * (1.0 / sp);
                    let toward = (f - vh * vh.dot(f)).norm();
                    v = (vh * turn.cos() + toward * turn.sin()) * sp;
                } else {
                    let l = lat.len();
                    if l > 1e-6 {
                        v = v - lat * ((k.grip * dt).min(l) / l);
                    }
                }
                tank_dv += v.dist(start);
            }
        }
        felt = felt + (v - before) * (1.0 / dt);
    }

    // 7. Rotation.
    let phi = att.bank();
    let (rm, ra) = (k.rate_max, k.rate_accel);
    let mut w = fly.w;
    if hover {
        let (roll_err, pitch_err) = steer_up(&att, up_target);
        w.x = drive_rate(w.x, k.att_gain * roll_err, rm.x, ra.x, dt);
        w.y = drive_rate(w.y, yaw * rm.y, rm.y, ra.y, dt);
        w.z = drive_rate(w.z, k.att_gain * pitch_err, rm.z, ra.z, dt);
    } else if fly.fa {
        let cmd_roll = if roll != 0.0 {
            roll * rm.x
        } else if fly.roll_idle >= k.auto_level_after {
            (-k.auto_level_gain * phi).clamp(-k.auto_level_max, k.auto_level_max)
        } else {
            0.0
        };
        w.x = drive_rate(w.x, cmd_roll, rm.x, ra.x, dt);
        let cmd_yaw = yaw * rm.y + k.coord * phi.sin() * (1.0 - roll.abs());
        w.y = drive_rate(w.y, cmd_yaw, rm.y, ra.y, dt);
        w.z = drive_rate(w.z, pitch * rm.z, rm.z, ra.z, dt);
    } else {
        // FA OFF: the stick sets a rate; let go and it keeps spinning,
        // decaying slowly (kill_rot stops it at full authority).
        let axis = |w: f32, stick: f32, max: f32, accel: f32| {
            if stick != 0.0 {
                drive_rate(w, stick * max, max, accel, dt)
            } else if c.kill_rot {
                drive_rate(w, 0.0, max, accel, dt)
            } else {
                w - w.signum() * w.abs().min(k.hands_off_decay * accel * dt)
            }
        };
        w.x = axis(w.x, roll, rm.x, ra.x);
        w.y = axis(w.y, yaw, rm.y, ra.y);
        w.z = axis(w.z, pitch, rm.z, ra.z);
    }
    fly.w = w;
    fly.att.rotate(w, dt);

    // 8. Caps, then move.
    if cx.gear && fly.fa {
        v = clamp_len(v, k.hover_speed);
    }
    v = clamp_len(v, k.speed_cap);
    body.vel = v;
    body.pos = body.pos + v * dt;

    // 9. Fuel. The small thrusters pay what they push beyond holding against
    // the field: from the charge, except (above `small_tank_speed`) the share
    // along the velocity, which pays the tank. In the hover all its thrust is
    // paid this way, so a hover move costs the same any way.
    let a_s = if hover { f * s.x + u * s.y + r * s.z } else { f * s.x.min(0.0) + u * s.y + r * s.z };
    let m = a_s.len().min((a_s + g_f).len()) * dt;
    let from_tank = if !hover && speed0 > k.small_tank_speed && a_s.len() > 1e-6 {
        m * a_s.norm().dot(v0 * (1.0 / speed0)).abs()
    } else {
        0.0
    };
    let want = m - from_tank;
    let paid = want.min(fly.charge).max(0.0);
    fly.charge -= paid;
    tank_dv += from_tank + (want - paid);
    let fuel_before = body.fuel;
    let mut burn = tank_dv * k.fuel_per_dv;
    if jink || boosting {
        burn += k.boost_burn * dt;
    }
    if ok {
        burn += over_burn(v.len(), &k) * dt;
    }
    body.fuel = (body.fuel - burn).max(0.0);
    if body.fuel > 0.0 {
        fly.charge = (fly.charge + e.charge_refill * dt).min(e.charge_cap);
    }

    let out = FlyOut {
        boosting,
        jinking: jink,
        hover_assist: hover,
        grip_on,
        main_frac: (s.x.max(0.0) / e.main).clamp(0.0, 1.0),
        lift_frac: (s.y.abs() / e.lift).clamp(0.0, 1.0),
        small_frac: ((f * s.x.min(0.0) + r * s.z).len() / e.retro).clamp(0.0, 1.0),
        accel: felt,
        charge_used: paid,
        fuel_used: fuel_before - body.fuel,
    };
    // Keep the heading current, so level-out near the vertical keeps it.
    fly.heading_pitch();
    fly.last = out;
    out
}

/// One tick of levelling out (for example before an autopilot takes over):
/// roll to 0, |pitch| to at most `level_max_pitch`, no thrust; the ship coasts
/// under gravity. Returns true once level, or after `level_snap_s`, when it
/// snaps level.
pub fn level_out_step(fly: &mut FlyState, body: &mut Body, gravity: V3, k: &Tuning, dt: f32) -> bool {
    fly.levelling = true;
    fly.level_t += dt.max(0.0);
    fly.spool = Spool::default();
    fly.last = FlyOut::default();
    body.vel = body.vel + gravity * dt;
    body.pos = body.pos + body.vel * dt;

    let target = |a: &Attitude, last: f32| {
        let (h, p) = a.heading_pitch(last);
        Attitude::from_heading_pitch(h, p.clamp(-k.level_max_pitch, k.level_max_pitch))
    };
    let tgt = target(&fly.att, fly.last_heading);
    let (err, _, _) = attitude_error(&fly.att, &tgt);
    let (rm, ra) = (k.rate_max, k.rate_accel);
    let mut w = fly.w;
    w.x = drive_rate(w.x, k.level_gain * err.x, rm.x, ra.x, dt);
    w.y = drive_rate(w.y, k.level_gain * err.y, rm.y, ra.y, dt);
    w.z = drive_rate(w.z, k.level_gain * err.z, rm.z, ra.z, dt);
    fly.w = w;
    fly.att.rotate(w, dt);

    let tgt = target(&fly.att, fly.last_heading);
    let (_, swing, roll) = attitude_error(&fly.att, &tgt);
    let done = swing.abs() < k.level_done && roll.abs() < k.level_done;
    if done || fly.level_t >= k.level_snap_s {
        fly.att = tgt;
        fly.w = V3::ZERO;
        fly.levelling = false;
    }
    fly.heading_pitch();
    !fly.levelling
}

/// Where the ship first meets a pad's plane (z = 0) under a constant
/// acceleration, for a landing HUD ring. `rel_pad` is its offset from the pad
/// centre, `v` and `a` relative to the pad. None if below the plane or never
/// coming down to it.
pub fn impact_point(rel_pad: V3, v: V3, a: V3) -> Option<V3> {
    if rel_pad.z < 0.0 {
        return None;
    }
    let (qa, qb, qc) = (0.5 * a.z, v.z, rel_pad.z);
    let t = if qa.abs() < 1e-6 {
        if qb < -1e-6 { -qc / qb } else { return None }
    } else {
        let disc = qb * qb - 4.0 * qa * qc;
        if disc < 0.0 {
            return None;
        }
        let q = -0.5 * (qb + qb.signum() * disc.sqrt());
        let roots = [q / qa, if q.abs() > 1e-12 { qc / q } else { f32::INFINITY }];
        let t = roots.into_iter().filter(|t| *t >= 0.0).fold(f32::INFINITY, f32::min);
        if !t.is_finite() {
            return None;
        }
        t
    };
    let p = rel_pad + v * t + a * (0.5 * t * t);
    Some(V3::new(p.x, p.y, 0.0))
}
