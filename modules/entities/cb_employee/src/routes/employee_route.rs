use crab_rocket_schema::DbPool;
use obj_traits::controller::controller_crud::ControllerCRUD;
use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
use obj_traits::request::request_param::RequestParam;
use rocket::State;
use rocket::{delete, get, http::Status, options, patch, post, serde::json::Json};

use crate::controllers::employee_controller::EmployeeController;
use crate::models::employee::{PatchEmployee, PostEmployee};
use crate::models::employee_filter::EmployeeFilter;

#[get("/employee?<limit>&<offset>")]
pub fn get_employees(
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
    let resp = EmployeeController::get_all(pool, &params).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/employee/filter", data = "<param>")]
pub fn filter_employees(
    pool: &State<DbPool>,
    param: Option<Json<RequestParam<EmployeeFilter>>>,
) -> Json<serde_json::Value> {
    println!("{:?}", param);
    let param = param.unwrap_or(Json(RequestParam::default()));
    let param = param.into_inner();
    crab_rocket_schema::update_reload::update_reload_count(pool);
    let resp = EmployeeController::filter(pool, &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[get("/employee/<id>")]
pub fn get_employee_by_id(pool: &State<DbPool>, id: i32) -> Json<serde_json::Value> {
    crab_rocket_schema::update_reload::update_reload_count(pool);
    let resp = EmployeeController::get_by_id(pool, id).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/employee", data = "<employee>")]
pub fn insert_single_employee(
    pool: &State<DbPool>,
    employee: Json<PostEmployee>,
) -> Json<serde_json::Value> {
    let mut obj: PostEmployee = employee.into_inner();

    let resp = EmployeeController::add_single(pool, &mut obj).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[delete("/employee/<id>")]
pub fn delete_employee_by_id(pool: &State<DbPool>, id: i32) -> Json<serde_json::Value> {
    let resp = EmployeeController::delete_by_id(pool, id).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[patch("/employee/<id>", data = "<task>")]
pub fn update_employee_by_id(
    pool: &State<DbPool>,
    id: i32,
    task: Json<PatchEmployee>,
) -> Json<serde_json::Value> {
    let resp = EmployeeController::update_by_id(pool, id, &task).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[options("/employee")]
pub fn options_employee() -> Status {
    Status::Ok
}
