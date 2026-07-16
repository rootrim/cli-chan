use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Config {
    anime_girls: HashMap<String, Personality>,
}

#[derive(Deserialize)]
pub struct Personality {
    personality: String,
    hobbies: Option<Vec<String>>,
}

impl Personality {
    pub fn summary(&self) -> String {
        let mut summary = String::new();

        summary.push_str(&self.personality);

        if let Some(hobbies) = &self.hobbies {
            match hobbies.len() {
                0 => (),
                1 => summary.push_str(&format!(" Your hobbie is {}.", hobbies[0])),
                _ => {
                    let mut hobbie_string = String::new();
                    hobbie_string.push_str("Your hobbies are ");
                    for hobbie in &hobbies[..hobbies.len().saturating_sub(1)] {
                        hobbie_string.push_str(&format!("{hobbie}, "));
                    }
                    hobbie_string.push_str(hobbies.last().unwrap());
                    summary.push_str(&format!(" {hobbie_string}."));
                }
            }
        }

        summary
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn personality_summaring_single_hobbie() {
        let personlaity_text =
            "You are Kuro, a jaded terminal spirit who has lived inside command lines for decades."
                .to_string();

        let personality = Personality {
            personality: personlaity_text,
            hobbies: Some(vec!["coding".into()]),
        };

        assert_eq!(personality.summary(), "You are Kuro, a jaded terminal spirit who has lived inside command lines for decades. Your hobbie is coding.".to_string());
    }

    #[test]
    fn personality_summaring_multiple_hobbies() {
        let personlaity_text =
            "You are Kuro, a jaded terminal spirit who has lived inside command lines for decades."
                .to_string();

        let personality = Personality {
            personality: personlaity_text,
            hobbies: Some(vec!["coding".into(), "hacking".into()]),
        };

        assert_eq!(personality.summary(), "You are Kuro, a jaded terminal spirit who has lived inside command lines for decades. Your hobbies are coding, hacking.".to_string());
    }

    #[test]
    fn personality_summaring_no_hobbies() {
        let personlaity_text =
            "You are Kuro, a jaded terminal spirit who has lived inside command lines for decades."
                .to_string();

        let personality = Personality {
            personality: personlaity_text,
            hobbies: None,
        };

        assert_eq!(
            personality.summary(),
            "You are Kuro, a jaded terminal spirit who has lived inside command lines for decades."
                .to_string()
        );
    }
}
