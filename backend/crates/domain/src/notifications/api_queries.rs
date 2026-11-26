use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct ListQuery {
    pub locale: Option<String>,
    pub unread_only: Option<bool>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct DocumentsQuery {
    pub kind: Option<String>,
    pub entity_id: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct PreviewQuery {
    pub locale: Option<String>,
}

pub fn paging(page: Option<u32>, page_size: Option<u32>) -> kernel::PageRequest {
    let default = kernel::PageRequest::default();
    kernel::PageRequest { page: page.unwrap_or(default.page), page_size: page_size.unwrap_or(default.page_size) }
}
