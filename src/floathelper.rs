pub trait FloatHelper {
    fn normalize(&self, min: Self, max: Self) -> Self;
}

impl FloatHelper for f32 {
    // normalise a value between 0 and 1
    fn normalize(&self, min: f32, max: f32) -> f32 {
        (self - min) / (max - min)
    }
}