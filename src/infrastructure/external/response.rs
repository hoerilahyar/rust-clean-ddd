use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpResponse<T> {
    pub status: u16,
    pub body: T,
}
