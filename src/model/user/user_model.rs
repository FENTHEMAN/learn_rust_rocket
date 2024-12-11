#[derive(Clone, Copy, Debug)]
pub struct User {
    pub name: &'static str,
    pub email: &'static str,
    pub is_admin: bool,
}

#[derive(Debug)]
pub enum UserError {
    Unauthorized,
    NotFound,
    NotAdmin,
}
