//! [Subscriptions API](https://gpoddernet.readthedocs.io/en/latest/api/reference/subscriptions.html)

use crate::client::AuthenticatedClient;
use crate::client::DeviceClient;
use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use url::Url;

/// A Subscription as returned by [`get_all_subscriptions`](./trait.GetAllSubscriptions.html#tymethod.get_all_subscriptions)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Subscription {
    /// feed URL
    pub url: Url,
    /// title of podcast
    pub title: String,
    /// description of podcast
    pub description: String,
    /// number of subscribers on service
    pub subscribers: u16,
    /// number of subscribers on service one week before
    pub subscribers_last_week: u16,
    /// URL to logo of podcast
    pub logo_url: Option<Url>,
    /// URL to a scaled logo of podcast
    pub scaled_logo_url: Option<Url>,
    /// website of podcast
    pub website: Option<Url>,
    /// service-internal feed URL
    pub mygpo_link: Url,
}

#[derive(Serialize)]
pub(crate) struct UploadSubscriptionChangesRequest {
    pub(crate) add: Vec<String>,
    pub(crate) remove: Vec<String>,
}

/// Response to [`upload_subscription_changes`](./trait.SubscriptionChanges.html#tymethod.upload_subscription_changes)
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct UploadSubscriptionChangesResponse {
    /// timestamp/ID that can be used for requesting changes since this upload in a subsequent API call
    pub timestamp: u64,
    /// list of URLs that have been rewritten as a list of tuples
    ///
    /// The client SHOULD parse this list and update the local subscription list accordingly (the server only sanitizes the URL, so the semantic “content” should stay the same and therefore the client can simply update the URL value locally and use it for future updates.
    pub update_urls: Vec<(String, String)>,
}

/// Response to [`get_subscription_changes`](/trait.SubscriptionChanges.html#tymethod.get_subscription_changes)
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct GetSubscriptionChangesResponse {
    /// The timestamp SHOULD be stored by the client in order to provide it in the since parameter in the next request.
    pub timestamp: u64,
    /// URLs that should be added
    pub add: Vec<Url>,
    /// URLs that should be removed
    pub remove: Vec<Url>,
}

/// see [`get_all_subscriptions`](./trait.GetAllSubscriptions.html#tymethod.get_all_subscriptions)
pub trait GetAllSubscriptions {
    /// Get All Subscriptions
    ///
    /// This can be used to present the user a list of podcasts when the application starts for the first time.
    ///
    /// # Examples
    ///
    /// ```
    /// use mygpoclient::client::AuthenticatedClient;
    /// use mygpoclient::subscription::GetAllSubscriptions;
    ///
    /// # let username = std::env::var("GPODDER_NET_USERNAME").unwrap();
    /// # let password = std::env::var("GPODDER_NET_PASSWORD").unwrap();
    /// #
    /// let client = AuthenticatedClient::new(&username, &password);
    ///
    /// let subscriptions = client.get_all_subscriptions()?;
    /// #
    /// # Ok::<(), mygpoclient::error::Error>(())
    /// ```
    ///
    /// # See also
    ///
    /// - [gpodder.net API Documentation](https://gpoddernet.readthedocs.io/en/latest/api/reference/subscriptions.html#get-all-subscriptions)
    fn get_all_subscriptions(&self) -> Result<Vec<Subscription>, Error>;
}

/// Get and upload subscriptions of a device
pub trait SubscriptionsOfDevice {
    /// Get Subscriptions of Device
    ///
    /// # Examples
    ///
    /// ```
    /// use mygpoclient::client::DeviceClient;
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
    /// # Ok::<(), mygpoclient::error::Error>(())
    /// ```
    ///
    /// # See also
    /// - [gpodder.net API Documentation](https://gpoddernet.readthedocs.io/en/latest/api/reference/subscriptions.html#get-subscriptions-of-device)
    fn get_subscriptions_of_device(&self) -> Result<Vec<Url>, Error>;

    /// Upload the current subscription list of the given user to the server.
    ///
    /// # See also
    /// - [gpodder.net API Documentation](https://gpoddernet.readthedocs.io/en/latest/api/reference/subscriptions.html#upload-subscriptions-of-device)
    fn upload_subscriptions_of_device(&self, subscriptions: &[Url]) -> Result<(), Error>;
}

