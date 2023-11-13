mod todo;
mod utilities;
mod query_builder;

use poem::{listener::TcpListener, Route, Server, EndpointExt, web::Redirect, endpoint::StaticFilesEndpoint, handler, get};
use poem_openapi::OpenApiService;
use sqlx::SqlitePool;
use todo::TodosApi;
use std::{error::Error, env};

const API_SET: TodosApi = TodosApi;

#[tokio::main]
async fn main()
    -> Result<(), Box<dyn Error>> {

    color_eyre::install()?;
    dotenvy::dotenv().expect("No .env file found.");

    let database_url = env::var("DATABASE_URL").expect("Enviornment variable DATABASE_URL not found.");
    let host = env::var("HOST").expect("Enviornment variable HOST not found.");

    let db_pool = 
	    SqlitePool::connect(&database_url).await?;

    let api_service =
        OpenApiService::new(API_SET, "LuCMS API (in R&D phase)", "1.0.0")
        .server(format!("http://{host}/api"));

    let docs_service = api_service.openapi_explorer();

    let app = Route::new()
        .nest("/", get(index_handler_redirect))
        .nest("/api", api_service)
        .nest("/docs", docs_service)
        .nest("/admin", StaticFilesEndpoint::new("./static").show_files_listing().index_file("index.html"))
        .data(db_pool);

    println!("\n🚀 starting server at url: http://{host}\n");

    let _ = Server::new(TcpListener::bind(host))
        .run(app)
        .await;

    Ok(())
}

#[handler]
async fn index_handler_redirect() -> Redirect {
    Redirect::temporary("/admin")
}
