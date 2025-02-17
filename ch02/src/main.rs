mod lib;

// Gcd service
//
// use lib::gcd;
// use actix_web::{App, HttpServer};

// #[actix_web::main] // or #[tokio::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().service(gcd::init_gcd).service(gcd::post_gcd))
//         .bind(("127.0.0.1", 8080))?
//         .run()
//         .await
// }

//Concurrency service
use lib::concurrency::render_image;
fn main() {
	let threads = 8;
	render_image(threads);
}

// Quickreplace service
// use lib::quickreplace::quick_replace;

// fn main() {
//     quick_replace();
// }
