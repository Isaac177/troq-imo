use kernel::error::Result;
use kernel::paging::PageRequest;

pub fn paging(page: Option<u32>, page_size: Option<u32>) -> Result<PageRequest> {
    let default = PageRequest::default();
    PageRequest {
        page: page.unwrap_or(default.page),
        page_size: page_size.unwrap_or(default.page_size),
    }
    .validated()
}
