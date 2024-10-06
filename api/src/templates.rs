use askama_axum::Template;

#[derive(Template)]
#[template(path = "pages/index.html")]
pub struct IndexTemplate {}

/// Renders the index page
pub async fn index_page() -> IndexTemplate {
    IndexTemplate {}
}

#[derive(Template)]
#[template(path = "pages/upload.html")]
pub struct UploadTemplate {
    pub upload_error: Option<String>,
    pub upload_id: Option<String>,
}

impl UploadTemplate {
    pub fn new() -> Self {
        UploadTemplate {
            upload_error: None,
            upload_id: None,
        }
    }

    pub fn upload_error(mut self, upload_error: &str) -> Self {
        self.upload_error = Some(upload_error.to_string());
        self
    }

    pub fn upload_id(mut self, upload_id: &str) -> Self {
        self.upload_id = Some(upload_id.to_string());
        self
    }
}
