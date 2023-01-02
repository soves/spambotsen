use csv::Reader;
use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};
use std::{error::Error, path::Path};

fn parse_lines(pepepains: &mut ForsenLines) -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path("forsen_lines.csv")?;
    for result in rdr.records() {
        let record = result?;

        if let Some(line) = record.get(1) {
            pepepains.lines.push((
                line.trim().to_string(),
                match record.get(4) {
                    Some("Very rare") => 1,
                    Some("Rare") => 2,
                    Some("Uncommon") => 3,
                    Some("Common") => 4,
                    _ => 1,
                },
            ));
        }
    }
    Ok(())
}

#[derive(Debug)]
pub struct ForsenLines {
    lines: Vec<(String, usize)>,
}

impl ForsenLines {
    pub fn new() -> Self {
        let mut result = Self { lines: Vec::new() };
        parse_lines(&mut result).expect("Error parsing csv");
        result
    }

    pub fn get_random(&self) -> String {
        let mut rng = thread_rng();
        self.lines
            .choose_weighted(&mut rng, |item| item.1)
            .unwrap()
            .0
            .clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::forsen_lines::ForsenLines;

    #[test]
    fn random_line() {
        let mut pains = ForsenLines::new();
        println!("'{}'", pains.get_random())
    }
}