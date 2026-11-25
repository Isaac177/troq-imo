#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Visibility {
    Public,
    Private,
}

impl Visibility {
    pub fn cache_control(self) -> &'static str {
        match self {
            Visibility::Public => "public, max-age=31536000, immutable",
            Visibility::Private => "private, no-store",
        }
    }
}
