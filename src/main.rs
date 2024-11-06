//! main.rs
use std::net::TcpListener;

use zero2prod::run;


#[tokio::main]
async fn main() {
	let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
	// We retrieve the port assigned to us by the OS.
	let port = listener.local_addr().unwrap().port();
	let server = run(listener).expect("Failed to bind address");
	let _ = tokio::spawn(server);
	// We return the application port to the caller.
	let finalport = format!("http://127.0.0.1:{}", port);
    println!("Server running on {}", finalport);
}
