use crab_rocket_employee::routes::employee_route;
use crab_rocket_file::routes::{bin_file_route, form_file_route};
use crab_rocket_follow::routes::follow_route;
use crab_rocket_info::routes::info_route;
use crab_rocket_permission::routes::permission_route;
use crab_rocket_post::routes::post_route;
use crab_rocket_role::routes::role_route;
use crab_rocket_schema::routes::schema_routes;
use crab_rocket_task::routes::task_route;
use crab_rocket_user::routes::user_route;
use rocket::{routes, Route};

pub fn module_routes() -> Vec<Route> {
    routes![
        bin_file_route::files,
        bin_file_route::retrieve_bin,
        bin_file_route::upload_bin,
        form_file_route::upload,
        form_file_route::upload_avatar,
        form_file_route::download_file,
        form_file_route::retrieve_file,
        form_file_route::get_all_files,
        form_file_route::file_stream,
        form_file_route::options_upload,
        info_route::get_info,
        // task routes
        task_route::index,
        task_route::demo,
        task_route::options_task_filter,
        //
        task_route::get_tasks,
        task_route::get_tasks_by_param,
        task_route::get_task_by_id,
        task_route::insert_single_task,
        task_route::delete_task_by_id,
        task_route::update_task_by_id,
        //user routes
        user_route::options_user,
        //
        user_route::get_users,
        user_route::get_user_by_id,
        user_route::insert_single_user,
        user_route::delete_user_by_id,
        user_route::update_user_by_id,
        // post routes
        post_route::get_posts,
        post_route::get_posts_by_param,
        post_route::get_post_by_id,
        post_route::insert_single_post,
        post_route::update_post_by_id,
        post_route::delete_post_by_id,
        post_route::options_post_filter,
        // follow routes
        follow_route::get_follows,
        follow_route::get_follows_by_params,
        follow_route::insert_single_follow,
        follow_route::insert_single_follow_by_params,
        follow_route::delete_follow_by_id,
        follow_route::update_follow_by_id,
        follow_route::delete_follow_specifically,
        follow_route::get_followeds_by_user_id,
        follow_route::get_followings_by_user_id,
        //employee routes
        employee_route::options_employee,
        //
        employee_route::get_employees,
        employee_route::get_employee_by_id,
        employee_route::get_employees_by_param,
        employee_route::insert_single_employee,
        employee_route::delete_employee_by_id,
        employee_route::update_employee_by_id,
        // role routes
        role_route::get_roles,
        role_route::get_role_by_id,
        role_route::insert_single_role,
        role_route::delete_role_by_id,
        role_route::update_role_by_id,
        // permission routes
        permission_route::get_all_permissions,
        // schema_routes
        schema_routes::get_reload_count
    ]
}