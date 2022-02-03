use fake::{Dummy, Fake, Faker};

use crate::ResultErr;

#[derive(Debug, Dummy)]
pub struct Person {
    name: String,
    id: uuid::Uuid,
}

impl Identifiable for Person {
    fn id(&self) -> u128 {
        self.id.as_u128()
    }
}

#[derive(Debug, Dummy)]
pub struct Table {
    dimensions: (usize, usize),
    id: usize,
}

impl Identifiable for Table {
    fn id(&self) -> u128 {
        self.id as u128
    }
}

pub trait Identifiable {
    fn id(&self) -> u128;
}

pub fn play_with_trait_objects() -> ResultErr<()> {
    let identifiables: Vec<Box<dyn Identifiable>> = vec![
        Box::new(Faker.fake::<Person>()),
        Box::new(Faker.fake::<Person>()),
        Box::new(Faker.fake::<Table>()),
        Box::new(Faker.fake::<Person>()),
    ];

    println!(
        "{:?}",
        identifiables.iter().map(|i| i.id()).collect::<Vec<u128>>()
    );
    Ok(())
}
