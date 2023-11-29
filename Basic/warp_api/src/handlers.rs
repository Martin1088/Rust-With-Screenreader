use std::u64;

use super::models::Post;

pub async fn get_post(id: u64) -> Result<impl warp::Reply, warp::Rejection> {
    let post = Post {
        id,
        title: String::from("Example Post"),
        body: String::from("here could be the Conntent of this Post "),
    };
    Ok(warp::reply::json(&post))
}
