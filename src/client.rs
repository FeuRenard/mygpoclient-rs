const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct AuthenticatedClient {
    pub(crate) username: String,
    pub(crate) password: String,
    client: reqwest::Client,
}

pub struct DeviceClient {
    pub(crate) device_id: String,
    pub(crate) authenticated_client: AuthenticatedClient,
}

impl AuthenticatedClient {
    pub fn new(username: &str, password: &str) -> AuthenticatedClient {
        AuthenticatedClient {
            username: username.to_owned(),
            password: password.to_owned(),
            client: reqwest::Client::new(),
        }
    }

    pub(crate) fn get<U: reqwest::IntoUrl>(
        &self,
        url: U,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let empty_slice: &[&String] = &[];
        self.get_with_query(url, empty_slice)
    }

    pub(crate) fn get_with_query<U: reqwest::IntoUrl, T: serde::Serialize + ?Sized>(
        &self,
        url: U,
        query_parameters: &[&T],
    ) -> Result<reqwest::Response, reqwest::Error> {
        self.client
            .get(url)
            .basic_auth(&self.username, Some(&self.password))
            .header(
                reqwest::header::USER_AGENT,
                &format!("{}/{}", PACKAGE_NAME, PACKAGE_VERSION),
            )
            .query(query_parameters)
            .send()
    }

    pub(crate) fn put<T: serde::Serialize + ?Sized, U: reqwest::IntoUrl>(
        &self,
        url: U,
        json: &T,
    ) -> Result<reqwest::Response, reqwest::Error> {
        self.client
            .put(url)
            .basic_auth(&self.username, Some(&self.password))
            .header(
                reqwest::header::USER_AGENT,
                &format!("{}/{}", PACKAGE_NAME, PACKAGE_VERSION),
            )
            .json(json)
            .send()
    }

    pub(crate) fn post<T: serde::Serialize + ?Sized, U: reqwest::IntoUrl>(
        &self,
        url: U,
        json: &T,
    ) -> Result<reqwest::Response, reqwest::Error> {
        self.client
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
    pub fn new(username: &str, password: &str, device_id: &str) -> DeviceClient {
        DeviceClient {
            device_id: device_id.to_owned(),
            authenticated_client: AuthenticatedClient::new(username, password),
        }
    }

    pub(crate) fn get<U: reqwest::IntoUrl>(
        &self,
        url: U,
    ) -> Result<reqwest::Response, reqwest::Error> {
        self.authenticated_client.get(url)
    }

    pub(crate) fn get_with_query<U: reqwest::IntoUrl, T: serde::Serialize + ?Sized>(
        &self,
        url: U,
        query_parameters: &[&T],
    ) -> Result<reqwest::Response, reqwest::Error> {
        self.authenticated_client
            .get_with_query(url, query_parameters)
    }

    pub(crate) fn put<T: serde::Serialize + ?Sized, U: reqwest::IntoUrl>(
        &self,
        url: U,
        json: &T,
    ) -> Result<reqwest::Response, reqwest::Error> {
        self.authenticated_client.put(url, json)
    }

    pub(crate) fn post<T: serde::Serialize + ?Sized, U: reqwest::IntoUrl>(
        &self,
        url: U,
        json: &T,
    ) -> Result<reqwest::Response, reqwest::Error> {
        self.authenticated_client.post(url, json)
    }
}
