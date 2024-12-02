//@ tests/master_deck_test.rs

#[cfg(feature = "serde")]
use serde::Deserialize;

use onejoker::prelude::*;

#[cfg(feature = "serde")]
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
struct MasterDeckInfo {
    name: String,
    dups_allowed: bool,
    low_aces: bool,
    aliases: Vec<String>,
    card_list: Vec<u8>
}

/// JSON file structure
#[cfg(feature = "serde")]
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
struct MasterDeckDataFile(Vec<MasterDeckInfo>);

#[test]
#[cfg(feature = "serde")]
fn test_masterdeck_file() -> OjResult<()> {
    use std::fs::File;
    use std::io::BufReader;

    let file = File::open("../data/json/master_decks.jsonc")?;
    let mut reader = BufReader::new(file);
    let data: MasterDeckDataFile = serde_json5::from_reader(&mut reader)?;

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

#[test]
fn test_no_json() -> OjResult<()> {
    Ok(())
}
