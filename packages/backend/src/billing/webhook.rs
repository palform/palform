use palform_entities::{organisation, prelude::*};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QuerySelect, Set,
};
use stripe::{EventType, Product, Subscription, SubscriptionStatus};

use crate::api_entities::billing::entitlement::APIEntitlementInfo;

use super::error::BillingError;

pub struct BillingWebhookManager<'a, T: ConnectionTrait> {
    stripe: &'a stripe::Client,
    db: &'a T,
}

impl<'a, T: ConnectionTrait> BillingWebhookManager<'a, T> {
    pub fn new(stripe: &'a stripe::Client, db: &'a T) -> Self {
        Self { stripe, db }
    }

    pub async fn subscription(
        &self,
        event: EventType,
        data: Subscription,
    ) -> Result<(), BillingError> {
        let matching_org: PalformDatabaseID<IDOrganisation> = Organisation::find()
            .select_only()
            .column(organisation::Column::Id)
            .filter(
                organisation::Column::BillingCustomerId.eq(Some(data.customer.id().to_string())),
            )
            .into_tuple()
            .one(self.db)
            .await?
            .ok_or(BillingError::FieldNone(
                "No organisation has this subscription".to_string(),
            ))?;

        if event == EventType::CustomerSubscriptionCreated
            || event == EventType::CustomerSubscriptionUpdated
            || event == EventType::CustomerSubscriptionResumed
                && (data.status == SubscriptionStatus::Active
                    || data.status == SubscriptionStatus::Trialing)
        {
            let items = data.items.data;
            for item in items {
                let product_id = item
                    .price
                    .ok_or(BillingError::FieldNone("item.price".to_string()))?
                    .product
                    .ok_or(BillingError::FieldNone("item.price.product".to_string()))?
                    .id();

                let product = Product::retrieve(self.stripe, &product_id, &[]).await?;
                let metadata = product
                    .metadata
                    .ok_or(BillingError::FieldNone("product.metadata".to_string()))?;

                let is_main_plan = metadata
                    .get("availability")
                    .is_some_and(|v| *v == "main_plan");
                if !is_main_plan {
                    continue;
                }

                let entitlement_info_str = metadata
                    .get("entitlement")
                    .ok_or(BillingError::FieldNone(
                        "main_plan product did not have entitlement".to_string(),
                    ))?
                    .clone();

                let entitlement_info: APIEntitlementInfo =
                    serde_json::from_str(&entitlement_info_str).map_err(|e| {
                        BillingError::FieldNone(format!(
                            "Failed to parse product entitlement: {}",
                            e
                        ))
                    })?;

                let mut entitlement_model = entitlement_info.to_active_model();
                entitlement_model.organisation_id = Set(matching_org);
                entitlement_model.update(self.db).await?;
            }
        } else {
            // reset to the "default"/free entitlement
            let mut updated_entitlement = APIEntitlementInfo::default().to_active_model();
            updated_entitlement.organisation_id = Set(matching_org);
            updated_entitlement.update(self.db).await?;
        }
        Ok(())
    }
}
