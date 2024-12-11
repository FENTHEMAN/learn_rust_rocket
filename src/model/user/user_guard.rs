use std::ops::Deref;

use rocket::{
    http::Status,
    outcome::try_outcome,
    request::{FromRequest, Outcome},
    Request,
};

use super::user_repository::db;

use super::user_model::{User, UserError};

pub struct UserGuard(User);

impl Deref for UserGuard {
    type Target = User;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserGuard {
    type Error = UserError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        fn is_auth(auth_key: &str) -> bool {
            auth_key == "popa_kaka"
        }

        match req.headers().get_one("x-auth-key") {
            None => Outcome::Error((Status::Unauthorized, UserError::Unauthorized)),
            Some(key) if is_auth(key) => match req.headers().get_one("x-user-name") {
                None => Outcome::Error((Status::Unauthorized, UserError::Unauthorized)),
                Some(name) => match db().get_user_by_name(name) {
                    None => Outcome::Error((Status::NotFound, UserError::NotFound)),
                    Some(user) => Outcome::Success(UserGuard(user)),
                },
            },
            Some(_) => Outcome::Error((Status::Unauthorized, UserError::Unauthorized)),
        }
    }
}

pub struct AdminUserGuard(User);

impl Deref for AdminUserGuard {
    type Target = User;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminUserGuard {
    type Error = UserError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user = try_outcome!(req.guard::<UserGuard>().await);

        if user.is_admin {
            Outcome::Success(AdminUserGuard(*user))
        } else {
            Outcome::Forward(Status::Forbidden)
        }
    }
}
