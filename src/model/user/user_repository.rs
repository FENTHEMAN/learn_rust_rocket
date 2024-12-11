use super::user_model::User;

pub struct Database {
    users: Vec<User>,
}

impl Database {
    pub fn get_user_by_name(&self, name: &str) -> Option<User> {
        self.users.iter().find(|user| user.name == name).cloned()
    }
}

pub fn db() -> Database {
    Database {
        users: vec![User {
            name: "s",
            email: "s",
            is_admin: true,
        }],
    }
}
