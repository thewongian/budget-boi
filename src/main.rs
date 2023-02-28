mod server;
use server::{mongo, filters};
use warp::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let mut db = mongo::Db::new();
    let err = db.init().await;
    if err.is_err() {
        panic!("Database could not be initialized")
    }
    let api = filters::budget(db);
    
    let routes = api.with(warp::log("budget"));
    //nice
    warp::serve(routes).run(([127, 0, 0, 1], 6969)).await;
}
