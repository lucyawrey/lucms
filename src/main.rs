mod hello;

use hello::HelloApi;
use poem::{listener::TcpListener, Route, Server};
use poem_openapi::OpenApiService;
use sqlx::{SqlitePool, Sqlite, migrate::MigrateDatabase};

const DB_FILENAME: &str = "sqlite:todos.db";

#[tokio::main]
async fn main()
    -> Result<(), Box<dyn std::error::Error>> {

    color_eyre::install()?;

    try_create_db(DB_FILENAME).await;
    let _db_pool = 
	    SqlitePool::connect(DB_FILENAME).await?;

    let hello_api_service =
        OpenApiService::new(HelloApi, "Hello API", "1.0").server("http://localhost:3000");
        
    let ui = hello_api_service.openapi_explorer();
    let app = Route::new().nest("/", hello_api_service).nest("/docs", ui);

    let _ = Server::new(TcpListener::bind("localhost:3000"))
        .run(app)
        .await;

    Ok(())
}

async fn try_create_db(db_filename: &str) {
    if !Sqlite::database_exists(db_filename).await.unwrap_or(false) {
        println!("Creating database {}", db_filename);
        match Sqlite::create_database(db_filename).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }
}
