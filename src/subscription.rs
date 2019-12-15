use crate::Client;
use crate::Error;
use serde::{Deserialize, Serialize};

/// A Subscription as returned by [`Client::get_all_subscriptions`]
#[derive(Serialize, Deserialize)]
pub struct Subscription {
    pub url: String,
    pub title: String,
    pub description: String,
    pub subscribers: u16,
    pub subscribers_last_week: u16,
    pub logo_url: Option<String>,
    pub scaled_logo_url: Option<String>,
    pub website: Option<String>,
    pub mygpo_link: String,
}

impl Client {
    /// Get All Subscriptions
    ///
    /// # See also
    /// https://gpoddernet.readthedocs.io/en/latest/api/reference/subscriptions.html#get-all-subscriptions
    pub fn get_all_subscriptions(&self) -> Result<Vec<Subscription>, Error> {
        Ok(self
            .get(&format!(
                "https://gpodder.net/subscriptions/{}.json",
                self.username
            ))?
            .json()?)
    }

    /// Get Subscriptions of Device
    ///
    /// # See also
    /// https://gpoddernet.readthedocs.io/en/latest/api/reference/subscriptions.html#get-subscriptions-of-device
    pub fn get_subscriptions(&self, deviceid: &str) -> Result<Vec<String>, Error> {
        Ok(self
            .get(&format!(
                "https://gpodder.net/subscriptions/{}/{}.json",
                self.username, deviceid
            ))?
            .json()?) // TODO handle response?
    }

    /// Upload Subscriptions of Device
    ///
    /// # See also
    /// https://gpoddernet.readthedocs.io/en/latest/api/reference/subscriptions.html#upload-subscriptions-of-device
    pub fn put_subscriptions(&self, subscriptions: &[String], deviceid: &str) -> Result<(), Error> {
        self.put(
            &format!(
                "https://gpodder.net/subscriptions/{}/{}.json",
                self.username, deviceid
            ),
            subscriptions,
        )?; // TODO handle response?
        Ok(())
    }
}

// TODO unit tests
