use poem::{error::{InternalServerError, BadRequest}, Result, web::Data};
use poem_openapi::{OpenApi, payload::{PlainText, Json}, Object, param::Path};
use sqlx::{SqlitePool, QueryBuilder, Sqlite};
use crate::utilities::{StrError, push_query_set_list, new_set_option};

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
        let mut query_builder: QueryBuilder<Sqlite> =
            QueryBuilder::new("UPDATE todos SET ");

        let any = push_query_set_list(&mut query_builder, vec![
            new_set_option("description", &update.description),
            new_set_option("done", &update.done),
        ]);

        if !any {
            return Err(BadRequest(StrError::new("request does not contain any valid fields for update in JSON body")));
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
