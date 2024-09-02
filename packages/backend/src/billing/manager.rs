use std::{collections::HashMap, str::FromStr};

use chrono::{DateTime, Datelike, TimeZone, Utc};
use palform_entities::{organisation, prelude::*};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use rocket_okapi::okapi::schemars::{self, JsonSchema};
use sea_orm::{ActiveModelTrait, ConnectionTrait, DbErr, EntityTrait, QuerySelect, Set};
use serde::Deserialize;
use stripe::{
    BillingPortalSession, CancelSubscription, CancellationDetails, CancellationDetailsFeedback,
    CheckoutSession, CheckoutSessionBillingAddressCollection, CheckoutSessionMode,
    CreateCheckoutSession, CreateCheckoutSessionAutomaticTax, CreateCheckoutSessionCustomerUpdate,
    CreateCheckoutSessionCustomerUpdateAddress, CreateCheckoutSessionCustomerUpdateName,
    CreateCheckoutSessionLineItems, CreateCheckoutSessionSubscriptionData,
    CreateCheckoutSessionTaxIdCollection, CreateCustomer, Currency, Customer, CustomerId,
    IdOrCreate, Invoice, List, ListInvoices, ListPrices, ListSubscriptions, Object, Price, PriceId,
    Product, RecurringInterval, Scheduled, Subscription, SubscriptionId, SubscriptionItemId,
    SubscriptionStatus, SubscriptionStatusFilter, UpdateSubscription,
    UpdateSubscriptionCancellationDetails, UpdateSubscriptionCancellationDetailsFeedback,
    UpdateSubscriptionItems,
};

use crate::api_entities::billing::{
    customer::{APIBillingCustomer, APIBillingCustomerPaymentMethod},
    invoice::{APIBillingInvoice, APIBillingInvoiceStatus, APIBillingUpcomingInvoice},
    plan::{APIBillingPlan, APIBillingSubscription, APIBillingSubscriptionFrequency},
};

use super::{
    error::BillingError,
    overrides::invoices::{
        CreatePreviewInvoiceAutomaticTax, CreatePreviewInvoiceRequest,
        CreatePreviewInvoiceSubscriptionDetails, InvoiceOverride, UpcomingInvoiceRequest,
    },
    util::stripe_timestamp_to_chrono,
};

#[derive(Deserialize, JsonSchema, Clone)]
pub enum CancelPlanRequestReason {
    CustomerService,
    LowQuality,
    MissingFeatures,
    Other,
    SwitchedService,
    TooComplex,
    TooExpensive,
    Unused,
}

// i hate rust sometimes

impl From<CancelPlanRequestReason> for CancellationDetailsFeedback {
    fn from(value: CancelPlanRequestReason) -> Self {
        match value {
            CancelPlanRequestReason::CustomerService => {
                CancellationDetailsFeedback::CustomerService
            }
            CancelPlanRequestReason::LowQuality => CancellationDetailsFeedback::LowQuality,
            CancelPlanRequestReason::MissingFeatures => {
                CancellationDetailsFeedback::MissingFeatures
            }
            CancelPlanRequestReason::Other => CancellationDetailsFeedback::Other,
            CancelPlanRequestReason::SwitchedService => {
                CancellationDetailsFeedback::SwitchedService
            }
            CancelPlanRequestReason::TooComplex => CancellationDetailsFeedback::TooComplex,
            CancelPlanRequestReason::TooExpensive => CancellationDetailsFeedback::TooExpensive,
            CancelPlanRequestReason::Unused => CancellationDetailsFeedback::Unused,
        }
    }
}

impl From<CancelPlanRequestReason> for UpdateSubscriptionCancellationDetailsFeedback {
    fn from(value: CancelPlanRequestReason) -> Self {
        match value {
            CancelPlanRequestReason::CustomerService => {
                UpdateSubscriptionCancellationDetailsFeedback::CustomerService
            }
            CancelPlanRequestReason::LowQuality => {
                UpdateSubscriptionCancellationDetailsFeedback::LowQuality
            }
            CancelPlanRequestReason::MissingFeatures => {
                UpdateSubscriptionCancellationDetailsFeedback::MissingFeatures
            }
            CancelPlanRequestReason::Other => UpdateSubscriptionCancellationDetailsFeedback::Other,
            CancelPlanRequestReason::SwitchedService => {
                UpdateSubscriptionCancellationDetailsFeedback::SwitchedService
            }
            CancelPlanRequestReason::TooComplex => {
                UpdateSubscriptionCancellationDetailsFeedback::TooComplex
            }
            CancelPlanRequestReason::TooExpensive => {
                UpdateSubscriptionCancellationDetailsFeedback::TooExpensive
            }
            CancelPlanRequestReason::Unused => {
                UpdateSubscriptionCancellationDetailsFeedback::Unused
            }
        }
    }
}

