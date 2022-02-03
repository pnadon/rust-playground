use std::{
    fs::File,
    io::{self, Read, Write},
};

use fake::{Dummy, Fake, Faker};
use reqwest::get;
use serde::{Deserialize, Serialize};

use crate::ResultErr;

#[derive(Deserialize, Serialize, Debug)]
pub struct Data {
    data: Vec<Nation>,
    #[serde(skip, alias = "source")]
    _source: String,
}

impl Data {
    pub fn nations(&self) -> &[Nation] {
        &self.data
    }

    pub fn from_nations(nations: Vec<Nation>) -> Self {
        Self {
            data: nations,
            _source: String::default(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Default, Dummy)]
pub struct Nation {
    #[serde(alias = "ID Nation")]
    id_nation: String,
    #[serde(alias = "Nation")]
    nation: String,
    #[serde(alias = "ID Year")]
    id_year: usize,
    #[serde(alias = "Year")]
    year: String,
    #[serde(alias = "Population")]
    population: usize,
    #[serde(alias = "Slug Nation")]
    slug_nation: String,
}

impl Nation {
    pub fn population(&self) -> usize {
        self.population
    }

    pub fn default_with_population(population: usize) -> Self {
        Nation {
            population,
            ..Default::default()
        }
    }

    pub fn fake_with_population(population: usize) -> Self {
        Nation {
            population,
            ..Faker.fake()
        }
    }
}

pub async fn get_pops() -> ResultErr<()> {
    // let mut buffer = vec![0; 256];
    // io::stdin().read_exact(&mut buffer)?;
    let mut str_buf = String::new();
    io::stdin().read_to_string(&mut str_buf)?;

    let buffer = str_buf.bytes().collect::<Vec<u8>>();

    let url = std::str::from_utf8(&buffer)?;

    let resp = get(url).await?;

    let data: Data = serde_json::from_slice(resp.bytes().await?.as_ref())?;

    println!("pop: {}", sum_population(&data));
    let mut file = File::create("example.txt")?;
    let num_written = file.write(format!("{:?}", data).as_bytes())?;
    println!("{num_written}");
    Ok(())
}

fn sum_population(data: &Data) -> usize {
    data.nations()
        .iter()
        .map(|nation| nation.population())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_population_returns_correct_sum() {
        let data = Data::from_nations(vec![
            Nation::default_with_population(5),
            Nation::default_with_population(3),
            Nation::default_with_population(67),
        ]);
        assert_eq!(sum_population(&data), 5 + 3 + 67);
    }
}
