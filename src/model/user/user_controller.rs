use std::fs::read_to_string;

use rocket::{
    get,
    response::{content::RawHtml, Redirect},
    uri,
};

use super::user_guard::{AdminUserGuard, UserGuard};

#[get("/<user_id>")]
pub fn get_user_by_id(user_id: &str) -> &str {
    user_id
}

#[get("/login")]
pub fn login() -> RawHtml<String> {
    RawHtml(read_to_string("public/pages/login/index.html").expect("Failed to read login page"))
}

#[get("/dashboard")]
pub fn admin_dashboard(user: Option<AdminUserGuard>) -> Result<&'static str, Redirect> {
    let user = user.ok_or_else(|| Redirect::to(uri!("/user/login")))?;
    Ok(user.name)
}

#[get("/dashboard", rank = 2)]
pub fn user_dashboard(user: Option<UserGuard>) -> Result<&'static str, Redirect> {
    let user = user.ok_or_else(|| Redirect::to(uri!("/user/login")))?;
    Ok(user.name)
}
