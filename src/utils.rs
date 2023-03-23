/**
* Ignore the over engineering
* My main interest was to test the strength of enums in RustLang
*/

#[derive(Debug)]
pub enum GamerLevel {
    Easy(Level),
    Intermediate(Level),
    Difficult(Level),
    Legend(Level)
}

#[derive(Debug)]
pub struct Level {
    pub lower: isize,
    pub higher: isize,
}

impl GamerLevel {

    pub fn new(level: isize) -> Self {
        match level {
            1 => GamerLevel::easy(),
            2 => GamerLevel::intermediate(),
            3 => GamerLevel::difficult(),
            4 => GamerLevel::legend(),
            _ => panic!("Invalid level.")
        }
    }

    fn easy() -> Self {
        Self::Easy(Level {
            lower: 1,
            higher: 5
        })
    }

    fn intermediate() -> Self {
        Self::Intermediate(Level {
            lower: 1,
            higher: 10
        })
    }

    fn difficult() -> Self {
        Self::Difficult(Level {
            lower: 1,
            higher: 100
        })
    }

    fn legend() -> Self {
        Self::Legend(Level {
            lower: 1,
            higher: 1000
        })
    }

}

