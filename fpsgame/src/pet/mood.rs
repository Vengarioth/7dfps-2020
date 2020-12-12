

pub struct PetMood {
    pub mood: Mood,
}

pub enum Mood{
    //positive
    Happy,
    Content,
    Smug,

    //negative
    Sad,
    Hurt,
    Angry,
    Scared,
}