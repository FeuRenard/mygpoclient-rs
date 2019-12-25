const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Client {
    pub(crate) username: String,
    pub(crate) password: String,
    client: reqwest::Client,
}

impl Client {
    pub fn new(username: &str, password: &str) -> Client {
        Client {
            username: username.to_owned(),
            password: password.to_owned(),
            client: reqwest::Client::new(),
        }
    }

    pub(crate) fn get<U: reqwest::IntoUrl>(
        &self,
        url: U,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let empty_slice: &[&(&String, &String)] = &[];
        self.get_with_query(url, empty_slice)
    }

    // TODO force key-value pairs for query parameters
    pub(crate) fn get_with_query<U: reqwest::IntoUrl, T: ToString + ?Sized>(
        &self,
        url: U,
        query_parameters: &[&(&T, &T)],
    ) -> Result<reqwest::Response, reqwest::Error> {
        let mut request_builder = self
            .client
            .get(url)
            .basic_auth(&self.username, Some(&self.password))
            .header(
                reqwest::header::USER_AGENT,
                &format!("{}/{}", PACKAGE_NAME, PACKAGE_VERSION),
            );

        for (key, value) in query_parameters {
            request_builder = request_builder.query(&(key.to_string(), value.to_string()));
        }

        request_builder.send()
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
