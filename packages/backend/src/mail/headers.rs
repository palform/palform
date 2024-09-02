use std::marker::PhantomData;

use lettre::message::header::{Header, HeaderName, HeaderValue};

pub trait MailgunHeaderTypeTrait: Clone {
    fn header_name() -> String;
}

#[derive(Clone)]
pub struct MailgunTemplateNameHeader;
impl MailgunHeaderTypeTrait for MailgunTemplateNameHeader {
    fn header_name() -> String {
        "X-Mailgun-Template-Name".to_string()
    }
}

#[derive(Clone)]
pub struct MailgunVariableListHeader;
impl MailgunHeaderTypeTrait for MailgunVariableListHeader {
    fn header_name() -> String {
        "X-Mailgun-Variables".to_string()
    }
}

#[derive(Clone)]
pub struct MailgunHeader<T: MailgunHeaderTypeTrait> {
    pub value: String,
    pub header_type: PhantomData<T>,
}

impl<T: MailgunHeaderTypeTrait> MailgunHeader<T> {
    pub fn new(value: String) -> Self {
        Self {
            value,
            header_type: PhantomData,
        }
    }
}

impl<T: MailgunHeaderTypeTrait> Header for MailgunHeader<T> {
    fn display(&self) -> HeaderValue {
        HeaderValue::new(Self::name(), self.value.clone())
    }

    fn name() -> HeaderName {
        let n = T::header_name();
        HeaderName::new_from_ascii(n).expect("Parse header name for Mailgun")
    }

    fn parse(_: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        unimplemented!("Do not need to parse Mailgun header");
    }
}
