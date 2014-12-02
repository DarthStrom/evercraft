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
    vitality: Vitality,
    strength: int,
    dexterity: int,
    constitution: int,
    wisdom: int,
    intelligence: int,
    charisma: int
}

impl Default for Character {
    fn default() -> Character {
        Character {
            name: "".to_string(),
            alignment: Evil,
            armor_class: 10,
            hit_points: 5,
            vitality: Alive,
            strength: 10,
            dexterity: 10,
            constitution: 10,
            wisdom: 10,
            intelligence: 10,
            charisma: 10
        }
    }
}

fn modifier_for(score: int) -> int {
    (score / 2) - 5
}

fn is_crit(roll: int) -> bool {
    roll == 20
}

fn get_vitality(hit_points: int) -> Vitality {
    if hit_points <= 0 {
        Dead
    }
    else {
        Alive
    }
}

fn attack(attacker: Character, roll: int, defender: Character) -> Character {
    let mut new_hit_points = defender.hit_points;
    let mut damage = 1;
    let strength_modifier = modifier_for(attacker.strength);
    let modified_roll = roll + strength_modifier;
    let mut modified_damage = 0;

    if is_crit(roll) {
        damage = damage * 2;
        modified_damage = damage + (2 * strength_modifier);
    }
    else {
        modified_damage = damage + strength_modifier;
    }

    if modified_roll >= defender.armor_class {
        new_hit_points = new_hit_points - modified_damage;
    }

    let new_vitality = get_vitality(new_hit_points);

    Character {
        name: defender.name,
        alignment: defender.alignment,
        armor_class: defender.armor_class,
        hit_points: new_hit_points,
        vitality: new_vitality,
        strength: defender.strength,
        dexterity: defender.dexterity,
        constitution: defender.constitution,
        wisdom: defender.wisdom,
        intelligence: defender.intelligence,
        charisma: defender.charisma
    }
}

#[cfg(test)]
mod tests {
    use std::default::Default;
    use super::{Good, Neutral, Evil, Alive, Dead, Character,
                modifier_for, attack};

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
        let regdar = Character { ..Default::default() };
        let tordek = Character { ..Default::default() };
        let original_hit_points = tordek.hit_points;

        let attacked_tordek = attack(regdar, 10, tordek);

        let normal_hit = 1;
        assert_eq!(original_hit_points - normal_hit,
                   attacked_tordek.hit_points);
    }

    #[test]
    fn test_attack_can_miss() {
        let regdar = Character { ..Default::default() };
        let tordek = Character { ..Default::default() };
        let original_hit_points = tordek.hit_points;

        let attacked_tordek = attack(regdar, 9, tordek);

        assert_eq!(original_hit_points, attacked_tordek.hit_points);
    }

    #[test]
    fn test_attack_can_crit() {
        let regdar = Character { ..Default::default() };
        let tordek = Character { ..Default::default() };
        let original_hit_points = tordek.hit_points;

        let attacked_tordek = attack(regdar, 20, tordek);

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
        let regdar = Character { ..Default::default() };
        let tordek = Character { hit_points: 1, ..Default::default() };

        let attacked_tordek = attack(regdar, 10, tordek);

        assert!(Dead == attacked_tordek.vitality);
    }

    #[test]
    fn test_character_dies_when_reduced_below_zero_hit_points() {
        let regdar = Character { ..Default::default() };
        let tordek = Character { hit_points: 1, ..Default::default() };

        let attacked_tordek = attack(regdar, 20, tordek);

        assert!(Dead == attacked_tordek.vitality);
    }

    #[test]
    fn test_character_abilities_default_to_10() {
        let krusk = Character { ..Default::default() };

        assert_eq!(10, krusk.strength);
        assert_eq!(10, krusk.dexterity);
        assert_eq!(10, krusk.constitution);
        assert_eq!(10, krusk.wisdom);
        assert_eq!(10, krusk.intelligence);
        assert_eq!(10, krusk.charisma);
    }

    #[test]
    fn test_abilities_have_modifiers() {
        assert_eq!(-5, modifier_for(1));
        assert_eq!(-4, modifier_for(2));
        assert_eq!(-4, modifier_for(3));
        assert_eq!(-1, modifier_for(8));
        assert_eq!(-1, modifier_for(9));
        assert_eq!(0, modifier_for(10));
        assert_eq!(0, modifier_for(11));
        assert_eq!(1, modifier_for(12));
        assert_eq!(1, modifier_for(13));
        assert_eq!(4, modifier_for(18));
        assert_eq!(4, modifier_for(19));
        assert_eq!(5, modifier_for(20));
    }

    #[test]
    fn test_strength_modifier_is_added_to_roll_and_damage() {
        let normal_damage = 1;
        let krusk = Character { strength: 12, ..Default::default() };
        let strength_modifier = modifier_for(krusk.strength);
        let tordek = Character { ..Default::default() };
        let expected_hit_points = tordek.hit_points - normal_damage - strength_modifier;
        let attacked_tordek = attack(krusk, 9, tordek);

        assert_eq!(expected_hit_points, attacked_tordek.hit_points);
    }

    #[test]
    fn test_double_strength_modifier_is_added_to_crit_damage() {
        let normal_damage = 1;
        let crit_multiplier = 2;
        let krusk = Character { strength: 12, ..Default::default() };
        let strength_modifier = modifier_for(krusk.strength);
        let tordek = Character { ..Default::default() };
        let expected_hit_points = tordek.hit_points - (normal_damage * crit_multiplier) - (2 *
                                                                                           strength_modifier);

        let attacked_tordek = attack(krusk, 20, tordek);

        assert_eq!(expected_hit_points, attacked_tordek.hit_points);
    }
}
