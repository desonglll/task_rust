use rocket::{delete, get, http::Status, options, patch, post, serde::json::Json};
use rocket::State;

use crab_rocket_schema::DbPool;
use obj_traits::controller::controller_crud::ControllerCRUD;
use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
use obj_traits::request::request_param::RequestParam;

use crate::controllers::shipment_controller::ShipmentController;
use crate::models::shipment::{PatchShipment, PostShipment};
use crate::models::shipment_filter::ShipmentFilter;

#[get("/shipment?<limit>&<offset>")]
pub fn get_shipments(
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
    let resp = ShipmentController::get_all(pool, &params).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/shipment/filter", data = "<param>")]
pub fn filter_shipments(
    pool: &State<DbPool>,
    param: Option<Json<RequestParam<ShipmentFilter>>>,
) -> Json<serde_json::Value> {
    println!("{:?}", param);
    let param = param.unwrap_or(Json(RequestParam::default()));
    let param = param.into_inner();
    crab_rocket_schema::update_reload::update_reload_count(pool);
    let resp = ShipmentController::filter(pool, &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[get("/shipment/<id>")]
pub fn get_shipment_by_id(pool: &State<DbPool>, id: i32) -> Json<serde_json::Value> {
    crab_rocket_schema::update_reload::update_reload_count(pool);
    let resp = ShipmentController::get_by_id(pool, id).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/shipment", data = "<shipment>")]
pub fn insert_single_shipment(
    pool: &State<DbPool>,
    shipment: Json<PostShipment>,
) -> Json<serde_json::Value> {
    let mut obj: PostShipment = shipment.into_inner();

    let resp = ShipmentController::add_single(pool, &mut obj).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[delete("/shipment/<id>")]
pub fn delete_shipment_by_id(pool: &State<DbPool>, id: i32) -> Json<serde_json::Value> {
    let resp = ShipmentController::delete_by_id(pool, id).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[patch("/shipment/<id>", data = "<task>")]
pub fn update_shipment_by_id(
    pool: &State<DbPool>,
    id: i32,
    task: Json<PatchShipment>,
) -> Json<serde_json::Value> {
    let resp = ShipmentController::update_by_id(pool, id, &task).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[options("/shipment")]
pub fn options_shipment() -> Status {
    Status::Ok
}
