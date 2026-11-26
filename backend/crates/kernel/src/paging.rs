use utoipa::ToSchema;
use serde::{Deserialize, Serialize};

use crate::error::codes::{PAGE_INVALID, PAGE_SIZE_INVALID};
use crate::error::{Error, Result};

pub const DEFAULT_PAGE_SIZE: u32 = 20;
pub const MAX_PAGE_SIZE: u32 = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct PageRequest {
    pub page: u32,
    pub page_size: u32,
}

impl Default for PageRequest {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: DEFAULT_PAGE_SIZE,
        }
    }
}

impl PageRequest {
    pub fn validated(self) -> Result<Self> {
        if self.page < 1 {
            return Err(Error::bad_request(PAGE_INVALID));
        }
        if self.page_size < 1 || self.page_size > MAX_PAGE_SIZE {
            return Err(Error::bad_request(PAGE_SIZE_INVALID)
                .with_params(serde_json::json!({ "max": MAX_PAGE_SIZE })));
        }
        Ok(self)
    }

    pub fn limit(&self) -> i64 {
        i64::from(self.page_size)
    }

    pub fn offset(&self) -> i64 {
        i64::from(self.page - 1) * i64::from(self.page_size)
    }
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Page<T: Serialize> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: u32,
    pub page_size: u32,
}

impl<T: Serialize> Page<T> {
    pub fn new(items: Vec<T>, total: i64, request: PageRequest) -> Self {
        Self {
            items,
            total,
            page: request.page,
            page_size: request.page_size,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offset_follows_one_based_page() {
        let request = PageRequest { page: 3, page_size: 25 }.validated().unwrap();
        assert_eq!(request.offset(), 50);
        assert_eq!(request.limit(), 25);
    }

    #[test]
    fn rejects_zero_page_and_oversized_pages() {
        assert!(PageRequest { page: 0, page_size: 10 }.validated().is_err());
        assert!(PageRequest { page: 1, page_size: 0 }.validated().is_err());
        assert!(PageRequest { page: 1, page_size: MAX_PAGE_SIZE + 1 }.validated().is_err());
    }

    #[test]
    fn default_request_is_first_page() {
        let request = PageRequest::default().validated().unwrap();
        assert_eq!(request.offset(), 0);
        assert_eq!(request.page_size, DEFAULT_PAGE_SIZE);
    }
}
