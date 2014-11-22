#[deriving(PartialEq)]
enum Alignment {
    Good,
    Neutral,
    Evil
}

struct Character {
    name: String,
    alignment: Alignment
}

#[cfg(test)]
#[test]
fn test_character_has_a_name() {
    let krusk = Character {
        name: "Krusk".to_string(),
        alignment: Neutral
    };

    assert_eq!("Krusk".to_string(), krusk.name);
}

#[cfg(test)]
#[test]
fn test_character_alignment_good() {
    let gimble = Character {
        name: "Gimble".to_string(),
        alignment: Good
    };

    assert!(Good == gimble.alignment);
}

#[cfg(test)]
#[test]
fn test_character_alignment_neutral() {
    let jozan = Character {
        name: "Jozan".to_string(),
        alignment: Neutral
    };

    assert!(Neutral == jozan.alignment);
}

#[cfg(test)]
#[test]
fn test_character_alignment_evil() {
    let vadania = Character {
        name: "Vadania".to_string(),
        alignment: Evil
    };

    assert!(Evil == vadania.alignment);
}
