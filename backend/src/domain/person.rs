#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Gender {
    Male,
    Female,
    #[default]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersonStats {
    pub id: String,
    pub given_name: String,
    pub surname: String,
    pub gender: Gender,
}

impl PersonStats {
    pub fn new(id: String, given_name: String, surname: String, gender: Gender) -> Self {
        Self {
            id,
            given_name,
            surname,
            gender,
        }
    }
}
