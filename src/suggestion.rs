use crate::AuthenticatedClient;
use crate::Error;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};

/// A Suggestion as returned by [`Client::get_suggestions`]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
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
    /// use mygpoclient::AuthenticatedClient;
    /// use mygpoclient::suggestion::Suggestions;
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
    /// # Ok::<(), mygpoclient::Error>(())
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

impl PartialEq for Suggestion {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
    }
}

impl Eq for Suggestion {}

impl Ord for Suggestion {
    fn cmp(&self, other: &Self) -> Ordering {
        self.url.cmp(&other.url)
    }
}

impl PartialOrd for Suggestion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for Suggestion {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.url.hash(state);
    }
}

impl fmt::Display for Suggestion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {} <{}>", self.title, self.description, self.url)
    }
}
