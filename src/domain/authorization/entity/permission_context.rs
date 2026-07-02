pub struct PermissionContext {
    pub user_id: u64,

    pub username: String,

    pub fullname: String,

    pub roles: Vec<String>,

    pub permissions: Vec<String>,
}
