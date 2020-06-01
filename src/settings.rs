//! [Settings API](https://gpoddernet.readthedocs.io/en/latest/api/reference/settings.html)

use crate::client::AuthenticatedClient;
use crate::client::DeviceClient;
use crate::error::Error;
use serde::Serialize;
use std::collections::HashMap;
use url::Url;

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

/// see [`save_device_settings`](./trait.SaveDeviceSettings.html#tymethod.save_device_settings)
pub trait SaveDeviceSettings {
    /// Save Device Settings
    ///
    /// # Examples
    ///
    /// ```
    /// use mygpoclient::client::DeviceClient;
    /// use mygpoclient::settings::SaveDeviceSettings;
    /// use std::collections::HashMap;
    ///
    /// # let username = std::env::var("GPODDER_NET_USERNAME").unwrap();
    /// # let password = std::env::var("GPODDER_NET_PASSWORD").unwrap();
    /// # let deviceid = std::env::var("GPODDER_NET_DEVICEID").unwrap();
    /// #
    /// let client = DeviceClient::new(&username, &password, &deviceid);
    /// let mut set = HashMap::new();
    /// set.insert(String::from("setting1"), String::from("value1"));
    /// set.insert(String::from("setting2"), String::from("value2"));
    /// let remove = vec![String::from("setting3"), String::from("setting4")];
    ///
    /// let settings = client.save_device_settings(set.clone(), remove.clone())?;
    /// assert!(set.iter().all(|(key, value)| settings.get_key_value(key).unwrap() == (key, value)));
    /// assert!(remove.iter().all(|key| settings.get(key).is_none()));
    /// #
    /// # Ok::<(), mygpoclient::error::Error>(())
    /// ```
    ///
    /// # See also
    ///
    /// - [gpodder.net API Documentation](https://gpoddernet.readthedocs.io/en/latest/api/reference/settings.html#save-settings)
    fn save_device_settings(
        &self,
        set: HashMap<String, String>,
        remove: Vec<String>,
    ) -> Result<HashMap<String, String>, Error>;
}

/// see [`save_podcast_settings`](./trait.SavePodcastSettings.html#tymethod.save_podcast_settings)
pub trait SavePodcastSettings {
    /// Save Podcast Settings
    ///
    /// # Examples
    ///
    /// ```
    /// use mygpoclient::client::AuthenticatedClient;
    /// use mygpoclient::settings::SavePodcastSettings;
    /// use std::collections::HashMap;
    /// use url::Url;
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
    /// let settings = client.save_podcast_settings(set.clone(), remove.clone(), Url::parse("http://goinglinux.com/mp3podcast.xml").unwrap())?;
    /// assert!(set.iter().all(|(key, value)| settings.get_key_value(key).unwrap() == (key, value)));
    /// assert!(remove.iter().all(|key| settings.get(key).is_none()));
    /// #
    /// # Ok::<(), mygpoclient::error::Error>(())
    /// ```
    ///
    /// # See also
    ///
    /// - [gpodder.net API Documentation](https://gpoddernet.readthedocs.io/en/latest/api/reference/settings.html#save-settings)
    fn save_podcast_settings(
        &self,
        set: HashMap<String, String>,
        remove: Vec<String>,
        podcast: Url,
    ) -> Result<HashMap<String, String>, Error>;
}

