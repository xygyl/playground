use owo_colors::OwoColorize;
use std::fmt::Display;

pub trait IntToF64 {
    fn to_f64(self) -> f64;
}

macro_rules! impl_int_to_f64 {
    ($($t:ty),* $(,)?) => {
        $(
            impl IntToF64 for $t {
                #[inline]
                fn to_f64(self) -> f64 { self as f64 }
            }
        )*
    };
}

impl_int_to_f64!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

pub fn green_to_red_gradient<T>(val: T, max: T) -> String
where
    T: Copy + IntToF64 + Display,
{
    let max_f = max.to_f64();

    if !(max_f.is_finite() || max_f < 0.0) {
        let s = format!("{:>2}", val);
        return s.truecolor(0, 255, 0).to_string();
    }

    let mut val_f = val.to_f64();

    if val_f < 0.0 {
        val_f = 0.0
    }
    if val_f > max_f {
        val_f = max_f
    }

    let t = (val_f / max_f).clamp(0.0, 1.0);

    // green (0,255,0) → yellow (255,255,0) at t=0.5 → red (255,0,0) at t=1
    let (r, g, b) = if t < 0.5 {
        // green → yellow
        let f = t / 0.5;
        ((255.0 * f) as u8, 255, 0)
    } else {
        // yellow → red
        let f = (t - 0.5) / 0.5;
        (255, (255.0 * (1.0 - f)) as u8, 0)
    };

    let s = format!("{:2}", val);
    s.truecolor(r, g, b).to_string()
}