pub struct BillingManager<'a> {
    stripe: &'a stripe::Client,
}
impl<'a> BillingManager<'a> {
    pub fn new(stripe: &'a stripe::Client) -> Self {
        Self { stripe }
    }

    fn product_is_plan(product: &Product) -> bool {
        product.metadata.clone().is_some_and(|metadata| {
            let availability_val = metadata.get("availability").cloned();
            availability_val.is_some_and(|v| v == "main_plan".to_string())
        })
    }

    fn find_price_with_frequency(
        list: &List<Price>,
        frequency: RecurringInterval,
    ) -> Option<Price> {
        list.data
            .iter()
            .find(|e| e.recurring.clone().is_some_and(|r| r.interval == frequency))
            .cloned()
    }

    pub async fn list_plans(
        &self,
        currency: Currency,
    ) -> Result<Vec<APIBillingPlan>, BillingError> {
        let all_products = Product::list(
            &self.stripe,
            &stripe::ListProducts {
                active: Some(true),
                limit: Some(100),
                ..Default::default()
            },
        )
        .await?;

        let mut plan_list = Vec::<APIBillingPlan>::new();
        for product in all_products.data {
            if !Self::product_is_plan(&product) {
                continue;
            }

            let prices = Price::list(
                &self.stripe,
                &ListPrices {
                    active: Some(true),
                    currency: Some(currency),
                    product: Some(IdOrCreate::Id(product.id.as_str())),
                    ..Default::default()
                },
            )
            .await?;

            let monthly_price = Self::find_price_with_frequency(&prices, RecurringInterval::Month)
                .ok_or(BillingError::FieldNone(
                    "monthly price on product".to_string(),
                ))?;
            let annual_price = Self::find_price_with_frequency(&prices, RecurringInterval::Year)
                .ok_or(BillingError::FieldNone(
                    "annual price on product".to_string(),
                ))?;

            let features = product.features.map_or(Vec::new(), |f| {
                f.iter()
                    .map(|v| {
                        v.name
                            .clone()
                            .ok_or(BillingError::FieldNone("feature.name".to_string()))
                    })
                    .collect()
            });

            let features: Result<Vec<String>, BillingError> = features.into_iter().collect();
            let features = features?;

            let is_highlight = product
                .metadata
                .is_some_and(|m| m.get("highlight").is_some_and(|v| v == "highlight"));

            plan_list.push(APIBillingPlan {
                name: product
                    .name
                    .ok_or(BillingError::FieldNone("name".to_string()))?,
                stripe_product_id: product.id.to_string(),
                price_monthly: monthly_price.try_into()?,
                price_annually: annual_price.try_into()?,
                currency: currency.to_string(),
                features,
                highlight: is_highlight,
            })
        }

        Ok(plan_list)
    }

    async fn get_org_customer_id<T: ConnectionTrait>(
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
    ) -> Result<CustomerId, BillingError> {
        let customer_id: Option<Option<String>> = Organisation::find_by_id(org_id)
            .select_only()
            .column(organisation::Column::BillingCustomerId)
            .into_tuple()
            .one(conn)
            .await?;
        let customer_id = customer_id
            .ok_or(BillingError::DBError(DbErr::RecordNotFound(
                "Organisation".to_string(),
            )))?
            .ok_or(BillingError::NotSetUp("organisation customer".to_string()))?;
        let customer_id = CustomerId::from_str(&customer_id)?;
        Ok(customer_id)
    }

    pub async fn get_org_customer<T: ConnectionTrait>(
        &self,
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
    ) -> Result<APIBillingCustomer, BillingError> {
        let customer_id = Self::get_org_customer_id(conn, org_id).await?;
        let customer = Customer::retrieve(self.stripe, &customer_id, &[]).await?;

        if customer.deleted {
            return Err(BillingError::NotSetUp("organisation customer".to_string()))?;
        }

        let customer = APIBillingCustomer {
            entity_name: customer.name,
            email: customer.email,
            address: customer.address.map(|a| a.into()),
            default_payment_method: customer
                .invoice_settings
                .and_then(|v| v.default_payment_method)
                .and_then(|v| v.as_object().cloned())
                .and_then(|v| Some(APIBillingCustomerPaymentMethod::from(v.to_owned()))),
        };

        Ok(customer)
    }

