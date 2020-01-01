use crate::Client;
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

impl Client {
    /// Retrieve Suggested Podcasts
    ///
    /// # Arguments
    /// * `number` - the maximum number of podcasts to return
    ///
    /// # Examples
    ///
    /// ```
    /// use libmygpo_rs::Client;
    ///
    /// # let username = std::env::var("GPODDER_NET_USERNAME").unwrap();
    /// # let password = std::env::var("GPODDER_NET_PASSWORD").unwrap();
    /// #
    /// let client = Client::new(&username, &password);
    ///
    /// let max_results = 3;
    /// let suggestions = client.get_suggestions(max_results)?;
    ///
    /// assert!(suggestions.len() <= max_results as usize);
    ///
    /// # Ok::<(), libmygpo_rs::Error>(())
    /// ```
    ///
    /// # See also
    ///
    /// - [Suggestions API: Retrieve Suggested Podcasts](https://gpoddernet.readthedocs.io/en/latest/api/reference/suggestions.html#retrieve-suggested-podcasts)
    pub fn get_suggestions(&self, number: u8) -> Result<Vec<Suggestion>, Error> {
        Ok(self
            .get(&format!("https://gpodder.net/suggestions/{}.json", number))?
            .json()?)
    }
}