/// Get or upload subscription changes
pub trait SubscriptionChanges {
    /// Upload Subscription Changes
    ///
    /// Only deltas are supported here. Timestamps are not supported, and are issued by the server.
    ///
    /// # Examples
    ///
    /// ```
    /// use mygpoclient::client::DeviceClient;
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
    /// # Ok::<(), mygpoclient::error::Error>(())
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
    /// # Examples
    ///
    /// ```
    /// use mygpoclient::client::DeviceClient;
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
    /// # Ok::<(), mygpoclient::error::Error>(())
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

impl GetAllSubscriptions for AuthenticatedClient {
    fn get_all_subscriptions(&self) -> Result<Vec<Subscription>, Error> {
        Ok(self
            .get(&format!(
                "https://gpodder.net/subscriptions/{}.json",
                self.username
            ))?
            .json()?)
    }
}

impl GetAllSubscriptions for DeviceClient {
    fn get_all_subscriptions(&self) -> Result<Vec<Subscription>, Error> {
        self.as_ref().get_all_subscriptions()
    }
}

impl SubscriptionsOfDevice for DeviceClient {
    fn get_subscriptions_of_device(&self) -> Result<Vec<Url>, Error> {
        Ok(self
            .get(&format!(
                "https://gpodder.net/subscriptions/{}/{}.json",
                self.authenticated_client.username, self.device_id
            ))?
            .json()?) // TODO handle response?
    }

    fn upload_subscriptions_of_device(&self, subscriptions: &[Url]) -> Result<(), Error> {
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

#[cfg(test)]
mod tests {
    use super::Subscription;
    use std::cmp::Ordering;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use url::Url;

    #[test]
    fn equal_subscription_means_equal_hash() {
        let subscription1 = Subscription {
            url: Url::parse("http://goinglinux.com/mp3podcast.xml").unwrap(),
            website: Some(Url::parse("http://www.linuxgeekdom.com").unwrap()),
            mygpo_link: Url::parse("http://gpodder.net/podcast/64439").unwrap(),
            description: String::from("Linux Geekdom"),
            subscribers: 0,
            title: String::from("Linux Geekdom"),
            subscribers_last_week: 0,
            logo_url: None,
            scaled_logo_url: None,
        };
        let subscription2 = Subscription {
            url: Url::parse("http://goinglinux.com/mp3podcast.xml").unwrap(),
            website: Some(Url::parse("http://goinglinux.com").unwrap()),
            mygpo_link: Url::parse("http://gpodder.net/podcast/11171").unwrap(),
            description: String::from("Going Linux"),
            subscribers: 571,
            title: String::from("Going Linux"),
            subscribers_last_week: 571,
            logo_url: Some(Url::parse("http://goinglinux.com/images/GoingLinux80.png").unwrap()),
            scaled_logo_url: Some(
                Url::parse("http://goinglinux.com/images/GoingLinux80.png").unwrap(),
            ),
        };

        assert_eq!(subscription1, subscription2);
        assert_eq!(
            subscription1.partial_cmp(&subscription2),
            Some(Ordering::Equal)
        );

        let mut hasher1 = DefaultHasher::new();
        subscription1.hash(&mut hasher1);

        let mut hasher2 = DefaultHasher::new();
        subscription2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn display() {
        let subscription = Subscription {
            url: Url::parse("http://goinglinux.com/mp3podcast.xml").unwrap(),
            website: Some(Url::parse("http://goinglinux.com").unwrap()),
            mygpo_link: Url::parse("http://gpodder.net/podcast/11171").unwrap(),
            description: String::from("Going Linux"),
            subscribers: 571,
            title: String::from("Going Linux"),
            subscribers_last_week: 571,
            logo_url: Some(Url::parse("http://goinglinux.com/images/GoingLinux80.png").unwrap()),
            scaled_logo_url: Some(
                Url::parse("http://goinglinux.com/images/GoingLinux80.png").unwrap(),
            ),
        };

        assert_eq!(
            "Going Linux: Going Linux <http://goinglinux.com/mp3podcast.xml>".to_owned(),
            format!("{}", subscription)
        );
    }
}
