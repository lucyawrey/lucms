use std::{fmt::{Display, Formatter, Result}, error::Error};
use sqlx::{QueryBuilder, Sqlite, Encode, Type, query_builder::Separated};

#[derive(Debug)]
pub struct StrError {
    msg: String,
}

impl StrError {
    pub fn new(msg: &str) -> StrError {
        StrError { msg: msg.to_string() }
    }
}

impl Display for StrError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for StrError {}

pub trait QueryBuilderOverrides<'args> {
    fn push_option<T>(&mut self, option: Option<T>, name: &str) -> &mut Separated<'args, 'args, Sqlite, &str>
    where T: 'args + Encode<'args, Sqlite> + Send + Type<Sqlite>;
}

pub trait SeparatedOverrides<'args> {
    fn push_option<T>(&mut self, option: Option<T>, name: &str) -> &mut Self
    where T: 'args + Encode<'args, Sqlite> + Send + Type<Sqlite>;
}

impl<'args> QueryBuilderOverrides<'args> for QueryBuilder<'args, Sqlite> {
    fn push_option<T>(&mut self, option: Option<T>, name: &str) -> &mut Separated<'args, 'args, Sqlite, &str>
    where T: 'args + Encode<'args, Sqlite> + Send + Type<Sqlite> {
        self.separated(", ")
        .push_option(option, name)
    }
}

impl<'args, Sep: Display> SeparatedOverrides<'args> for Separated<'args, 'args, Sqlite, Sep> {
    fn push_option<T>(&mut self, optional: Option<T>, name: &str) -> &mut Self
    where T: 'args + Encode<'args, Sqlite> + Send + Type<Sqlite> {
        if let Some(value) = optional {
            self.push(format!("({name}) = "));
            self.push_bind(value);
        }
        self
    }
}