/// see [`save_episode_settings`](./trait.SaveEpisodeSettings.html#tymethod.save_episode_settings)
pub trait SaveEpisodeSettings {
    /// Save Podcast Settings
    ///
    /// # Examples
    ///
    /// ```
    /// use mygpoclient::client::AuthenticatedClient;
    /// use mygpoclient::settings::SaveEpisodeSettings;
    /// use std::collections::HashMap;
    /// use url::Url;
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
    /// let settings = client.save_episode_settings(set.clone(), remove.clone(), Url::parse("http://example.com/feed1.rss").unwrap(), Url::parse("http://example.com/files/s01e20.mp3").unwrap())?;
    /// assert!(set.iter().all(|(key, value)| settings.get_key_value(key).unwrap() == (key, value)));
    /// assert!(remove.iter().all(|key| settings.get(key).is_none()));
    /// #
    /// # Ok::<(), mygpoclient::error::Error>(())
    /// ```
    ///
    /// # See also
    ///
    /// - [gpodder.net API Documentation](https://gpoddernet.readthedocs.io/en/latest/api/reference/settings.html#save-settings)
    fn save_episode_settings(
        &self,
        set: HashMap<String, String>,
        remove: Vec<String>,
        podcast: Url,
        episode: Url,
    ) -> Result<HashMap<String, String>, Error>;
}

/// see [`get_account_settings`](./trait.GetAccountSettings.html#tymethod.get_account_settings)
pub trait GetAccountSettings {
    /// Get Account Settings
    ///
    /// # Examples
    ///
    /// ```
    /// use mygpoclient::client::AuthenticatedClient;
    /// use mygpoclient::settings::GetAccountSettings;
    ///
    /// # let username = std::env::var("GPODDER_NET_USERNAME").unwrap();
    /// # let password = std::env::var("GPODDER_NET_PASSWORD").unwrap();
    /// #
    /// let client = AuthenticatedClient::new(&username, &password);
    ///
    /// let settings = client.get_account_settings()?;
    /// #
    /// # Ok::<(), mygpoclient::error::Error>(())
    /// ```
    ///
    /// # See also
    ///
    /// - [gpodder.net API Documentation](https://gpoddernet.readthedocs.io/en/latest/api/reference/settings.html#get-settings)
    fn get_account_settings(&self) -> Result<HashMap<String, String>, Error>;
}

/// see [`get_device_settings`](./trait.GetDeviceSettings.html#tymethod.get_device_settings)
pub trait GetDeviceSettings {
    /// Get Device Settings
    ///
    /// # Examples
    ///
    /// ```
    /// use mygpoclient::client::DeviceClient;
    /// use mygpoclient::settings::GetDeviceSettings;
    ///
    /// # let username = std::env::var("GPODDER_NET_USERNAME").unwrap();
    /// # let password = std::env::var("GPODDER_NET_PASSWORD").unwrap();
    /// # let deviceid = std::env::var("GPODDER_NET_DEVICEID").unwrap();
    /// #
    /// let client = DeviceClient::new(&username, &password, &deviceid);
    ///
    /// let settings = client.get_device_settings()?;
    /// #
    /// # Ok::<(), mygpoclient::error::Error>(())
    /// ```
    ///
    /// # See also
    ///
    /// - [gpodder.net API Documentation](https://gpoddernet.readthedocs.io/en/latest/api/reference/settings.html#get-settings)
    fn get_device_settings(&self) -> Result<HashMap<String, String>, Error>;
}

