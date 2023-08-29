use sqlx::FromRow;

use super::{filters, url_source};

#[derive(Debug, Clone, FromRow)]
struct Feed {
    pub id: i64,
    pub name: String,
    pub link_code: String,
    pub filters: filters::Filters,
    #[sqlx(flatten)]
    pub url_source: Option<url_source::UrlSource>,
}

impl Feed {}
