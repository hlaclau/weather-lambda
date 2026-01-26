use lambda_runtime::{tracing, Error};

mod event_handler;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    let _ = dotenvy::dotenv();

    #[cfg(feature = "local")]
    {
        event_handler::fetch_and_notify().await?;
        return Ok(());
    }

    #[cfg(not(feature = "local"))]
    {
        use lambda_runtime::{run, service_fn};
        run(service_fn(event_handler::function_handler)).await
    }
}
