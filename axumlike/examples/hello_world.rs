use axumlike;


// Hander or  Endpoint
async fn hello() -> &'static str {
    "<h1>Hello, World!</h1>"
}


#[tokio::main]
async fn main() {
    
    let app = axumlike::app()
        .route("/", 
            post(hello)
            .get(other_handler)
        );
        .route("/users/:id", any(hello)); 
    
    axumlike::start(([127, 0, 0, 1], 3000))
        .serve(app.into_make_service())
        .await
        .unwrap();

    // let app = Router::new().route("/", get(handler));
    // run it
    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // println!("listening on {}", addr);
    // axumlike::Server::bind(&addr)
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();

    
}


