use kernel::paging::PageRequest;
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Debug, Default, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct PageQuery {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

impl PageQuery {
    pub fn request(&self) -> PageRequest {
        let default = PageRequest::default();
        PageRequest {
            page: self.page.unwrap_or(default.page),
            page_size: self.page_size.unwrap_or(default.page_size),
        }
    }
}
