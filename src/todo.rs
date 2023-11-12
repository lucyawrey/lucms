use poem::{error::{InternalServerError, BadRequest}, Result, web::Data};
use poem_openapi::{OpenApi, payload::{PlainText, Json}, Object, param::Path};
use sqlx::{SqlitePool, QueryBuilder, Sqlite};
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
    async fn update(
        &self,
        pool: Data<&SqlitePool>,
        id: Path<i64>,
        update: Json<UpdateTodo>,
    ) -> Result<()> {
        if update.description.is_none() && update.done.is_none() {
            return Err(BadRequest(StrError::new("request does not contain any valid fields for update in JSON body")));
        }

        let mut query_builder: QueryBuilder<Sqlite> =
            QueryBuilder::new("UPDATE todos SET ");

        if let Some(description) = &update.description {
            query_builder.push("(description) = ");
            query_builder.push_bind(description);
        }
        if update.description.is_some() && update.done.is_some() {
            query_builder.push(", ");
        }
        if let Some(done) = &update.done {
            query_builder.push("(done) = ");
            query_builder.push_bind(done);
        }

        query_builder.push(" WHERE (id) = ");
        query_builder.push_bind(id.0);

        query_builder.build()
            .execute(pool.0)
            .await
            .map_err(InternalServerError)?;
        Ok(())
    }
}
