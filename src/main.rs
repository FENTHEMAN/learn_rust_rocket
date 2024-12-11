pub mod model;
pub mod utils;

use rocket::{build, fs::FileServer, routes, Error};

use model::user::{admin_dashboard, get_user_by_id, login, user_dashboard};

#[rocket::main]
async fn main() -> Result<(), Error> {
    let _ = build()
        .mount(
            "/user",
            routes![get_user_by_id, admin_dashboard, user_dashboard, login,],
        )
        .mount("/public", FileServer::from("public"))
        .launch()
        .await?;

    Ok(())
}
