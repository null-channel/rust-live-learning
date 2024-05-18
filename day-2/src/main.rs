use axum::routing::get;
use axum::Router;
use clap::Parser;

#[derive(Parser)]
struct CliArgs {
    #[arg(short, long)]
    port: String,
}

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();

    let app = Router::new().route("/", get(root));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", args.port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
