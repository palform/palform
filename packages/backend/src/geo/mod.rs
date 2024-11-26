use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use bytes::Buf;
use keshvar::Country;
use log::info;
use nodit::{InclusiveInterval, Interval, NoditMap};
use thiserror::Error;

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Copy, Clone)]
struct Ipv4AddrRange {
    start: Ipv4Addr,
    end: Ipv4Addr,
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Copy, Clone)]
struct Ipv6AddrRange {
    start: Ipv6Addr,
    end: Ipv6Addr,
}

impl InclusiveInterval<u32> for Ipv4AddrRange {
    fn start(&self) -> u32 {
        self.start.to_bits()
    }
    fn end(&self) -> u32 {
        self.end.to_bits()
    }
}

impl InclusiveInterval<u128> for Ipv6AddrRange {
    fn start(&self) -> u128 {
        self.start.to_bits()
    }
    fn end(&self) -> u128 {
        self.end.to_bits()
    }
}

impl From<Interval<u32>> for Ipv4AddrRange {
    fn from(value: Interval<u32>) -> Self {
        Self {
            start: Ipv4Addr::from_bits(value.start()),
            end: Ipv4Addr::from_bits(value.end()),
        }
    }
}

impl From<Interval<u128>> for Ipv6AddrRange {
    fn from(value: Interval<u128>) -> Self {
        Self {
            start: Ipv6Addr::from_bits(value.start()),
            end: Ipv6Addr::from_bits(value.end()),
        }
    }
}

pub struct IPGeolocator {
    cache_v4: NoditMap<u32, Ipv4AddrRange, String>,
    cache_v6: NoditMap<u128, Ipv6AddrRange, String>,
}

#[derive(Error, Debug)]
pub enum IPGeolocatorError {
    #[error("Download IP DB: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Parse IP DB: {0}")]
    Csv(#[from] csv::Error),
    #[error("A field was missing")]
    MissingField,
    #[error("Parse IP address: {0}")]
    ParseIP(#[from] std::net::AddrParseError),
    #[error("Key not found in DB")]
    KeyNotFound,
    #[error("Failed to resolve country: {0}")]
    CountryNotFound(#[from] keshvar::SearchError),
    #[error("NoditMap insertion overlap")]
    Nodit,
}

impl IPGeolocator {
    pub async fn new() -> Result<Self, IPGeolocatorError> {
        let mut n = Self {
            cache_v4: NoditMap::new(),
            cache_v6: NoditMap::new(),
        };

        n.load_entries(false).await?;
        n.load_entries(true).await?;

        info!(
            "loaded IP country cache with {} IPv4 + {} IPv6 entries",
            n.cache_v4.len(),
            n.cache_v6.len()
        );
        Ok(n)
    }

    async fn load_entries(&mut self, v6: bool) -> Result<(), IPGeolocatorError> {
        let resp: bytes::Bytes = reqwest::get(format!(
            "https://cdn.jsdelivr.net/npm/@ip-location-db/asn-country/asn-country-{}.csv",
            if v6 { "ipv6" } else { "ipv4" }
        ))
        .await?
        .bytes()
        .await?;

        let mut reader = csv::Reader::from_reader(resp.reader());

        for entry in reader.records() {
            let entry = entry?;

            let ip_range_start = entry.get(0).ok_or(IPGeolocatorError::MissingField)?;
            let ip_range_end = entry.get(1).ok_or(IPGeolocatorError::MissingField)?;
            let country_code = entry.get(2).ok_or(IPGeolocatorError::MissingField)?;

            if v6 {
                let range = Ipv6AddrRange {
                    start: ip_range_start.parse()?,
                    end: ip_range_end.parse()?,
                };

                self.cache_v6
                    .insert_strict(range, country_code.to_owned())
                    .map_err(|_| IPGeolocatorError::Nodit)?;
            } else {
                let range = Ipv4AddrRange {
                    start: ip_range_start.parse()?,
                    end: ip_range_end.parse()?,
                };

                self.cache_v4
                    .insert_strict(range, country_code.to_owned())
                    .map_err(|_| IPGeolocatorError::Nodit)?;
            }
        }

        Ok(())
    }

    pub fn lookup_country(&self, ip: IpAddr) -> Result<Country, IPGeolocatorError> {
        let country_code = match ip {
            IpAddr::V4(v4) => self.cache_v4.get_at_point(v4.to_bits()),
            IpAddr::V6(v6) => self.cache_v6.get_at_point(v6.to_bits()),
        }
        .ok_or(IPGeolocatorError::KeyNotFound)?;

        let alpha2 = keshvar::Alpha2::try_from(country_code.to_owned().as_str())?;
        println!("resolved: {}", alpha2.to_string());

        Ok(alpha2.to_country())
    }
}
