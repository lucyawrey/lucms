use poem_openapi::{payload::PlainText, OpenApi, param::Query};

pub struct HelloApi;

#[OpenApi]
impl HelloApi {
    #[oai(path = "/hello", method = "get")]
    async fn index(&self) -> PlainText<&'static str> {
        PlainText("hello, world!")
    }

    #[oai(path = "/hello-x", method = "get")]
    async fn echo(&self, name: Query<Option<String>>)
		-> PlainText<String> {
        match name.0 {
            Some(name) => PlainText(format!("hi, {name}!")),
            None => PlainText("hi!".to_string()),
        }
	}
}
