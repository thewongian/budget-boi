use super::handlers;
use super::mongo::{User, Expense, Db, LoginInfo};
use warp::Filter;

pub fn budget(db: Db) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    expenses_list(db.clone())
        .or(expense_create(db.clone()))
        .or(income_create(db.clone()))
        .or(user_create(db.clone()))
        .or(expense_delete(db.clone()))
        .or(login(db.clone()))
}

pub fn expenses_list(db: Db) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("expenses" / String)
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::list_expenses)
}


pub fn expense_create(db: Db) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("create_expense" / String)
        .and(warp::post())
        .and(json_body_expense())
        .and(with_db(db))
        .and_then(handlers::add_expense)
}

pub fn income_create(db: Db) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("create_income" / String)
        .and(warp::post())
        .and(json_body_expense())
        .and(with_db(db))
        .and_then(handlers::add_income)
}

pub fn user_create(db: Db) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("create_user")
        .and(warp::post())
        .and(json_body_user())
        .and(with_db(db))
        .and_then(handlers::add_user)
}

pub fn expense_delete(db: Db) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    
    warp::path!("expenses" / String / u64)
        .and(warp::delete())
        .and(with_db(db))
        .and_then(handlers::delete_expense)
}

pub fn login(db: Db) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("login")
    .and(warp::post())
    .and(json_body_login())
    .and(with_db(db))
    .and_then(handlers::user_login)
}

fn json_body_user() -> impl Filter<Extract = (User,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn json_body_login() -> impl Filter<Extract = (LoginInfo,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn json_body_expense() -> impl Filter<Extract = (Expense,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
