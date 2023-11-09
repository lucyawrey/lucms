use std::fmt::{Write, Display};

use poem::{error::{InternalServerError, BadRequest}, Result, web::Data};
use poem_openapi::{OpenApi, payload::{PlainText, Json}, Object, param::Path};
use sqlx::SqlitePool;

use crate::utilities::StrError;

/// Todo
#[derive(Object)]
struct Todo {
    id: i64,
    description: String,
    done: bool,
}

/// Todo
#[derive(Object)]
struct UpdateTodo {
    description: Option<String>,
    done: Option<bool>,
}

pub struct TodosApi;

#[OpenApi]
impl TodosApi {
    #[oai(path = "/todo", method = "post")]
    async fn create(
        &self,
        pool: Data<&SqlitePool>,
        description: PlainText<String>,
    ) -> Result<Json<i64>> {
        let id = sqlx::query!(
	        "INSERT INTO todos (description) VALUES (?)",
            description.0
		)
		.execute(pool.0)
		.await
		.map_err(InternalServerError)?
		.last_insert_rowid();
		
        Ok(Json(id))
    }

    #[oai(path = "/todo", method = "get")]
    async fn get_all(
        &self,
        pool: Data<&SqlitePool>
    ) -> Result<Json<Vec<Todo>>> {
        let todos = sqlx::query_as!(
	        Todo, 
	        "SELECT * FROM todos"
	    )
		.fetch_all(pool.0)
		.await
        .map_err(InternalServerError)?;
        
        Ok(Json(todos))
    }

    #[oai(path = "/todos/:id", method = "put")]
    async fn update(
        &self,
        pool: Data<&SqlitePool>,
        id: Path<i64>,
        update: Json<UpdateTodo>,
    ) -> Result<()> {
        let query_string_option = get_set_query_string_from_options(vec![
            opt_col_new("description", &update.description),
            opt_col_new("done", &update.done),
        ]);

        if let Some(set_query_string) = query_string_option {
            let sql = format!("UPDATE todos {set_query_string} WHERE (id) = ?");
            sqlx::query(&sql)
                .bind(id.0)
                .execute(pool.0)
                .await
                .map_err(InternalServerError)?;
            Ok(())
        } else {
            Err(BadRequest(StrError::new("test")))
        }
    }
}

type OptionalColumn<'a> = Option<(String, Box<dyn Display + 'a>)>;

fn opt_col_new<'a, T: Display>(name: &str, option: &'a Option<T>)
    -> OptionalColumn<'a> {
    match option {
        Some(val) => Some((name.to_string(), Box::new(val))),
        None => None,
    }
}

fn get_set_query_string_from_options(
    options: Vec<OptionalColumn>) -> Option<String> {
    let mut query_string = "SET ".to_string();
    let mut any = false;
    for option in options {
        if let Some((name, val)) = option {
            if any {
                query_string.write_str(", ").unwrap()
            };
            write!(query_string, "({name}) = \"{val}\"").unwrap();
            any = true;
        }
    }
    if any {
        Some(query_string)
    } else {
        None
    }
}
