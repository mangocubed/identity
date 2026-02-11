use std::{borrow::Cow, net::IpAddr, sync::OnceLock};

use serde::Deserialize;
use serde::de::DeserializeOwned;
use url::Url;

use crate::config::IP_GEO_CONFIG;

#[derive(Deserialize)]
pub struct IpGeoLocation<'a> {
    pub country_code2: Cow<'a, str>,
    pub state_prov: Cow<'a, str>,
    pub city: Cow<'a, str>,
}

#[derive(Deserialize)]
pub struct IpGeoInfo<'a> {
    pub location: IpGeoLocation<'a>,
}

static IP_GEO: OnceLock<IpGeo> = OnceLock::new();

pub struct IpGeo<'a> {
    api_key: Cow<'a, str>,
}

impl Default for IpGeo<'_> {
    fn default() -> Self {
        Self {
            api_key: Cow::Owned(IP_GEO_CONFIG.api_key.clone()),
        }
    }
}

impl<'a> IpGeo<'a> {
    pub fn new() -> &'a Self {
        IP_GEO.get_or_init(IpGeo::default)
    }

    fn request_url(&self, path: &str) -> Url {
        Url::parse(&format!("https://api.ipgeolocation.io/v2/{}", path)).unwrap()
    }

    pub async fn get_with_query<T>(&self, path: &str, query: Option<&str>) -> reqwest::Result<T>
    where
        T: DeserializeOwned,
    {
        let mut request_url = self.request_url(path);

        request_url.set_query(Some(&format!("apiKey={}&{}", self.api_key, query.unwrap_or_default())));

        reqwest::Client::new()
            .get(request_url.clone())
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
    }

    pub async fn info(&self, ip_addr: IpAddr) -> reqwest::Result<IpGeoInfo<'_>> {
        self.get_with_query("ipgeo", Some(&format!("ip={}", ip_addr))).await
    }
}
