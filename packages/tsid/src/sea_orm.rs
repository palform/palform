use sea_orm::{
    sea_query::{ArrayType, Nullable, ValueType, ValueTypeErr},
    ColIdx, ColumnType, DbErr, QueryResult, TryFromU64, TryGetError, TryGetable, Value,
};

use crate::{resources::PalformIDResource, tsid::PalformDatabaseID};

impl<Resource: PalformIDResource> TryGetable for PalformDatabaseID<Resource> {
    fn try_get_by<I: ColIdx>(res: &QueryResult, index: I) -> Result<Self, TryGetError> {
        <i64 as TryGetable>::try_get_by(res, index).map(|v| PalformDatabaseID::from_integer(v))
    }
}

impl<Resource: PalformIDResource> ValueType for PalformDatabaseID<Resource> {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        <i64 as ValueType>::try_from(v).map(|v| PalformDatabaseID::from_integer(v))
    }

    fn type_name() -> String {
        stringify!(PalformDatabaseID).to_owned()
    }

    fn array_type() -> ArrayType {
        ArrayType::BigUnsigned
    }

    fn column_type() -> ColumnType {
        ColumnType::BigUnsigned
    }
}

impl<Resource: PalformIDResource> From<PalformDatabaseID<Resource>> for Value {
    fn from(value: PalformDatabaseID<Resource>) -> Self {
        Value::BigUnsigned(Some(value.number()))
    }
}


impl<Resource: PalformIDResource> TryFromU64 for PalformDatabaseID<Resource> {
    fn try_from_u64(n: u64) -> Result<Self, DbErr> {
        Ok(Self::from_raw_number(n))
    }
}

impl<Resource: PalformIDResource> Nullable for PalformDatabaseID<Resource> {
    fn null() -> Value {
        Value::BigUnsigned(None)
    }
}
