use std::{fmt::{Display, Formatter, Result}, error::Error};
use sqlx::{QueryBuilder, Sqlite};

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


pub type SetOption<'a> = Option<(String, Box<dyn Display + 'a>)>;

pub fn new_set_option<'a, T: Display>(name: &str, option: &'a Option<T>)
    -> SetOption<'a> {
    match option {
        Some(val) => Some((name.to_string(), Box::new(val))),
        None => None,
    }
}

pub fn push_query_set_list(
    query_builder: &mut QueryBuilder<Sqlite>,
    options: Vec<SetOption>) -> bool {
    let mut any = false;
    for option in options {
        if let Some((name, val)) = option {
            if any {
                query_builder.push(", ");
            };
            query_builder.push(format!("({name}) = "));
            query_builder.push_bind(val.to_string());
            any = true;
        }
    }
    return any;
}
