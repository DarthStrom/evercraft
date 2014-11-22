use std::default::Default;

#[deriving(PartialEq)]
enum Alignment {
    Good,
    Neutral,
    Evil
}

struct Character {
    name: String,
    alignment: Alignment,
    armor_class: int
}

impl Default for Character {
    fn default() -> Character {
        Character {
            name: "".to_string(),
            alignment: Evil,
            armor_class: 10
        }
    }
}

#[cfg(test)]
mod tests {
    use std::default::Default;
    use super::{Good, Neutral, Evil, Character};

    #[test]
    fn test_character_has_a_name() {
        let krusk = Character {
            name: "Krusk".to_string(),
            ..Default::default()
        };

        assert_eq!("Krusk".to_string(), krusk.name);
    }

    #[test]
    fn test_character_alignment_good() {
        let gimble = Character {
            name: "Gimble".to_string(),
            alignment: Good,
            ..Default::default()
        };

        assert!(Good == gimble.alignment);
    }

    #[test]
    fn test_character_alignment_neutral() {
        let jozan = Character {
            name: "Jozan".to_string(),
            alignment: Neutral,
            ..Default::default()
        };

        assert!(Neutral == jozan.alignment);
    }

    #[test]
    fn test_character_alignment_evil() {
        let vadania = Character {
            name: "Vadania".to_string(),
            alignment: Evil,
            ..Default::default()
        };

        assert!(Evil == vadania.alignment);
    }

    #[test]
    fn test_character_default_armor_class_is_10() {
        let tordek = Character {
            name: "Tordek".to_string(),
            alignment: Neutral,
            ..Default::default()
        };

        assert_eq!(10i, tordek.armor_class);
    }
}
