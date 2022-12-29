use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Eq, PartialEq, PartialOrd, Hash, Copy, Clone, Debug, FromPrimitive, ToPrimitive, Ord)]
pub enum Judgement {
    Marv,
    Perf,
    Great,
    Good,
    Okay,
    Miss,
    Ghost,
}

pub struct JudgementWindows {
    pub id: i32,
    pub name: String,
    pub is_default: bool,
    pub combo_break_judgement: Judgement,
    pub marvelous: f32,
    pub perfect: f32,
    pub great: f32,
    pub good: f32,
    pub okay: f32,
    pub miss: f32,
}

impl Default for JudgementWindows {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::default(),
            marvelous: 18.,
            perfect: 43.,
            great: 76.,
            good: 106.,
            okay: 127.,
            miss: 164.,
            combo_break_judgement: Judgement::Miss,
            is_default: true,
        }
    }
}

impl JudgementWindows {
    pub fn get_value_from_judgement(&mut self, j: Judgement) -> Option<f32> {
        match j {
            Judgement::Marv => Some(self.marvelous),
            Judgement::Perf => Some(self.perfect),
            Judgement::Great => Some(self.great),
            Judgement::Good => Some(self.good),
            Judgement::Okay => Some(self.okay),
            Judgement::Miss => Some(self.miss),
            _ => None,
        }
    }
}