    pub async fn register_org_customer_stub<T: ConnectionTrait>(
        &self,
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
    ) -> Result<(), BillingError> {
        let new_customer = Customer::create(
            &self.stripe,
            CreateCustomer {
                metadata: Some(HashMap::from([(
                    "organisation_id".to_string(),
                    org_id.to_string(),
                )])),
                ..Default::default()
            },
        )
        .await?;

        let updated_org = organisation::ActiveModel {
            id: Set(org_id),
            billing_customer_id: Set(Some(new_customer.id.to_string())),
            ..Default::default()
        };
        updated_org.update(conn).await?;
        Ok(())
    }

    pub async fn get_org_plans<T: ConnectionTrait>(
        &self,
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
    ) -> Result<Vec<APIBillingSubscription>, BillingError> {
        let customer_id = Self::get_org_customer_id(conn, org_id).await?;
        let subscriptions = Subscription::list(
            &self.stripe,
            &ListSubscriptions {
                customer: Some(customer_id),
                ..Default::default()
            },
        )
        .await?;

        let mut subscription_vec = Vec::<APIBillingSubscription>::new();
        for subscription in subscriptions.data {
            if subscription.status != SubscriptionStatus::Active
                && subscription.status != SubscriptionStatus::Trialing
            {
                continue;
            }

            for item in subscription.items.data {
                let item_price = item
                    .price
                    .ok_or(BillingError::FieldNone("item.price".to_string()))?;

                let product_id = item_price
                    .clone()
                    .product
                    .ok_or(BillingError::FieldNone("item.price.product".to_string()))?
                    .id();

                let frequency = item_price
                    .clone()
                    .recurring
                    .and_then(|f| Some(f.interval))
                    .ok_or(BillingError::FieldNone("item.price.recurring".to_string()))?;
                let frequency = APIBillingSubscriptionFrequency::try_from(frequency)?;

                let currency = item_price
                    .currency
                    .ok_or(BillingError::FieldNone("item.price.currency".to_string()))?
                    .to_string();

                let product = Product::retrieve(&self.stripe, &product_id, &[]).await?;
                if !Self::product_is_plan(&product) {
                    continue;
                }

                subscription_vec.push(APIBillingSubscription {
                    stripe_subscription_id: subscription.id.to_string(),
                    stripe_plan_product_id: product.id.to_string(),
                    stripe_plan_price_id: item_price.id().to_string(),
                    plan_name: product.name.ok_or(BillingError::FieldNone(
                        "item.price.product.name".to_string(),
                    ))?,
                    plan_frequency: frequency,
                    is_trial: subscription.status == SubscriptionStatus::Trialing,
                    price: item_price.try_into()?,
                    period_end: DateTime::<Utc>::from_timestamp(subscription.current_period_end, 0)
                        .ok_or(BillingError::FieldNone(
                            "subscription.period_end".to_string(),
                        ))?,
                    canceling_at_end: subscription.cancel_at_period_end,
                    currency,
                })
            }
        }

        Ok(subscription_vec)
    }

    pub async fn preview_invoice<T: ConnectionTrait>(
        &self,
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
        subscription_id: String,
    ) -> Result<APIBillingUpcomingInvoice, BillingError> {
        let customer_id = Self::get_org_customer_id(conn, org_id).await?;
        let subscription_id = SubscriptionId::from_str(&subscription_id)?;
        let response = InvoiceOverride::upcoming(
            &self.stripe,
            UpcomingInvoiceRequest {
                customer: customer_id,
                subscription: subscription_id,
                expand: Some(vec![
                    "lines.data.price.product".to_string(),
                    "discounts".to_string(),
                ]),
            },
        )
        .await?;

        response.try_into()
    }

