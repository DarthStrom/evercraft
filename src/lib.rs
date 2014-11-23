use std::default::Default;

#[deriving(PartialEq)]
enum Alignment {
    Good,
    Neutral,
    Evil
}

#[deriving(PartialEq)]
enum Vitality {
    Alive,
    Dead
}

struct Character {
    name: String,
    alignment: Alignment,
    armor_class: int,
    hit_points: int,
    vitality: Vitality
}

impl Default for Character {
    fn default() -> Character {
        Character {
            name: "".to_string(),
            alignment: Evil,
            armor_class: 10,
            hit_points: 5,
            vitality: Alive
        }
    }
}

fn attack(roll: int, defender: Character) -> Character {
    let mut new_hit_points = defender.hit_points;
    let mut damage = 1;
    let mut new_vitality = Alive;
    
    if roll == 20 {
        damage = damage * 2;
    }

    if roll >= defender.armor_class {
        new_hit_points = new_hit_points - damage;
    }

    if new_hit_points == 0 {
        new_vitality = Dead;
    }

    Character {
        name: defender.name,
        alignment: defender.alignment,
        armor_class: defender.armor_class,
        hit_points: new_hit_points,
        vitality: new_vitality
    }
}

#[cfg(test)]
mod tests {
    use std::default::Default;
    use super::{Good, Neutral, Evil, Alive, Dead, Character, attack};

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
        let tordek = Character { ..Default::default() };
        let original_hit_points = tordek.hit_points;

        let attacked_tordek = attack(10, tordek);

        let normal_hit = 1;
        assert_eq!(original_hit_points - normal_hit,
                   attacked_tordek.hit_points);
    }

    #[test]
    fn test_attack_can_miss() {
        let tordek = Character { ..Default::default() };
        let original_hit_points = tordek.hit_points;

        let attacked_tordek = attack(9, tordek);

        assert_eq!(original_hit_points, attacked_tordek.hit_points);
    }

    #[test]
    fn test_attack_can_crit() {
        let tordek = Character { ..Default::default() };
        let original_hit_points = tordek.hit_points;

        let attacked_tordek = attack(20, tordek);

        let normal_hit = 1;
        let crit_multiplier = 2;
        assert_eq!(original_hit_points - (crit_multiplier * normal_hit),
                   attacked_tordek.hit_points);
    }

    #[test]
    fn test_character_is_alive() {
        let tordek = Character { ..Default::default() };

        assert!(Alive == tordek.vitality);
    }

    #[test]
    fn test_character_dies_when_reduced_to_zero_hit_points() {
        let tordek = Character { hit_points: 1, ..Default::default() };

        let attacked_tordek = attack(10, tordek);

        assert!(Dead == attacked_tordek.vitality);
    }
}
