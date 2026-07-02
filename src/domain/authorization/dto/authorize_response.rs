use crate::domain::authorization::entity::PermissionContext;

pub struct AuthorizeResponse {
    pub context: PermissionContext,
}
