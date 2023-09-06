use sqlx::{Executor, FromRow, Sqlite};
use uuid::Uuid;

use super::{filters, source};

#[derive(Debug, Clone, FromRow)]
pub struct Feed {
    pub id: i64,
    pub link_code: String,
    #[sqlx(flatten)]
    pub data: FeedData,
}

#[derive(Debug, Clone, FromRow)]
pub struct FeedData {
    pub name: String,
    #[sqlx(flatten)]
    pub source: source::Source,
    pub filters: filters::Filters,
}

impl FeedData {
    pub async fn create<'a>(
        self,
        executor: impl Executor<'a, Database = Sqlite>,
    ) -> sqlx::Result<Feed> {
        let link_code = Uuid::new_v4().simple().to_string();
        let id = sqlx::query_scalar!(
            "INSERT INTO Feed(link_code, name, filters) VALUES (?, ?, ?) RETURNING id",
            link_code,
            self.name,
            self.filters
        )
        .fetch_one(executor)
        .await?;

        Ok(Feed {
            id,
            link_code,
            data: self,
        })
    }
}

impl Feed {
    pub async fn select<'a>(
        executor: impl Executor<'a, Database = Sqlite>,
    ) -> sqlx::Result<Vec<Feed>> {
        sqlx::query_as("SELECT * FROM Feed")
            .fetch_all(executor)
            .await
    }

    pub async fn select_by_id<'a>(
        id: i64,
        executor: impl Executor<'a, Database = Sqlite>,
    ) -> sqlx::Result<Option<Feed>> {
        sqlx::query_as("SELECT * FROM Feed WHERE id = ?")
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn select_by_link_code<'a>(
        link_code: &str,
        executor: impl Executor<'a, Database = Sqlite>,
    ) -> sqlx::Result<Option<Feed>> {
        sqlx::query_as("SELECT * FROM Feed WHERE link_code = ?")
            .bind(link_code)
            .fetch_optional(executor)
            .await
    }

    pub async fn update<'a>(
        &self,
        executor: impl Executor<'a, Database = Sqlite>,
    ) -> sqlx::Result<()> {
        sqlx::query!(
            "UPDATE Feed SET link_code = ?, name = ?, filters = ? WHERE id = ? RETURNING id",
            self.link_code,
            self.data.name,
            self.data.filters,
            self.id
        )
        .fetch_one(executor)
        .await?;
        Ok(())
    }

    pub async fn delete_by_id<'a>(
        id: i64,
        executor: impl Executor<'a, Database = Sqlite>,
    ) -> sqlx::Result<bool> {
        Ok(sqlx::query_scalar!("DELETE FROM Feed WHERE id = ?", id)
            .fetch_optional(executor)
            .await?
            .is_some())
    }
}
