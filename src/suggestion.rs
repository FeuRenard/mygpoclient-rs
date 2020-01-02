use crate::AuthenticatedClient;
use crate::Error;
use serde::Deserialize;

/// A Suggestion as returned by [`Client::get_suggestions`]
#[derive(Deserialize)]
pub struct Suggestion {
    pub website: String,
    pub mygpo_link: String,
    pub description: String,
    pub subscribers: u16,
    pub title: String,
    pub url: String,
    pub subscribers_last_week: u16,
    pub logo_url: Option<String>,
}

/// [Suggestions API](https://gpoddernet.readthedocs.io/en/latest/api/reference/suggestions.html)
pub trait Suggestions {
    /// Retrieve Suggested Podcasts
    ///
    /// # Arguments
    /// * `max_results` - the maximum number of podcasts to return
    ///
    /// # Examples
    ///
    /// ```
    /// use libmygpo_rs::AuthenticatedClient;
    /// use libmygpo_rs::suggestion::Suggestions;
    ///
    /// # let username = std::env::var("GPODDER_NET_USERNAME").unwrap();
    /// # let password = std::env::var("GPODDER_NET_PASSWORD").unwrap();
    /// #
    /// let client = AuthenticatedClient::new(&username, &password);
    ///
    /// let max_results = 3;
    /// let suggestions = client.retrieve_suggested_podcasts(max_results)?;
    ///
    /// assert!(suggestions.len() <= max_results as usize);
    ///
    /// # Ok::<(), libmygpo_rs::Error>(())
    /// ```
    ///
    /// # See also
    ///
    /// - [Suggestions API: Retrieve Suggested Podcasts](https://gpoddernet.readthedocs.io/en/latest/api/reference/suggestions.html#retrieve-suggested-podcasts)
    fn retrieve_suggested_podcasts(&self, max_results: u8) -> Result<Vec<Suggestion>, Error>;
}

impl Suggestions for AuthenticatedClient {
    fn retrieve_suggested_podcasts(&self, max_results: u8) -> Result<Vec<Suggestion>, Error> {
        Ok(self
            .get(&format!(
                "https://gpodder.net/suggestions/{}.json",
                max_results
            ))?
            .json()?)
    }
}
