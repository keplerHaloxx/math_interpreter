pub trait Round {
    fn round_to(self, percision: u32) -> f64;
}

impl Round for f64 {
    fn round_to(self, percision: u32) -> f64 {
        if self == 0. || percision == 0 {
            0.
        } else {
            let shift = percision as i32 - self.abs().log10().ceil() as i32;
            let shift_factor = 10_f64.powi(shift);

            (self * shift_factor).round() / shift_factor
        }
    }
}
