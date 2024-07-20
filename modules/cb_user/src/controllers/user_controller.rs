use crate::models::user::{NewUser, PatchUser, User};
use crate::models::user_filter::UserFilter;
use crate::services::user_service::UserService;
use obj_traits::controller::controller_crud::{
    controller_add_single, controller_delete_by_id, controller_filter, controller_get_all,
    controller_get_by_id, controller_update_by_id, ControllerCRUD,
};
use obj_traits::request::pagination_request_param::PaginationParam;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::api_response::ApiResponse;
use obj_traits::response::data::Data;
use std::error::Error;

pub struct UserController {}

impl ControllerCRUD for UserController {
    type Item = User;
    type NewItem = NewUser;
    type PatchItem = PatchUser;
    type Param = RequestParam<PaginationParam, UserFilter>;
    fn get_all(
        param: &RequestParam<PaginationParam, UserFilter>,
    ) -> Result<ApiResponse<Data<Vec<Self::Item>>>, Box<dyn Error>> {
        controller_get_all::<Self::Item, UserService, UserFilter>(param)
    }

    fn get_by_id(pid: i32) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_get_by_id::<Self::Item, UserService>(pid)
    }

    fn add_single(obj: &mut NewUser) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_add_single::<Self::Item, UserService, NewUser>(obj)
    }

    fn delete_by_id(pid: i32) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_delete_by_id::<Self::Item, UserService>(pid)
    }

    fn update_by_id(pid: i32, obj: &PatchUser) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_update_by_id::<Self::Item, UserService, PatchUser>(pid, obj)
    }
    fn filter(
        param: &RequestParam<PaginationParam, UserFilter>,
    ) -> Result<ApiResponse<Data<Vec<Self::Item>>>, Box<dyn std::error::Error>> {
        controller_filter::<Self::Item, UserService, UserFilter>(param)
    }
}
