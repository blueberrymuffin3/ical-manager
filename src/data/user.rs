use sqlx::{Executor, FromRow, Sqlite};

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: i64,
    #[sqlx(flatten)]
    pub data: UserData,
}

#[derive(Debug, Clone, FromRow)]
pub struct UserData {
    pub name: Option<String>,
    pub icon: Option<String>,
}

impl User {
    pub async fn create(executor: impl Executor<'_, Database = Sqlite>) -> sqlx::Result<User> {
        sqlx::query_as("INSERT INTO User DEFAULT VALUES RETURNING id, name, icon")
            .fetch_one(executor)
            .await
    }

    pub async fn select_by_id(
        id: i64,
        executor: impl Executor<'_, Database = Sqlite>,
    ) -> sqlx::Result<Option<User>> {
        sqlx::query_as("SELECT id, name, icon FROM User WHERE id = ?")
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn update(&self, executor: impl Executor<'_, Database = Sqlite>) -> sqlx::Result<()> {
        sqlx::query_scalar!(
            "UPDATE User SET name = ?, icon = ? WHERE id = ? RETURNING id",
            self.data.name,
            self.data.icon,
            self.id
        )
        .fetch_one(executor)
        .await?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct UserOAuthLink {
    pub id: i64,
    pub issuer: String,
    pub subject: String,
}

impl UserOAuthLink {
    pub async fn select_by_issuer_subject(
        issuer: &str,
        subject: &str,
        executor: impl Executor<'_, Database = Sqlite>,
    ) -> sqlx::Result<Option<UserOAuthLink>> {
        sqlx::query_as!(
            UserOAuthLink,
            "SELECT id, issuer, subject FROM UserOAuthLink WHERE issuer = ? AND subject = ?",
            issuer,
            subject
        )
        .fetch_optional(executor)
        .await
    }

    pub async fn create(&self, executor: impl Executor<'_, Database = Sqlite>) -> sqlx::Result<()> {
        sqlx::query!(
            "INSERT INTO UserOAuthLink(id, issuer, subject) VALUES (?, ?, ?)",
            self.id,
            self.issuer,
            self.subject
        )
        .fetch_all(executor)
        .await?;
        Ok(())
    }
}
