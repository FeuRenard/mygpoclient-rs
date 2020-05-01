//! Clients for communication with the service

use reqwest::blocking::{Client, Response};
use reqwest::IntoUrl;
use serde::Serialize;

const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Client without authenticatication
#[derive(Debug, Clone, Default)]
pub struct PublicClient {
    pub(crate) client: Client,
}

/// Client authenticated with username and password
#[derive(Debug, Clone)]
pub struct AuthenticatedClient {
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) public_client: PublicClient,
}

/// Device-specific [`AuthenticatedClient`](./struct.AuthenticatedClient.html)
#[derive(Debug, Clone)]
pub struct DeviceClient {
    pub(crate) device_id: String,
    pub(crate) authenticated_client: AuthenticatedClient,
}

impl PublicClient {
    /// Create [`PublicClient`](./struct.PublicClient.html) locally
    pub fn new() -> PublicClient {
        PublicClient {
            client: Default::default(),
        }
    }

    pub(crate) fn get<U: IntoUrl>(&self, url: U) -> Result<Response, reqwest::Error> {
        let empty_slice: &[&String] = &[];
        self.get_with_query(url, empty_slice)
    }

    pub(crate) fn get_with_query<U: IntoUrl, T: Serialize + ?Sized>(
        &self,
        url: U,
        query_parameters: &[&T],
    ) -> Result<Response, reqwest::Error> {
        self.client
            .get(url)
            .header(
                reqwest::header::USER_AGENT,
                &format!("{}/{}", PACKAGE_NAME, PACKAGE_VERSION),
            )
            .query(query_parameters)
            .send()
    }
}

impl AuthenticatedClient {
    /// Create [`AuthenticatedClient`](./struct.AuthenticatedClient.html) locally
    pub fn new(username: &str, password: &str) -> AuthenticatedClient {
        AuthenticatedClient {
            username: username.to_owned(),
            password: password.to_owned(),
            public_client: PublicClient::new(),
        }
    }

    pub(crate) fn get<U: IntoUrl>(&self, url: U) -> Result<Response, reqwest::Error> {
        let empty_slice: &[&String] = &[];
        self.get_with_query(url, empty_slice)
    }

    pub(crate) fn get_with_query<U: IntoUrl, T: Serialize + ?Sized>(
        &self,
        url: U,
        query_parameters: &[&T],
    ) -> Result<Response, reqwest::Error> {
        self.public_client
            .client
            .get(url)
            .basic_auth(&self.username, Some(&self.password))
            .header(
                reqwest::header::USER_AGENT,
                &format!("{}/{}", PACKAGE_NAME, PACKAGE_VERSION),
            )
            .query(query_parameters)
            .send()
    }

    pub(crate) fn put<T: Serialize + ?Sized, U: IntoUrl>(
        &self,
        url: U,
        json: &T,
    ) -> Result<Response, reqwest::Error> {
        self.public_client
            .client
            .put(url)
            .basic_auth(&self.username, Some(&self.password))
            .header(
                reqwest::header::USER_AGENT,
                &format!("{}/{}", PACKAGE_NAME, PACKAGE_VERSION),
            )
            .json(json)
            .send()
    }

    pub(crate) fn post<T: Serialize + ?Sized, U: IntoUrl>(
        &self,
        url: U,
        json: &T,
    ) -> Result<Response, reqwest::Error> {
        self.public_client
            .client
            .post(url)
            .basic_auth(&self.username, Some(&self.password))
            .header(
                reqwest::header::USER_AGENT,
                &format!("{}/{}", PACKAGE_NAME, PACKAGE_VERSION),
            )
            .json(json)
            .send()
    }
}

impl DeviceClient {
    /// Create [`DeviceClient`](./struct.DeviceClient.html)
    pub fn new(username: &str, password: &str, device_id: &str) -> DeviceClient {
        DeviceClient {
            device_id: device_id.to_owned(),
            authenticated_client: AuthenticatedClient::new(username, password),
        }
    }

    pub(crate) fn get<U: IntoUrl>(&self, url: U) -> Result<Response, reqwest::Error> {
        self.authenticated_client.get(url)
    }

    pub(crate) fn get_with_query<U: IntoUrl, T: Serialize + ?Sized>(
        &self,
        url: U,
        query_parameters: &[&T],
    ) -> Result<Response, reqwest::Error> {
        self.authenticated_client
            .get_with_query(url, query_parameters)
    }

    pub(crate) fn put<T: Serialize + ?Sized, U: IntoUrl>(
        &self,
        url: U,
        json: &T,
    ) -> Result<Response, reqwest::Error> {
        self.authenticated_client.put(url, json)
    }

    pub(crate) fn post<T: Serialize + ?Sized, U: IntoUrl>(
        &self,
        url: U,
        json: &T,
    ) -> Result<Response, reqwest::Error> {
        self.authenticated_client.post(url, json)
    }
}

impl From<DeviceClient> for AuthenticatedClient {
    fn from(device_client: DeviceClient) -> Self {
        device_client.authenticated_client
    }
}

impl AsRef<AuthenticatedClient> for DeviceClient {
    fn as_ref(&self) -> &AuthenticatedClient {
        &self.authenticated_client
    }
}
