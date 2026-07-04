#[derive(Debug, Clone, PartialEq)]
pub enum Rotation {
    Clockwise(u32),
    CounterClockwise(u32),
}

impl From<i32> for Rotation {
    fn from(value: i32) -> Self {
        if value > 0 {
            Rotation::Clockwise(value as u32)
        } else {
            Rotation::CounterClockwise(value.abs() as u32)
        }
    }
}
