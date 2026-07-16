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
                    for hobbie in hobbies {
                        hobbie_string.push_str(&format!("{hobbie}, "))
                    }
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
    fn personality_summaring() {
        let personlaity_text =
            "You are Kuro, a jaded terminal spirit who has lived inside command lines for decades."
                .to_string();

        let personality = Personality {
            personality: personlaity_text,
            hobbies: Some(vec!["coding".into()]),
        };

        assert_eq!(personality.summary(), "You are Kuro, a jaded terminal spirit who has lived inside command lines for decades. Your hobbie is coding.".to_string());
    }
}
