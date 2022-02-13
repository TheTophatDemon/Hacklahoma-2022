use crate::personality::Personality;

pub enum Action {
    Party,
    Read,
    Fight,
    Family,
    Clean,
    Nap,
    Worry,
    Flirt,
    Travel,
    Church
}

impl Action {
    pub fn get_probability(self, personality: Personality) -> f32 {
        match self {
            Self::Party => personality.extroversion,
            Self::Read => 1.0 - personality.extroversion,
            Self::Fight => 1.0 - personality.agreeableness,
            Self::Family => personality.agreeableness,
            Self::Clean => personality.conscientiousness,
            Self::Nap => 1.0 - personality.conscientiousness,
            Self::Worry => personality.neuroticism,
            Self::Flirt => 1.0 - personality.neuroticism,
            Self::Travel => personality.openness,
            Self::Church => 1.0 - personality.openness
        }
    }
}