use crab_rocket_schema::DbPool;
use obj_traits::response::data::Data;
use rocket::State;

use crate::models::follow::{Follow, PostFollow};

pub trait FollowServiceTrait<P> {
    fn delete_follow_specifically(
        pool: &State<DbPool>,
        obj: &PostFollow,
    ) -> Result<Follow, Box<dyn std::error::Error>>;
    fn get_followings_by_user_id(
        pool: &State<DbPool>,
        uid: i32,
        param: &P,
    ) -> Result<Data<Vec<Follow>>, Box<dyn std::error::Error>>;
    fn get_followeds_by_user_id(
        pool: &State<DbPool>,
        uid: i32,
        param: &P,
    ) -> Result<Data<Vec<Follow>>, Box<dyn std::error::Error>>;
}
