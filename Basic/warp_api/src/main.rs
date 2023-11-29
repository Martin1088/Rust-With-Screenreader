use routes::routes;

mod handlers;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    let routes_api = routes();
    println!("start Server Test warp at localhost:8000");
    warp::serve(routes_api).run(([0, 0, 0, 0], 8000)).await;
}