/// see [`get_podcast_settings`](./trait.GetPodcastSettings.html#tymethod.get_podcast_settings)
pub trait GetPodcastSettings {
    /// Get Podcast Settings
    ///
    /// # Examples
    ///
    /// ```
    /// use mygpoclient::client::AuthenticatedClient;
    /// use mygpoclient::settings::GetPodcastSettings;
    /// use url::Url;
    ///
    /// # let username = std::env::var("GPODDER_NET_USERNAME").unwrap();
    /// # let password = std::env::var("GPODDER_NET_PASSWORD").unwrap();
    /// #
    /// let client = AuthenticatedClient::new(&username, &password);
    ///
    /// let settings = client.get_podcast_settings(Url::parse("http://goinglinux.com/mp3podcast.xml").unwrap())?;
    /// #
    /// # Ok::<(), mygpoclient::error::Error>(())
    /// ```
    ///
    /// # See also
    ///
    /// - [gpodder.net API Documentation](https://gpoddernet.readthedocs.io/en/latest/api/reference/settings.html#get-settings)
    fn get_podcast_settings(&self, podcast: Url) -> Result<HashMap<String, String>, Error>;
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

impl SaveDeviceSettings for DeviceClient {
    fn save_device_settings(
        &self,
        set: HashMap<String, String>,
        remove: Vec<String>,
    ) -> Result<HashMap<String, String>, Error> {
        Ok(self
            .post_with_query(
                &format!(
                    "https://gpodder.net/api/2/settings/{}/device.json",
                    self.authenticated_client.username
                ),
                &SaveSettingsRequest { set, remove },
                &[&("device", self.device_id.as_str())],
            )?
            .json()?)
    }
}

impl SavePodcastSettings for AuthenticatedClient {
    fn save_podcast_settings(
        &self,
        set: HashMap<String, String>,
        remove: Vec<String>,
        podcast: Url,
    ) -> Result<HashMap<String, String>, Error> {
        Ok(self
            .post_with_query(
                &format!(
                    "https://gpodder.net/api/2/settings/{}/podcast.json",
                    self.username
                ),
                &SaveSettingsRequest { set, remove },
                &[&("podcast", podcast.as_str())],
            )?
            .json()?)
    }
}

impl SavePodcastSettings for DeviceClient {
    fn save_podcast_settings(
        &self,
        set: HashMap<String, String>,
        remove: Vec<String>,
        podcast: Url,
    ) -> Result<HashMap<String, String>, Error> {
        self.authenticated_client
            .save_podcast_settings(set, remove, podcast)
    }
}

impl SaveEpisodeSettings for AuthenticatedClient {
    fn save_episode_settings(
        &self,
        set: HashMap<String, String>,
        remove: Vec<String>,
        podcast: Url,
        episode: Url,
    ) -> Result<HashMap<String, String>, Error> {
        Ok(self
            .post_with_query(
                &format!(
                    "https://gpodder.net/api/2/settings/{}/episode.json",
                    self.username
                ),
                &SaveSettingsRequest { set, remove },
                &[
                    &("podcast", podcast.as_str()),
                    &("episode", episode.as_str()),
                ],
            )?
            .json()?)
    }
}

impl SaveEpisodeSettings for DeviceClient {
    fn save_episode_settings(
        &self,
        set: HashMap<String, String>,
        remove: Vec<String>,
        podcast: Url,
        episode: Url,
    ) -> Result<HashMap<String, String>, Error> {
        self.authenticated_client
            .save_episode_settings(set, remove, podcast, episode)
    }
}

impl GetAccountSettings for AuthenticatedClient {
    fn get_account_settings(&self) -> Result<HashMap<String, String>, Error> {
        Ok(self
            .get(&format!(
                "https://gpodder.net/api/2/settings/{}/account.json",
                self.username
            ))?
            .json()?)
    }
}

impl GetAccountSettings for DeviceClient {
    fn get_account_settings(&self) -> Result<HashMap<String, String>, Error> {
        self.authenticated_client.get_account_settings()
    }
}

impl GetDeviceSettings for DeviceClient {
    fn get_device_settings(&self) -> Result<HashMap<String, String>, Error> {
        Ok(self
            .get_with_query(
                &format!(
                    "https://gpodder.net/api/2/settings/{}/device.json",
                    self.authenticated_client.username
                ),
                &[&("device", self.device_id.as_str())],
            )?
            .json()?)
    }
}

impl GetPodcastSettings for AuthenticatedClient {
    fn get_podcast_settings(&self, podcast: Url) -> Result<HashMap<String, String>, Error> {
        Ok(self
            .get_with_query(
                &format!(
                    "https://gpodder.net/api/2/settings/{}/podcast.json",
                    self.username
                ),
                &[&("podcast", podcast.as_str())],
            )?
            .json()?)
    }
}

impl GetPodcastSettings for DeviceClient {
    fn get_podcast_settings(&self, podcast: Url) -> Result<HashMap<String, String>, Error> {
        self.authenticated_client.get_podcast_settings(podcast)
    }
}
