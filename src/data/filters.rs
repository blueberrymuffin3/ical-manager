use anyhow::Context;
use serde::{Deserialize, Serialize};
use sqlx::{
    sqlite::{SqliteTypeInfo, SqliteValueRef},
    Decode, Encode, Sqlite, Type,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Filter {
    RemoveCarraigeReturn,
}

#[derive(Debug, Clone, Default)]
pub struct Filters(pub Vec<Filter>);
impl<'a> Encode<'a, Sqlite> for Filters {
    fn encode_by_ref(
        &self,
        buf: &mut <Sqlite as sqlx::database::HasArguments<'a>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        // There is no way to return an erro here, so we panic
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
