use crate::Error;
use crate::{AuthenticatedClient, DeviceClient};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};

/// A Subscription as returned by [`Client::get_all_subscriptions`]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
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

#[derive(Serialize)]
pub(crate) struct UploadSubscriptionChangesRequest {
    pub(crate) add: Vec<String>,
    pub(crate) remove: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct UploadSubscriptionChangesResponse {
    pub timestamp: u64,
    pub update_urls: Vec<(String, String)>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct GetSubscriptionChangesResponse {
    pub timestamp: u64,
    pub add: Vec<String>,
    pub remove: Vec<String>,
}

/// [Subscriptions API](https://gpoddernet.readthedocs.io/en/latest/api/reference/subscriptions.html)
pub trait Subscriptions: AllSubscriptions + SubscriptionsOfDevice + SubscriptionChanges {}

pub trait AllSubscriptions {
    /// Get All Subscriptions
    ///
    /// # Returns
    ///
    /// A `Result` which is:
    ///
    /// - `Ok`: A `Vec<Subscription>` of all subscriptions
    /// - `Err`: A wrapped JSON or network error
    ///
    /// # Examples
    ///
    /// ```
    /// use mygpoclient::AuthenticatedClient;
    /// use mygpoclient::subscription::AllSubscriptions;
    ///
    /// # let username = std::env::var("GPODDER_NET_USERNAME").unwrap();
    /// # let password = std::env::var("GPODDER_NET_PASSWORD").unwrap();
    /// #
    /// let client = AuthenticatedClient::new(&username, &password);
    ///
    /// let subscriptions = client.get_all_subscriptions()?;
    /// #
    /// # Ok::<(), mygpoclient::Error>(())
    /// ```
    ///
    /// # See also
    ///
    /// - [gpodder.net API Documentation](https://gpoddernet.readthedocs.io/en/latest/api/reference/subscriptions.html#get-all-subscriptions)
    fn get_all_subscriptions(&self) -> Result<Vec<Subscription>, Error>;
}

pub trait SubscriptionsOfDevice {
    /// Get Subscriptions of Device
    ///
    /// # Returns
    ///
    /// A `Result` which is:
    ///
    /// - `Ok`: A `Vec<String>` of all subscriptions as podcast feed URLs
    /// - `Err`: A wrapped JSON or network error
    ///
    /// # Examples
    ///
    /// ```
    /// use mygpoclient::DeviceClient;
    /// use mygpoclient::subscription::SubscriptionsOfDevice;
    ///
    /// # let username = std::env::var("GPODDER_NET_USERNAME").unwrap();
    /// # let password = std::env::var("GPODDER_NET_PASSWORD").unwrap();
    /// # let deviceid = std::env::var("GPODDER_NET_DEVICEID").unwrap();
    /// #
    /// let client = DeviceClient::new(&username, &password, &deviceid);
    ///
    /// let subscriptions = client.get_subscriptions_of_device()?;
    /// #
    /// # Ok::<(), mygpoclient::Error>(())
    /// ```
    ///
    /// # See also
    /// - [gpodder.net API Documentation](https://gpoddernet.readthedocs.io/en/latest/api/reference/subscriptions.html#get-subscriptions-of-device)
    fn get_subscriptions_of_device(&self) -> Result<Vec<String>, Error>;

    /// Upload Subscriptions of Device
    ///
    /// # Parameters
    ///
    /// - `subscriptions`: A slice of podcast feed URLs as String representing the current subscription list
    ///
    /// # See also
    /// - [gpodder.net API Documentation](https://gpoddernet.readthedocs.io/en/latest/api/reference/subscriptions.html#upload-subscriptions-of-device)
    fn upload_subscriptions_of_device(&self, subscriptions: &[String]) -> Result<(), Error>;
}

pub trait SubscriptionChanges {
    /// Upload Subscription Changes
    ///
    /// # Parameters
    ///
    /// - `add`: A slice of podcast feed URLs as String, that should be added to the device subscriptions
    /// - `remove`: A slice of podcast feed URLs as String, that should be removed from the device subscriptions
    ///
    /// # Returns
    ///
    /// A `Result` which is:
    ///
    /// - `Ok`: A `UploadSubscriptionChangesResponse`
    /// - `Err`: A wrapped JSON or network error
    ///
    /// # Examples
    ///
    /// ```
    /// use mygpoclient::DeviceClient;
    /// use mygpoclient::subscription::SubscriptionChanges;
    ///
    /// # let username = std::env::var("GPODDER_NET_USERNAME").unwrap();
    /// # let password = std::env::var("GPODDER_NET_PASSWORD").unwrap();
    /// # let deviceid = std::env::var("GPODDER_NET_DEVICEID").unwrap();
    /// #
    /// let client = DeviceClient::new(&username, &password, &deviceid);
    ///
    /// # let url1 = "http://example.com/feed.rss".to_owned();
    /// # let url2 = "http://example.org/podcast.php".to_owned();
    /// # let url3 = "http://example.net/foo.xml".to_owned();
    /// #
    /// let add = vec![url1,url2];
    /// let remove = vec![url3];
    /// let response = client.upload_subscription_changes(&add, &remove)?;
    /// #
    /// # Ok::<(), mygpoclient::Error>(())
    /// ```
    ///
    /// # See also
    ///
    /// - [gpodder.net API Documentation](https://gpoddernet.readthedocs.io/en/latest/api/reference/subscriptions.html#upload-subscription-changes)
    fn upload_subscription_changes(
        &self,
        add: &[String],
        remove: &[String],
    ) -> Result<UploadSubscriptionChangesResponse, Error>;

    /// Get Subscription Changes
    ///
    /// # Returns
    ///
    /// A `Result` which is:
    ///
    /// - `Ok`: A `GetSubscriptionChangesResponse`
    /// - `Err`: A wrapped JSON or network error
    ///
    /// # Examples
    ///
    /// ```
    /// use mygpoclient::DeviceClient;
    /// use mygpoclient::subscription::SubscriptionChanges;
    ///
    /// # let username = std::env::var("GPODDER_NET_USERNAME").unwrap();
    /// # let password = std::env::var("GPODDER_NET_PASSWORD").unwrap();
    /// # let deviceid = std::env::var("GPODDER_NET_DEVICEID").unwrap();
    /// #
    /// let client = DeviceClient::new(&username, &password, &deviceid);
    ///
    /// let subscription_changes = client.get_subscription_changes(0)?;
    /// #
    /// # Ok::<(), mygpoclient::Error>(())
    /// ```
    ///
    /// # See also
    ///
    /// - [gpodder.net API Documentation](https://gpoddernet.readthedocs.io/en/latest/api/reference/subscriptions.html#get-subscription-changes)
    fn get_subscription_changes(
        &self,
        timestamp: u64,
    ) -> Result<GetSubscriptionChangesResponse, Error>;
}

impl AllSubscriptions for AuthenticatedClient {
    fn get_all_subscriptions(&self) -> Result<Vec<Subscription>, Error> {
        Ok(self
            .get(&format!(
                "https://gpodder.net/subscriptions/{}.json",
                self.username
            ))?
            .json()?)
    }
}

impl AllSubscriptions for DeviceClient {
    fn get_all_subscriptions(&self) -> Result<Vec<Subscription>, Error> {
        self.authenticated_client.get_all_subscriptions()
    }
}

impl SubscriptionsOfDevice for DeviceClient {
    fn get_subscriptions_of_device(&self) -> Result<Vec<String>, Error> {
        Ok(self
            .get(&format!(
                "https://gpodder.net/subscriptions/{}/{}.json",
                self.authenticated_client.username, self.device_id
            ))?
            .json()?) // TODO handle response?
    }

    fn upload_subscriptions_of_device(&self, subscriptions: &[String]) -> Result<(), Error> {
        self.put(
            &format!(
                "https://gpodder.net/subscriptions/{}/{}.json",
                self.authenticated_client.username, self.device_id
            ),
            subscriptions,
        )?; // TODO handle response?
        Ok(())
    }
}

impl SubscriptionChanges for DeviceClient {
    fn upload_subscription_changes(
        &self,
        add: &[String],
        remove: &[String],
    ) -> Result<UploadSubscriptionChangesResponse, Error> {
        let input = UploadSubscriptionChangesRequest {
            add: add.to_owned(),
            remove: remove.to_owned(),
        };
        Ok(self
            .post(
                &format!(
                    "https://gpodder.net/api/2/subscriptions/{}/{}.json",
                    self.authenticated_client.username, self.device_id
                ),
                &input,
            )?
            .json()?)
    }

    fn get_subscription_changes(
        &self,
        timestamp: u64,
    ) -> Result<GetSubscriptionChangesResponse, Error> {
        Ok(self
            .get_with_query(
                &format!(
                    "https://gpodder.net/api/2/subscriptions/{}/{}.json",
                    self.authenticated_client.username, self.device_id
                ),
                &[&("since", timestamp)],
            )?
            .json()?)
    }
}

impl PartialEq for Subscription {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
    }
}

impl Eq for Subscription {}

impl Ord for Subscription {
    fn cmp(&self, other: &Self) -> Ordering {
        self.url.cmp(&other.url)
    }
}

impl PartialOrd for Subscription {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for Subscription {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.url.hash(state);
    }
}

impl fmt::Display for Subscription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {} <{}>", self.title, self.description, self.url)
    }
}

impl fmt::Display for UploadSubscriptionChangesResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {:?}>", self.timestamp, self.update_urls)
    }
}

impl fmt::Display for GetSubscriptionChangesResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: add{:?}, remove{:?}>",
            self.timestamp, self.add, self.remove
        )
    }
}
