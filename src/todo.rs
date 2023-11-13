use poem::{error::{InternalServerError, BadRequest}, Result, web::Data};
use poem_openapi::{OpenApi, payload::{PlainText, Json}, Object, param::Path};
use sqlx::{SqlitePool, QueryBuilder};
use crate::utilities::StrError;
use crate::query_builder::{QueryBuilderExtension, SeparatedExtension};

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
    #[oai(path = "/todos", method = "post")]
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

    #[oai(path = "/todos/:id", method = "get")]
    async fn get(&self, pool: Data<&SqlitePool>, id: Path<i64>)
        -> Result<Json<Todo>> {
        let todo: Option<Todo> =
            sqlx::query_as!(
                Todo,
                "SELECT * from todos WHERE (id) = (?)",
                id.0
            )
                .fetch_optional(pool.0)
                .await
                .map_err(InternalServerError)?;

        match todo {
            Some(todo) => Ok(Json(todo)),
            None => Err(BadRequest(StrError::new(&format!(
                "todo `{}` not found",
                id.0
            ))))
        }
    }


    #[oai(path = "/todos", method = "get")]
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

    #[oai(path = "/todos/:id", method = "delete")]
    async fn delete(&self, pool: Data<&SqlitePool>, id: Path<i64>)
        -> Result<()> {
        sqlx::query!(
            "DELETE from todos where (id) = (?)",
            id.0
        )
            .execute(pool.0)
            .await
            .map_err(InternalServerError)?;
        Ok(())
    }

    #[oai(path = "/todos/:id", method = "put")]
    async fn update2(
        &self,
        pool: Data<&SqlitePool>,
        id: Path<i64>,
        update: Json<UpdateTodo>,
    ) -> Result<()> {
        if update.description.is_none() && update.done.is_none() {
            return Err(BadRequest(StrError::new("request does not contain any valid fields for update in JSON body")));
        }

        let mut query_builder =
            QueryBuilder::new("UPDATE todos SET ");
        
        query_builder.push_option(update.description.to_owned(), "description")
            .push_option(update.done, "done");

        query_builder.push(" WHERE (id) = ")
            .push_bind(id.0)
            .build()
            .execute(pool.0)
            .await
            .map_err(InternalServerError)?;
        Ok(())
    }
}
