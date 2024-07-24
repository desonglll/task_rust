use rocket::{delete, get, http::Status, options, patch, post, serde::json::Json};
use rocket::State;

use crab_rocket_schema::DbPool;
use obj_traits::controller::controller_crud::ControllerCRUD;
use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
use obj_traits::request::request_param::RequestParam;

use crate::controllers::user_controller::UserController;
use crate::models::user::{PatchUser, PostUser};
use crate::models::user_filter::UserFilter;

#[get("/user?<limit>&<offset>")]
pub fn get_users(
    pool: &State<DbPool>,
    mut limit: Option<i32>,
    mut offset: Option<i32>,
) -> Json<serde_json::Value> {
    if limit.is_none() {
        limit = Some(10);
    };
    if offset.is_none() {
        offset = Some(0);
    };
    let params = RequestParam::new(Some(PaginationParam::new(limit, offset)), None);
    println!("{:?}", params);
    crab_rocket_schema::update_reload::update_reload_count(pool);
    let resp = UserController::get_all(pool, &params).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/user/filter", data = "<param>")]
pub fn filter_users(
    pool: &State<DbPool>,
    param: Option<Json<RequestParam<UserFilter>>>,
) -> Json<serde_json::Value> {
    println!("{:?}", param);
    let param = param.unwrap_or(Json(RequestParam::default()));
    let param = param.into_inner();
    crab_rocket_schema::update_reload::update_reload_count(pool);
    let resp = UserController::filter(pool, &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[get("/user/<id>")]
pub fn get_user_by_id(pool: &State<DbPool>, id: i32) -> Json<serde_json::Value> {
    crab_rocket_schema::update_reload::update_reload_count(pool);
    let resp = UserController::get_by_id(pool, id).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/user", data = "<user>")]
pub fn insert_single_user(pool: &State<DbPool>, user: Json<PostUser>) -> Json<serde_json::Value> {
    let mut obj: PostUser = user.into_inner();

    let resp = UserController::add_single(pool, &mut obj).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[delete("/user/<id>")]
pub fn delete_user_by_id(pool: &State<DbPool>, id: i32) -> Json<serde_json::Value> {
    let resp = UserController::delete_by_id(pool, id).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[patch("/user/<id>", data = "<task>")]
pub fn update_user_by_id(
    pool: &State<DbPool>,
    id: i32,
    task: Json<PatchUser>,
) -> Json<serde_json::Value> {
    let resp = UserController::update_by_id(pool, id, &task).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[options("/user")]
pub fn options_user() -> Status {
    Status::Ok
}