    pub async fn create_checkout_session<T: ConnectionTrait>(
        &self,
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
        price_id: String,
        trial: bool,
        success_url: String,
    ) -> Result<String, BillingError> {
        let price_id = PriceId::from_str(&price_id)?;

        let mut line_items = Vec::<CreateCheckoutSessionLineItems>::new();
        line_items.push(CreateCheckoutSessionLineItems {
            price: Some(price_id.to_string()),
            quantity: Some(1),
            ..Default::default()
        });

        let customer_id = Self::get_org_customer_id(conn, org_id).await?;
        let new_session = CheckoutSession::create(
            &self.stripe,
            CreateCheckoutSession {
                customer: Some(customer_id),
                line_items: Some(line_items),
                mode: Some(CheckoutSessionMode::Subscription),
                success_url: Some(success_url.as_str()),
                cancel_url: Some(success_url.as_str()),
                billing_address_collection: Some(CheckoutSessionBillingAddressCollection::Required),
                tax_id_collection: Some(CreateCheckoutSessionTaxIdCollection { enabled: true }),
                customer_update: Some(CreateCheckoutSessionCustomerUpdate {
                    address: Some(CreateCheckoutSessionCustomerUpdateAddress::Auto),
                    name: Some(CreateCheckoutSessionCustomerUpdateName::Auto),
                    ..Default::default()
                }),
                automatic_tax: Some(CreateCheckoutSessionAutomaticTax {
                    enabled: true,
                    ..Default::default()
                }),
                subscription_data: Some(CreateCheckoutSessionSubscriptionData {
                    trial_period_days: if trial { Some(14) } else { None },
                    ..Default::default()
                }),
                allow_promotion_codes: Some(true),
                ..Default::default()
            },
        )
        .await?;

        Ok(new_session
            .url
            .ok_or(BillingError::FieldNone("url".to_string()))?)
    }

    pub async fn list_org_invoices<T: ConnectionTrait>(
        &self,
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
    ) -> Result<Vec<APIBillingInvoice>, BillingError> {
        let customer_id = Self::get_org_customer_id(conn, org_id).await?;
        let resp = Invoice::list(
            &self.stripe,
            &ListInvoices {
                customer: Some(customer_id.clone()),
                limit: Some(100),
                ..Default::default()
            },
        )
        .await?;

        let invoices: Result<Vec<APIBillingInvoice>, BillingError> = resp
            .data
            .iter()
            .map(|i| {
                let i = i.to_owned();
                Ok(APIBillingInvoice {
                    id: i.id.to_string(),
                    amount: i.amount_due.unwrap_or(0),
                    currency: i
                        .currency
                        .ok_or(BillingError::FieldNone("invoice.currency".to_string()))?
                        .to_string(),
                    status: APIBillingInvoiceStatus::from(
                        i.status
                            .ok_or(BillingError::FieldNone("invoice.status".to_string()))?,
                    ),
                    url: i.hosted_invoice_url,
                    created: stripe_timestamp_to_chrono(i.created, "invoice.created")?,
                })
            })
            .collect();

        let invoices = invoices?;
        Ok(invoices)
    }

