mod hello;
mod todo;

use hello::HelloApi;
use todo::TodosApi;
use poem::{listener::TcpListener, Route, Server, EndpointExt, web::Redirect, endpoint::StaticFilesEndpoint, handler, get};
use poem_openapi::OpenApiService;
use sqlx::SqlitePool;
use std::error::Error;

const API_SET: (HelloApi, TodosApi) = (HelloApi, TodosApi);

const DB_FILENAME: &str = "sqlite:lucms.db";

#[tokio::main]
async fn main()
    -> Result<(), Box<dyn Error>> {

    color_eyre::install()?;

    let db_pool = 
	    SqlitePool::connect(DB_FILENAME).await?;

    let api_service =
        OpenApiService::new(API_SET, "LuCMS API (in R&D phase)", "1.0.0")
        .server("http://localhost:3000/api");

    let docs_service = api_service.openapi_explorer();

    let app = Route::new()
        .nest("/", get(index_handler_redirect))
        .nest("/api", api_service)
        .nest("/docs", docs_service)
        .nest("/admin", StaticFilesEndpoint::new("./static").show_files_listing().index_file("index.html"))
        .data(db_pool);

    

    let _ = Server::new(TcpListener::bind("localhost:3000"))
        .run(app)
        .await;

    Ok(())
}

#[handler]
async fn index_handler_redirect() -> Redirect {
    Redirect::temporary("/admin")
}
