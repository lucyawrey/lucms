#[derive(Object)]
struct Todo {
    id: i64,
    description: String,
    done: bool,
}

type TodoResponse = Result<Json<Vec<Todo>>>;

struct TodosApi;

#[OpenApi]
impl TodosApi {
    #[oai(path = "/todos", method = "post")]
    async fn create(
        &self,
        pool: Data<&SqlitePool>,
        description: PlainText<String>,
    ) -> Result<Json<i64>> {
        let id = sqlx::query!(
	        "insert into todos (description) values (?)",
            description.0
		)
		.execute(pool.0)
		.await
		.map_err(InternalServerError)?
		.last_insert_rowid();
		
        Ok(Json(id))
    }

    #[oai(path = "/todos", method = "get")]
    async fn get_all(&self, pool: Data<&SqlitePool>
    ) -> TodoResponse {
        let todos = sqlx::query_as!(
	        Todo, 
	        "SELECT * FROM todos"
	    )
		.fetch_all(pool.0)
		.await
		.unwrap();
        
        Ok(Json(todos))
    }
}