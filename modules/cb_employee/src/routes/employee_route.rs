use obj_traits::controller::controller_crud::ControllerCRUD;
use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
use obj_traits::request::request_param::RequestParam;
use rocket::{delete, get, http::Status, options, patch, post, serde::json::Json};

use crate::controllers::employee_controller::EmployeeController;
use crate::models::employee::{NewEmployee, PatchEmployee};
use crate::models::employee_filter::EmployeeFilter;

#[get("/employee?<limit>&<offset>")]
pub fn get_employees(mut limit: Option<i32>, mut offset: Option<i32>) -> Json<serde_json::Value> {
    if limit.is_none() {
        limit = Some(10);
    };
    if offset.is_none() {
        offset = Some(0);
    };
    let params = RequestParam::new(PaginationParam::new(limit, offset), None);
    println!("{:?}", params);
    crab_rocket_schema::update_reload::update_reload_count();
    let resp = EmployeeController::get_all(&params).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}
#[post("/employee/filter", data = "<param>")]
pub fn filter_employees(
    param: Option<Json<RequestParam<PaginationParam, EmployeeFilter>>>,
) -> Json<serde_json::Value> {
    println!("{:?}", param);
    let param = param.unwrap().into_inner();
    crab_rocket_schema::update_reload::update_reload_count();
    let resp = EmployeeController::filter(&param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[get("/employee/<id>")]
pub fn get_employee_by_id(id: i32) -> Json<serde_json::Value> {
    crab_rocket_schema::update_reload::update_reload_count();
    let resp = EmployeeController::get_by_id(id).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/employee", data = "<employee>")]
pub fn insert_single_employee(employee: Json<NewEmployee>) -> Json<serde_json::Value> {
    let mut obj: NewEmployee = employee.into_inner();

    let resp = EmployeeController::add_single(&mut obj).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[delete("/employee/<id>")]
pub fn delete_employee_by_id(id: i32) -> Json<serde_json::Value> {
    let resp = EmployeeController::delete_by_id(id).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[patch("/employee/<id>", data = "<task>")]
pub fn update_employee_by_id(id: i32, task: Json<PatchEmployee>) -> Json<serde_json::Value> {
    let resp = EmployeeController::update_by_id(id, &task).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[options("/employee")]
pub fn options_employee() -> Status {
    Status::Ok
}
