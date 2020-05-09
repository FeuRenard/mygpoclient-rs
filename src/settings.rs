//! [Settings API](https://gpoddernet.readthedocs.io/en/latest/api/reference/settings.html)

use crate::client::AuthenticatedClient;
use crate::client::DeviceClient;
use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub(crate) enum SettingsScope {
    /// user account
    Account,
    /// user device
    Device {
        /// device id
        id: String,
    },
    /// podcast feed
    Podcast {
        /// feed url
        url: Url,
    },
    /// podcast episode
    Episode {
        /// media url
        url: Url,
        /// feed url
        feed_url: Url,
    },
}

#[derive(Serialize)]
pub(crate) struct SaveSettingsRequest {
    pub(crate) set: HashMap<String, String>,
    pub(crate) remove: Vec<String>,
}

/// see [`save_account_settings`](./trait.SaveAccountSettings.html#tymethod.save_account_settings)
pub trait SaveAccountSettings {
    /// Save Account Settings
    ///
    /// # Examples
    ///
    /// ```
    /// use mygpoclient::client::AuthenticatedClient;
    /// use mygpoclient::settings::SaveAccountSettings;
    /// use std::collections::HashMap;
    ///
    /// # let username = std::env::var("GPODDER_NET_USERNAME").unwrap();
    /// # let password = std::env::var("GPODDER_NET_PASSWORD").unwrap();
    /// #
    /// let client = AuthenticatedClient::new(&username, &password);
    /// let mut set = HashMap::new();
    /// set.insert(String::from("setting1"), String::from("value1"));
    /// set.insert(String::from("setting2"), String::from("value2"));
    /// let remove = vec![String::from("setting3"), String::from("setting4")];
    ///
    /// let settings = client.save_account_settings(set.clone(), remove.clone())?;
    /// assert!(set.iter().all(|(key, value)| settings.get_key_value(key).unwrap() == (key, value)));
    /// assert!(remove.iter().all(|key| settings.get(key).is_none()));
    /// #
    /// # Ok::<(), mygpoclient::error::Error>(())
    /// ```
    ///
    /// # See also
    ///
    /// - [gpodder.net API Documentation](https://gpoddernet.readthedocs.io/en/latest/api/reference/settings.html#save-settings)
    fn save_account_settings(
        &self,
        set: HashMap<String, String>,
        remove: Vec<String>,
    ) -> Result<HashMap<String, String>, Error>;
}

impl SaveAccountSettings for AuthenticatedClient {
    fn save_account_settings(
        &self,
        set: HashMap<String, String>,
        remove: Vec<String>,
    ) -> Result<HashMap<String, String>, Error> {
        Ok(self
            .post(
                &format!(
                    "https://gpodder.net/api/2/settings/{}/account.json",
                    self.username
                ),
                &SaveSettingsRequest { set, remove },
            )?
            .json()?)
    }
}

impl SaveAccountSettings for DeviceClient {
    fn save_account_settings(
        &self,
        set: HashMap<String, String>,
        remove: Vec<String>,
    ) -> Result<HashMap<String, String>, Error> {
        self.authenticated_client.save_account_settings(set, remove)
    }
}
