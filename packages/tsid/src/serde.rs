use std::marker::PhantomData;

use serde::{
    de::{Unexpected, Visitor},
    Deserialize, Serialize,
};

use crate::{resources::PalformIDResource, tsid::PalformDatabaseID};

impl<Resource: PalformIDResource> Serialize for PalformDatabaseID<Resource> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct PalformDatabaseIDVisitor<Resource: PalformIDResource> {
    resource: PhantomData<Resource>,
}

impl<'de, Resource: PalformIDResource> Visitor<'de> for PalformDatabaseIDVisitor<Resource> {
    type Value = PalformDatabaseID<Resource>;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "string TSID prefixed with {}_",
            Resource::prefix().unwrap_or("nothing".to_owned())
        )
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        PalformDatabaseID::<Resource>::from_str(v)
            .map_err(|e| E::invalid_value(Unexpected::Str(&e.to_string()), &self))
    }
}

impl<'de, Resource: PalformIDResource> Deserialize<'de> for PalformDatabaseID<Resource> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let visitor = PalformDatabaseIDVisitor::<Resource> {
            resource: PhantomData,
        };
        deserializer.deserialize_string(visitor)
    }
}