    pub async fn cancel_subscription<T: ConnectionTrait>(
        &self,
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
        subscription_id: String,
        reason: CancelPlanRequestReason,
    ) -> Result<(), BillingError> {
        let customer_id = Self::get_org_customer_id(conn, org_id).await?;
        let subscription_id = SubscriptionId::from_str(&subscription_id)?;
        let subscription = Subscription::retrieve(&self.stripe, &subscription_id, &[]).await?;

        if subscription.customer.id() != customer_id {
            return Err(BillingError::NotAllowed);
        }

        if subscription.status == SubscriptionStatus::Canceled {
            return Err(BillingError::BadState(
                "cancel already cancelled subscription".to_string(),
            ));
        }

        if subscription.status == SubscriptionStatus::Trialing {
            Subscription::cancel(
                &self.stripe,
                &subscription_id,
                CancelSubscription {
                    cancellation_details: Some(CancellationDetails {
                        feedback: Some(reason.into()),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            )
            .await?;
        } else {
            Subscription::update(
                &self.stripe,
                &subscription_id,
                UpdateSubscription {
                    cancel_at_period_end: Some(true),
                    cancellation_details: Some(UpdateSubscriptionCancellationDetails {
                        feedback: Some(reason.into()),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            )
            .await?;
        }

        Ok(())
    }

    pub async fn get_org_billing_month_start<T: ConnectionTrait>(
        &self,
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
    ) -> Result<(DateTime<Utc>, bool), BillingError> {
        let customer_id = Self::get_org_customer_id(conn, org_id).await?;

        let subscriptions = Subscription::list(
            &self.stripe,
            &ListSubscriptions {
                customer: Some(customer_id),
                status: Some(SubscriptionStatusFilter::Active),
                ..Default::default()
            },
        )
        .await?;

        let date_now = Utc::now();
        for subscription in subscriptions.data {
            let mut is_main_plan_subscription = false;
            let mut interval = RecurringInterval::Month;

            for item in subscription.items.data {
                let price = item
                    .price
                    .ok_or(BillingError::FieldNone("item.price".to_string()))?;

                let product_id = price
                    .product
                    .ok_or(BillingError::FieldNone("item.price.product".to_string()))?
                    .id();

                let product = Product::retrieve(&self.stripe, &product_id, &[]).await?;
                if Self::product_is_plan(&product) {
                    is_main_plan_subscription = true;

                    interval = price
                        .recurring
                        .ok_or(BillingError::FieldNone("item.price.recurring".to_string()))?
                        .interval;

                    break;
                }
            }

            if !is_main_plan_subscription {
                continue;
            }

            return Ok((
                DateTime::<Utc>::from_timestamp(subscription.current_period_start, 0).ok_or(
                    BillingError::FieldNone("subscription.current_period_start".to_string()),
                )?,
                interval == RecurringInterval::Year,
            ));
        }

        let natural_start_of_month = Utc
            .with_ymd_and_hms(date_now.year(), date_now.month(), 1, 0, 0, 0)
            .unwrap();
        Ok((natural_start_of_month, false))
    }

    async fn calculate_plan_update_items<T: ConnectionTrait>(
        &self,
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
        new_price_id: String,
    ) -> Result<(Vec<UpdateSubscriptionItems>, SubscriptionId), BillingError> {
        let new_price_id = PriceId::from_str(&new_price_id)?;
        let customer_id = Self::get_org_customer_id(conn, org_id).await?;

        let subscriptions = Subscription::list(
            &self.stripe,
            &ListSubscriptions {
                customer: Some(customer_id),
                ..Default::default()
            },
        )
        .await?;

        let subscription = subscriptions.data.get(0).ok_or(BillingError::BadState(
            "Organisation does not have a subscription, so it cannot be changed".to_string(),
        ))?;

        let mut current_subscription_item_id = None::<SubscriptionItemId>;

        for item in &subscription.items.data {
            let price = item.price.clone().ok_or(BillingError::FieldNone(
                "subscription.items.price".to_string(),
            ))?;

            let product_id = price
                .product
                .ok_or(BillingError::FieldNone(
                    "subscription.items.price.product".to_string(),
                ))?
                .id();

            let product = Product::retrieve(&self.stripe, &product_id, &[]).await?;
            if Self::product_is_plan(&product) {
                current_subscription_item_id = Some(item.id.clone());
            }
        }
        let current_subscription_item_id = current_subscription_item_id.ok_or(
            BillingError::BadState("Cannot change plan as there currently is no plan".to_string()),
        )?;

        Ok((
            vec![UpdateSubscriptionItems {
                id: Some(current_subscription_item_id.to_string()),
                price: Some(new_price_id.to_string()),
                ..Default::default()
            }],
            subscription.id.clone(),
        ))
    }

    pub async fn preview_plan_change<T: ConnectionTrait>(
        &self,
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
        new_price_id: String,
    ) -> Result<APIBillingUpcomingInvoice, BillingError> {
        let (items, subscription_id) = self
            .calculate_plan_update_items(conn, org_id, new_price_id)
            .await?;

        let req = CreatePreviewInvoiceRequest {
            subscription: subscription_id,
            automatic_tax: CreatePreviewInvoiceAutomaticTax { enabled: true },
            subscription_details: CreatePreviewInvoiceSubscriptionDetails {
                cancel_at_period_end: Some(false),
                trial_end: Some(Scheduled::now()),
                items,
            },
            expand: Some(vec!["lines.data.price.product".to_string()]),
        };
        let invoice = InvoiceOverride::create_preview(&self.stripe, req).await?;

        Invoice::from(invoice).try_into()
    }

    pub async fn change_subscription_plan<T: ConnectionTrait>(
        &self,
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
        new_price_id: String,
    ) -> Result<(), BillingError> {
        let (items, subscription_id) = self
            .calculate_plan_update_items(conn, org_id, new_price_id)
            .await?;

        Subscription::update(
            &self.stripe,
            &subscription_id,
            UpdateSubscription {
                cancel_at_period_end: Some(false),
                trial_end: Some(Scheduled::now()),
                items: Some(items),
                ..Default::default()
            },
        )
        .await?;

        Ok(())
    }

    pub async fn update_payment_method_link<T: ConnectionTrait>(
        &self,
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
        return_url: String,
    ) -> Result<String, BillingError> {
        let customer_id = Self::get_org_customer_id(conn, org_id).await?;
        let session = BillingPortalSession::create(
            &self.stripe,
            stripe::CreateBillingPortalSession {
                customer: customer_id,
                flow_data: Some(stripe::CreateBillingPortalSessionFlowData {
                    type_: stripe::CreateBillingPortalSessionFlowDataType::PaymentMethodUpdate,
                    ..Default::default()
                }),
                configuration: None,
                expand: &[],
                locale: None,
                on_behalf_of: None,
                return_url: Some(&return_url),
            },
        )
        .await?;

        Ok(session.url)
    }
}
