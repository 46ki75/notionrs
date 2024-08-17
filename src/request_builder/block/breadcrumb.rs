use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct BreadcrumbBlockRequest {
    pub r#type: String,

    pub breadcrumb: std::collections::HashMap<(), ()>,
}

impl BreadcrumbBlockRequest {
    pub fn new() -> Self {
        BreadcrumbBlockRequest {
            r#type: "breadcrumb".to_string(),
            breadcrumb: std::collections::HashMap::default(),
        }
    }
}
