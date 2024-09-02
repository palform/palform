use anyhow::anyhow;
use palform_tsid_primitive::{create_tsid, TSID};
use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
    ops::Deref,
};

use crate::resources::{IDUnknown, PalformIDResource};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(type = "string", concrete(Resource = IDUnknown)))]
pub struct PalformDatabaseID<Resource: PalformIDResource> {
    pub(crate) id: TSID,
    resource: PhantomData<Resource>,
}

impl<Resource: PalformIDResource> PalformDatabaseID<Resource> {
    pub fn from_raw_number(number: u64) -> Self {
        Self {
            id: TSID::from(number),
            resource: PhantomData,
        }
    }

    pub fn from_integer(number: i64) -> Self {
        Self {
            id: TSID::from(number as u64),
            resource: PhantomData,
        }
    }

    pub fn random() -> Self {
        Self {
            id: create_tsid(),
            resource: PhantomData,
        }
    }

    pub(crate) fn from_tsid(tsid: TSID) -> Self {
        Self {
            id: tsid,
            resource: PhantomData,
        }
    }

    pub fn into_unknown(&self) -> PalformDatabaseID<IDUnknown> {
        PalformDatabaseID::<IDUnknown> {
            id: self.id,
            resource: PhantomData,
        }
    }

    pub fn from_str(v: &str) -> Result<PalformDatabaseID<Resource>, anyhow::Error> {
        let tsid_only = if let Some(prefix) = Resource::prefix() {
            v.strip_prefix(&format!("{}_", prefix))
                .ok_or(anyhow!("missing prefix {}_", prefix))?
        } else {
            v
        };

        let tsid = TSID::try_from(tsid_only).map_err(|_| anyhow!("invalid tsid"))?;
        Ok(PalformDatabaseID::<Resource>::from_tsid(tsid))
    }
}

impl<Resource: PalformIDResource> Display for PalformDatabaseID<Resource> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(prefix) = Resource::prefix() {
            write!(f, "{}_{}", prefix, self.id.to_string())
        } else {
            write!(f, "{}", self.id.to_string())
        }
    }
}

impl<Resource: PalformIDResource> Deref for PalformDatabaseID<Resource> {
    type Target = TSID;
    fn deref(&self) -> &Self::Target {
        &self.id
    }
}
