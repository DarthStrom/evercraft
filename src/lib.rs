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
    charisma: int,
    experience: int
}

struct Combatants {
    attacker: Character,
    defender: Character
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
            charisma: 10,
            experience: 0
        }
    }
}

fn modifier_for(score: int) -> int {
    (score / 2) - 5
}

fn get_crit_multiplier(roll: int) -> int {
    if roll == 20 {
        2
    } else {
        1
    }
}

fn get_vitality(hit_points: int) -> Vitality {
    if hit_points <= 0 {
        Dead
    } else {
        Alive
    }
}

fn get_hit_point_reduction(roll: int, armor_class: int, damage: int) -> int {
    if roll >= armor_class {
        damage
    } else {
        0
    }
}

fn get_modified_damage(roll: int, modifier: int) -> int {
    let normal_damage = 1;
    let crit_multiplier = get_crit_multiplier(roll);
    let mut modified_damage = 0;

    modified_damage = crit_multiplier * (normal_damage + modifier);

    if modified_damage < 1 {
        1
    } else {
        modified_damage
    }
}

fn get_modified_hit_points(hit_points: int, modifier: int) -> int {
    let modified_hit_points = hit_points + modifier;

    if modified_hit_points >= 1 {
        modified_hit_points
    } else {
        1
    }
}

fn attack(combatants: Combatants, roll: int) -> Combatants {
    let attacker = combatants.attacker;
    let defender = combatants.defender;
    let strength_modifier = modifier_for(attacker.strength);
    let dexterity_modifier = modifier_for(defender.dexterity);
    let constitution_modifier = modifier_for(defender.constitution);
    let modified_roll = roll + strength_modifier;
    let modified_damage = get_modified_damage(roll, strength_modifier);
    let modified_armor_class = defender.armor_class + dexterity_modifier;
    let modified_hit_points = get_modified_hit_points(defender.hit_points, constitution_modifier);
    let hit_point_reduction = get_hit_point_reduction(modified_roll, modified_armor_class,
                                                      modified_damage);
    let new_hit_points = defender.hit_points - hit_point_reduction;
    let new_vitality = get_vitality(modified_hit_points - hit_point_reduction);

    Combatants {
        attacker: Character {
            name: attacker.name,
            alignment: attacker.alignment,
            armor_class: attacker.armor_class,
            hit_points: attacker.hit_points,
            vitality: attacker.vitality,
            strength: attacker.strength,
            dexterity: attacker.dexterity,
            constitution: attacker.constitution,
            wisdom: attacker.wisdom,
            intelligence: attacker.intelligence,
            charisma: attacker.charisma,
            experience: attacker.experience + 10
        },

        defender: Character {
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
            charisma: defender.charisma,
            experience: defender.experience
        }
    }
}

#[cfg(test)]
mod tests {
    use std::default::Default;
    use super::{Good, Neutral, Evil, Alive, Dead, Character, Combatants,
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
        let combatants = Combatants { attacker: regdar, defender: tordek };

        let new_combatants = attack(combatants, 10);

        let normal_hit = 1;
        assert_eq!(original_hit_points - normal_hit,
                   new_combatants.defender.hit_points);
    }

    #[test]
    fn test_attack_can_miss() {
        let regdar = Character { ..Default::default() };
        let tordek = Character { ..Default::default() };
        let original_hit_points = tordek.hit_points;
        let combatants = Combatants { attacker: regdar, defender: tordek };

        let new_combatants = attack(combatants, 9);

        assert_eq!(original_hit_points, new_combatants.defender.hit_points);
    }

    #[test]
    fn test_attack_can_crit() {
        let regdar = Character { ..Default::default() };
        let tordek = Character { ..Default::default() };
        let original_hit_points = tordek.hit_points;
        let combatants = Combatants { attacker: regdar, defender: tordek };

        let new_combatants = attack(combatants, 20);

        let normal_hit = 1;
        let crit_multiplier = 2;
        assert_eq!(original_hit_points - (crit_multiplier * normal_hit),
            new_combatants.defender.hit_points);
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
        let combatants = Combatants { attacker: regdar, defender: tordek };

        let new_combatants = attack(combatants, 10);

        assert!(Dead == new_combatants.defender.vitality);
    }

    #[test]
    fn test_character_dies_when_reduced_below_zero_hit_points() {
        let regdar = Character { ..Default::default() };
        let tordek = Character { hit_points: 1, ..Default::default() };
        let combatants = Combatants { attacker: regdar, defender: tordek };

        let new_combatants = attack(combatants, 20);

        assert!(Dead == new_combatants.defender.vitality);
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
        let combatants = Combatants { attacker: krusk, defender: tordek };

        let new_combatants = attack(combatants, 9);

        assert_eq!(expected_hit_points, new_combatants.defender.hit_points);
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
        let combatants = Combatants { attacker: krusk, defender: tordek };

        let new_combatants = attack(combatants, 20);

        assert_eq!(expected_hit_points, new_combatants.defender.hit_points);
    }

    #[test]
    fn test_minimum_damage_is_1() {
        let normal_damage = 1;
        let krusk = Character { strength: 9, ..Default::default() };
        let tordek = Character { ..Default::default() };
        let expected_hit_points = tordek.hit_points - normal_damage;
        let combatants = Combatants { attacker: krusk, defender: tordek };

        let new_combatants = attack(combatants, 11);

        assert_eq!(expected_hit_points, new_combatants.defender.hit_points);
    }

    #[test]
    fn test_dexterity_modifier_is_added_to_armor_class() {
        let krusk = Character { ..Default::default() };
        let tordek = Character { dexterity: 12, ..Default::default() };
        let expected_hit_points = tordek.hit_points;
        let combatants = Combatants { attacker: krusk, defender: tordek };

        let new_combatants = attack(combatants, 10);

        assert_eq!(expected_hit_points, new_combatants.defender.hit_points);
    }

    #[test]
    fn test_constitution_modifier_is_added_to_hit_points() {
        let krusk = Character { ..Default::default() };
        let tordek = Character { hit_points: 1, constitution: 12, ..Default::default() };
        let combatants = Combatants { attacker: krusk, defender: tordek };

        let new_combatants = attack(combatants, 10);

        assert!(Alive == new_combatants.defender.vitality);
    }

    #[test]
    fn test_constitution_modifier_does_not_lower_hit_points_below_1() {
        let krusk = Character { ..Default::default() };
        let tordek = Character { hit_points: 1, constitution: 9, ..Default::default() };
        let combatants = Combatants { attacker: krusk, defender: tordek };

        let new_combatants = attack(combatants, 9);

        assert!(Alive == new_combatants.defender.vitality);
    }

    #[test]
    fn test_attacking_gives_experience() {
        let krusk = Character { ..Default::default() };
        let tordek = Character { ..Default::default() };
        let combatants = Combatants { attacker: krusk, defender: tordek };

        let new_combatants = attack(combatants, 10);

        assert_eq!(10, new_combatants.attacker.experience);
    }
}
