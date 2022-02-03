use std::{io, fs::File};
use std::io::prelude::*;
use chrono::Local;

fn main() {
    let mut state = Thunk::empty();
    let mut input = String::new();
    loop {
        io::stdin().read_line(&mut input).expect("failed to read stdin");
        let input_chr = input.chars().next();
        if let Some('e') = input_chr {
            if state.run().is_err() {
                println!("Received error while running Thunk");
            }
            return;
        } 
        state = state.chain_fn(match input_chr {
            Some('a') => Box::new(|| {println!("hello world!"); Ok(())}),
            Some('b') => Box::new(create_foo),
            Some('c') => Box::new(read_foo),
            _ => {
                let time_constructed = Local::now().to_rfc3339();
                Box::new(move || {
                    let time_ran = Local::now().to_rfc3339();
                    println!("Thunk at {}, ran at {}", time_constructed, time_ran);
                    Ok(())
            })}
        });
        input.clear();
    }
}

type EmptyResult = Result<(), ()>;

struct Thunk(Box<dyn FnOnce() -> EmptyResult>);

impl Thunk {
    fn new(func: Box<dyn FnOnce() -> EmptyResult>) -> Self {
        Self(func)
    }

    fn empty() -> Self {
        Self(Box::new(|| Ok(())))
    }

    fn run(self) -> EmptyResult {
        self.0()
    }

    fn chain_fn(self, other: Box<dyn FnOnce() -> EmptyResult>) -> Self {
        self.chain(Thunk::new(other))
    }

    fn chain(self, other: Self) -> Self {
        Self::new(Box::new(move || 
            self.run().and_then(|_| other.run())
        ))
    }
}

fn create_foo() -> EmptyResult {
    File::create("foo.txt")
        .and_then(|mut f| {
            f.write_all(b"Yo!")
        }).map_err(|_| ())
}

fn read_foo() -> EmptyResult {
    let mut contents = String::new();
    let res = File::open("foo.txt")
        .and_then(|mut f| f.read_to_string(&mut contents));

    if res.is_ok() {
        println!("{contents}");
        Ok(())
    } else {
        Err(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, 1);
    }
}
