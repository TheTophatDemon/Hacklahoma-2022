use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Personality {
    pub openness: f32,
    pub conscientiousness: f32,
    pub agreeableness: f32,
    pub neuroticism: f32,
    pub extroversion: f32
}

