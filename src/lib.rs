struct Character {
    name: String,
}

#[cfg(test)]
mod test {
    use super::Character;

    #[test]
    fn test_character_has_a_name() {
        let krusk = Character { name: "Krusk".to_string() };

        assert_eq!("Krusk".to_string(), krusk.name);
    }
}
