use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

async fn handle_request(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello, World!")))
}

#[tokio::main]
async fn main() {
    // Define the socket address to listen on
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Create a service handler
    let make_svc = make_service_fn(|_conn| {
        async { Ok::<_, Infallible>(service_fn(handle_request)) }
    });

    // Create the server
    let server = Server::bind(&addr).serve(make_svc);

    // Run the server
    println!("Listening on http://{}", addr);
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
