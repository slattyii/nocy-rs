mod app;
mod fs;
mod player;
mod provider;
mod renderer;
mod version;

#[tokio::main]
async fn main() {
	app::Nocy::init().run().await.unwrap();
}
