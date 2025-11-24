use axum::{routing::get, Router};
use axum_test::TestServer; // Add this import

async fn hello_world() -> &'static str {
    "Hello from MediaClay API!"
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(hello_world));

    Ok(router.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use shuttle_axum::ShuttleAxum; // Import ShuttleAxum

    async fn create_test_app() -> Router {
        let shuttle_axum_service: ShuttleAxum = main().await.expect("Failed to create app");
        shuttle_axum_service.into_router()
    }

    #[tokio::test]
    async fn test_hello_world_api() {
        let app = create_test_app().await;
        let server = TestServer::new(app).expect("Failed to create test server");

        let response = server.get("/").await;

        response.assert_ok().assert_body_text("Hello from MediaClay API!");
    }
}
