use crate::models::{Status, CreateTodoList, ResultResponse, AppState};
use crate::db;
use crate::errors::{AppError};
use slog::{Logger, o, crit, error};

use deadpool_postgres::{Pool, Client};
use actix_web::{web, Responder, HttpResponse};

pub async fn get_client(pool: Pool, log: Logger) -> Result<Client, AppError> {
    return pool.get().await
        .map_err(|err| {
            let sublog = log.new(o!("cause" => err.to_string()));
            crit!(sublog, "Error creating client");
            AppError::db_error(err)
        })
}
 
pub fn log_error(log: Logger) -> Box<dyn Fn(AppError) -> AppError> {
   Box::new(move |err| {
        let sublog = log.new(o!("cause" => err.to_string()));
        error!(sublog, "{}", err.message());
        err
    })
}

pub async fn status() -> impl Responder {
    web::HttpResponse::Ok()
        .json(Status {status: "Up".to_string()})
}

pub async fn get_todos(state: web::Data<AppState>) -> Result<impl Responder, AppError> {

    let log = state.logger.new(o!("handler" => "get_todos"));

    let client = get_client(state.pool.clone(), log.clone()).await?;
    
    let result = db::get_todos(&client).await;
    result.map(|todos| HttpResponse::Ok().json(todos))
        .map_err(log_error(log))
}

pub async fn get_todo(list_id: web::Path<(i32,)>, state: web::Data<AppState>) -> Result<impl Responder, AppError> {
    let sublog = state.logger.new(o!(
        "handler" => "get_todo",
        "list_id" => list_id.0
    ));

    let client: Client = get_client(state.pool.clone(), sublog.clone()).await?;

    let result = db::get_todo(&client, list_id.0).await;

    result
        .map(|todo| HttpResponse::Ok().json(todo))
        .map_err(log_error(sublog))
}

pub async fn get_items(state: web::Data<AppState>, path: web::Path<(i32,)>) -> Result<impl Responder, AppError> {

    let log = state.logger.new(o!("handler" => "get_items"));

    let client = get_client(state.pool.clone(), log.clone()).await?;

    let result = db::get_items(&client, path.0).await;

    result.map(|items| HttpResponse::Ok().json(items))
        .map_err(log_error(log))
}

pub async fn create_todo(state: web::Data<AppState>, json: web::Json<CreateTodoList>) -> Result<impl Responder, AppError> {

    let log = state.logger.new(o!("handler" => "create_todo"));

    let client = get_client(state.pool.clone(), log.clone()).await?;

    let result = db::create_todo(&client, json.title.clone()).await;

    result.map(|todo| HttpResponse::Ok().json(todo))
        .map_err(log_error(log))
}

pub async fn check_item(state: web::Data<AppState>, path: web::Path<(i32,i32)>) -> Result<impl Responder, AppError> {

    let log = state.logger.new(o!("handler" => "create_todo"));

    let client = get_client(state.pool.clone(), log.clone()).await?;

    let result = db::check_item(&client, path.0, path.1).await;

    result.map(|updated: bool| HttpResponse::Ok().json(ResultResponse{success: updated}))
        .map_err(log_error(log))
}