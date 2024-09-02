use std::borrow::Cow;

use schemars::{
    schema::{InstanceType, Schema, SchemaObject, SingleOrVec, StringValidation},
    JsonSchema,
};

use crate::{resources::PalformIDResource, tsid::PalformDatabaseID};

impl<Resource: PalformIDResource> JsonSchema for PalformDatabaseID<Resource> {
    fn schema_id() -> std::borrow::Cow<'static, str> {
        Cow::Owned(format!(
            "{}::PalformID<{}>",
            module_path!(),
            Resource::prefix().unwrap_or("unknown".to_owned())
        ))
    }

    fn schema_name() -> String {
        format!(
            "PalformID_{}",
            Resource::prefix().unwrap_or("unknown".to_owned())
        )
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> Schema {
        Schema::Object(SchemaObject {
            instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::String))),
            string: Resource::prefix().map(|p| {
                Box::new(StringValidation {
                    pattern: Some(format!("^({}_)(\\w|\\d)+$", p)),
                    ..Default::default()
                })
            }),
            ..Default::default()
        })
    }
}
