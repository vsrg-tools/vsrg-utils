pub struct RatingProcessor {
    pub difficulty_rating: f32,
}

impl RatingProcessor {
    #[allow(dead_code)]
    const VERSION: &'static str = "0.0.1";

    pub fn new(difficulty_rating: f32) -> Self {
        Self { difficulty_rating }
    }

    pub fn calculate_rating(&self, accuracy: f32, failed: Option<bool>) -> f32 {
        if failed.unwrap_or(false) {
            return 0.;
        }

        self.difficulty_rating * (accuracy / 98.).powf(6.)
    }
}
