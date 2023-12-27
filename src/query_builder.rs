use std::fmt::Display;

use sqlx::{QueryBuilder, Postgres, Encode, Type, query_builder::Separated};

pub trait QueryBuilderExtension<'args> {
    fn push_option<'qb, T>(&'qb mut self, option: Option<T>, name: &str) -> Separated<'qb, 'args, Postgres, &str>
    where T: 'args + Encode<'args, Postgres> + Send + Type<Postgres>, 'args: 'qb;
}

impl<'args> QueryBuilderExtension<'args> for QueryBuilder<'args, Postgres> {
    fn push_option<'qb, T>(&'qb mut self, option: Option<T>, name: &str) -> Separated<'qb, 'args, Postgres, &str>
    where T: 'args + Encode<'args, Postgres> + Send + Type<Postgres>, 'args: 'qb {
        let mut separated = self.separated(", ");
        separated.push_option(option, name);
        separated
    }
}

pub trait SeparatedExtension<'args> {
    fn push_option<T>(&mut self, option: Option<T>, name: &str) -> &mut Self
    where T: 'args + Encode<'args, Postgres> + Send + Type<Postgres>;
}

impl<'qb, 'args, Sep: Display> SeparatedExtension<'args> for Separated<'qb, 'args, Postgres, Sep> {
    fn push_option<T>(&mut self, optional: Option<T>, name: &str) -> &mut Self
    where T: 'args + Encode<'args, Postgres> + Send + Type<Postgres> {
        if let Some(value) = optional {
            self.push(format!("({name}) = "))
                .push_bind_unseparated(value);
        }
        self
    }
}
