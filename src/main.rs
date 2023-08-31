mod server;
use std::env;
use dotenv::dotenv;
use server::{mongo, filters};
use warp::Filter;

#[tokio::main]
async fn main() {
    dotenv().ok();
    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=todos=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "todos=info");
    }
    pretty_env_logger::init();
    let mut db = mongo::Db::new();
    let err = db.init().await;
    if err.is_err() {
        eprintln!("{}", err.unwrap_err());
        panic!("Database could not be initialized");
    }
    let api = filters::budget(db);
    
    let routes = api.with(warp::log("budget"));
    //nice
    warp::serve(routes).run(([127, 0, 0, 1], 6969)).await;
}
