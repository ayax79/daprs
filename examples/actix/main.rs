

fn main() {
    println!("Starting my rust dapr app");
}
fn init() -> Result<(), DaprHelloError> {
    init_with_port(env_dapper_http_port()?)
}

fn init_with_port(port: u16) -> Result<(), DaprHelloError> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
    })
    .bind(format!("127.0.0.1:{}", dapper_http_port()?))
    .unwrap()
    .run()
    .unwrap();
    Ok(())
}

