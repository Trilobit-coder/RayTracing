/// A real interval from `min` to `max`.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Interval {
    /// The lower bound.
    pub min: f32,
    /// The upper bound.
    pub max: f32,
}

impl Interval {
    /// Creates an interval from its bounds.
    pub const fn new(min: f32, max: f32) -> Interval {
        Interval { min, max }
    }

    /// The length of the interval.
    pub const fn size(&self) -> f32 {
        self.max - self.min
    }

    /// Whether `x` lies within the interval, boundaries included.
    pub const fn contains(&self, x: f32) -> bool {
        self.min <= x && self.max >= x
    }
    /// Whether `x` lies strictly within the interval, boundaries excluded.
    pub const fn surround(&self, x: f32) -> bool {
        self.min < x && self.max > x
    }

    /// Clamps `x` to the interval.
    pub const fn clamp(&self, x: f32) -> f32 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        x
    }

    /// The empty interval.
    pub const fn empty() -> Interval {
        Interval {
            min: f32::INFINITY,
            max: -f32::INFINITY,
        }
    }

    /// The interval containing every real value.
    pub const fn universe() -> Interval {
        Interval {
            min: -f32::INFINITY,
            max: f32::INFINITY,
        }
    }
}
