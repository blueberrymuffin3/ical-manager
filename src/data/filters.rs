use std::ops::Deref;

use anyhow::Context;
use serde::{Deserialize, Serialize};
use sqlx::{
    sqlite::{SqliteTypeInfo, SqliteValueRef},
    Decode, Encode, Sqlite, Type,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Filter {
}

#[derive(Debug, Clone, Default)]
pub struct Filters(pub Vec<Filter>);
impl Filters {
    pub(super) async fn select(
        txn: &mut sqlx::Transaction<'_, Sqlite>,
        id: i64,
    ) -> anyhow::Result<Self> {
        let mut filters: Filters = Default::default();

        // let filter_remove_carrige_return =
        //     sqlx::query_scalar!("SELECT id FROM FilterRemoveCarriageReturn WHERE id = ?", id)
        //         .fetch_optional(&mut *txn)
        //         .await?;
        // if let Some(_) = filter_remove_carrige_return {
        //     filters.0.push(Filter::RemoveCarriageReturn);
        // }

        Ok(filters)
    }

    pub(crate) async fn upsert(
        &self,
        txn: &mut sqlx::Transaction<'_, Sqlite>,
        id: i64,
    ) -> anyhow::Result<()> {
        // sqlx::query!("DELETE FROM FilterRemoveCarriageReturn WHERE id = ?", id)
        //     .fetch_all(&mut *txn)
        //     .await?;

        for filter in &self.0 {
            match filter {
                _ => todo!()
                // Filter::RemoveCarriageReturn => {
                //     sqlx::query!("INSERT INTO FilterRemoveCarriageReturn(id) VALUES (?)", id)
                //         .fetch_all(&mut *txn)
                //         .await?;
                // }
            }
        }

        Ok(())
    }
}
impl<'a> Encode<'a, Sqlite> for Filters {
    fn encode_by_ref(
        &self,
        buf: &mut <Sqlite as sqlx::database::HasArguments<'a>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        // There is no way to return an error here, so we panic
        let data = serde_json::to_vec(&self.0).expect("Error serializing filters");
        data.encode(buf)
    }
}

impl<'a> Decode<'a, Sqlite> for Filters {
    fn decode(value: SqliteValueRef) -> Result<Self, sqlx::error::BoxDynError> {
        let data = <&[u8]>::decode(value)?;

        Ok(Filters(
            serde_json::from_slice(data).context("Error deserializing filters")?,
        ))
    }
}

impl Type<Sqlite> for Filters {
    fn type_info() -> SqliteTypeInfo {
        <[u8]>::type_info()
    }
}

impl Deref for Filters {
    type Target = Vec<Filter>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
