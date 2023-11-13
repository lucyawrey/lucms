use std::fmt::Display;

use sqlx::{QueryBuilder, Sqlite, Encode, Type, query_builder::Separated};

pub trait QueryBuilderExtension<'args> {
    fn push_option<'qb, T>(&'qb mut self, option: Option<T>, name: &str) -> Separated<'qb, 'args, Sqlite, &str>
    where T: 'args + Encode<'args, Sqlite> + Send + Type<Sqlite>, 'args: 'qb;
}

impl<'args> QueryBuilderExtension<'args> for QueryBuilder<'args, Sqlite> {
    fn push_option<'qb, T>(&'qb mut self, option: Option<T>, name: &str) -> Separated<'qb, 'args, Sqlite, &str>
    where T: 'args + Encode<'args, Sqlite> + Send + Type<Sqlite>, 'args: 'qb {
        let mut separated = self.separated(", ");
        separated.push_option(option, name);
        separated
    }
}

pub trait SeparatedExtension<'args> {
    fn push_option<T>(&mut self, option: Option<T>, name: &str) -> &mut Self
    where T: 'args + Encode<'args, Sqlite> + Send + Type<Sqlite>;
}

impl<'qb, 'args, Sep: Display> SeparatedExtension<'args> for Separated<'qb, 'args, Sqlite, Sep> {
    fn push_option<T>(&mut self, optional: Option<T>, name: &str) -> &mut Self
    where T: 'args + Encode<'args, Sqlite> + Send + Type<Sqlite> {
        if let Some(value) = optional {
            self.push(format!("({name}) = "))
                .push_bind_unseparated(value);
        }
        self
    }
}
