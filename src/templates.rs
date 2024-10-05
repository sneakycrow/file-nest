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
pub struct UploadTemplate<'a> {
    pub upload_error: Option<&'a str>,
}

impl<'a> UploadTemplate<'a> {
    pub fn new() -> Self {
        UploadTemplate { upload_error: None }
    }

    pub fn upload_error(mut self, upload_error: &'a str) -> Self {
        self.upload_error = Some(upload_error);
        self
    }
}
