use anyhow::Error;
use chrono::NaiveDateTime;

pub fn parse(name: &str) -> Result<Box<dyn Formatter>, Error> {
    if name.starts_with("human") {
        return Human::parse(name);
    }

    Err(Error::msg(format!("Unknown format '{}'", name)))
}

pub trait Formatter {
    fn format(&self, t: NaiveDateTime) -> String;
}

#[derive(Debug, Clone)]
struct Human {}

impl Human {
    fn parse(_name: &str) -> Result<Box<dyn Formatter>, Error> {
        Ok(Box::new(Human {}))
    }
}

impl Formatter for Human {
    fn format(&self, t: NaiveDateTime) -> String {
        t.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}
