use async_trait::async_trait;
use sqlx::SqlitePool;

pub trait SecretType: Sized {
    fn id() -> &'static str;
    fn gen() -> Self;
    fn into_bytes(&self) -> anyhow::Result<Vec<u8>>;
    fn from_bytes(bytes: &[u8]) -> anyhow::Result<Self>;
}

impl SecretType for cookie::Key {
    fn id() -> &'static str {
        "Cookie Key"
    }

    fn gen() -> Self {
        Self::generate()
    }

    fn into_bytes(&self) -> anyhow::Result<Vec<u8>> {
        Ok(self.master().to_owned())
    }

    fn from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        Ok(bytes.try_into()?)
    }
}

#[async_trait]
pub trait SecretReader: SecretType {
    async fn read_or_gen(pool: &SqlitePool) -> anyhow::Result<Self> {
        let mut txn = pool.begin().await?;

        let id = Self::id();
        let data = sqlx::query_scalar!("SELECT value FROM Secrets WHERE id = ?", id)
            .fetch_optional(&mut txn)
            .await?;

        Ok(match data {
            Some(data) => Self::from_bytes(&data)?,
            None => {
                let generated = Self::gen();
                let data = generated.into_bytes()?;

                sqlx::query_scalar!("INSERT INTO Secrets(id, value) VALUES (?, ?)", id, data)
                    .fetch_all(&mut txn)
                    .await?;

                txn.commit().await?;

                generated
            }
        })
    }
}
#[async_trait]
impl<T: SecretType> SecretReader for T {}
