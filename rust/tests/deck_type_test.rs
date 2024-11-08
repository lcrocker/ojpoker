//@ tests/master_deck_test.rs

use std::fs;
use serde::Deserialize;
use onejoker::*;

#[derive(Deserialize, Debug)]
struct MasterDeckInfo {
    name: String,
    dups_allowed: bool,
    low_aces: bool,
    aliases: Vec<String>,
    card_list: Vec<u8>   
}

/// JSON file structure
#[derive(Deserialize, Debug)]
struct MasterDeckDataFile(Vec<MasterDeckInfo>);

#[test]
fn test_masterdeck_file() -> Result<(), OjError> {
    let text = fs::read_to_string("../data/json/master_decks.jsonc")?;
    let data = json5::from_str::<MasterDeckDataFile>(&text[..])?;

    for i in 0..data.0.len() {
        let info: &MasterDeckInfo = &data.0[i];
        let t = DeckType::by_name(&info.name);

        assert_eq!(t.dups_allowed(), info.dups_allowed);
        assert_eq!(t.low_aces(), info.low_aces);
        assert_eq!(t.card_list().len(), info.card_list.len());

        for j in 0..info.aliases.len() {
            assert_eq!(t, DeckType::by_name(&info.aliases[j]));
        }
    }
    Ok(())
}
