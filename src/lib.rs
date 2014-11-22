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
    armor_class: int,
    hit_points: int
}

impl Default for Character {
    fn default() -> Character {
        Character {
            name: "".to_string(),
            alignment: Evil,
            armor_class: 10,
            hit_points: 5
        }
    }
}

fn attack(roll: int, defender: Character) -> Character {
    Character {
        name: defender.name,
        alignment: defender.alignment,
        armor_class: defender.armor_class,
        hit_points: defender.hit_points - 1
    }
}

#[cfg(test)]
mod tests {
    use std::default::Default;
    use super::{Good, Neutral, Evil, Character, attack};

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
            alignment: Good,
            ..Default::default()
        };

        assert!(Good == gimble.alignment);
    }

    #[test]
    fn test_character_alignment_neutral() {
        let jozan = Character {
            alignment: Neutral,
            ..Default::default()
        };

        assert!(Neutral == jozan.alignment);
    }

    #[test]
    fn test_character_alignment_evil() {
        let vadania = Character {
            alignment: Evil,
            ..Default::default()
        };

        assert!(Evil == vadania.alignment);
    }

    #[test]
    fn test_character_default_armor_class_is_10() {
        let tordek = Character {
            ..Default::default()
        };

        assert_eq!(10, tordek.armor_class);
    }

    #[test]
    fn test_character_default_hit_points_are_5() {
        let regdar = Character {
            ..Default::default()
        };

        assert_eq!(5, regdar.hit_points);
    }

    #[test]
    fn test_character_can_attack() {
        let vadania = Character { ..Default::default() };
        let tordek = Character { ..Default::default() };
        let original_hit_points = tordek.hit_points;

        let attacked_tordek = attack(10, tordek);

        assert!(original_hit_points != attacked_tordek.hit_points);
    }
}
