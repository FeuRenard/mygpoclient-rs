use crate::Error;
use crate::{AuthenticatedClient, DeviceClient};
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

#[derive(Serialize)]
pub(crate) struct UploadSubscriptionChangesRequest {
    pub(crate) add: Vec<String>,
    pub(crate) remove: Vec<String>,
}

#[derive(Deserialize)]
pub struct UploadSubscriptionChangesResponse {
    pub timestamp: u64,
    pub update_urls: Vec<(String, String)>,
}

#[derive(Deserialize)]
pub struct GetSubscriptionChangesResponse {
    pub add: Vec<String>,
    pub remove: Vec<String>,
    pub timestamp: u64,
}

/// [Subscriptions API](https://gpoddernet.readthedocs.io/en/latest/api/reference/subscriptions.html)
pub trait Subscriptions: AllSubscriptions + SubscriptionsOfDevice + SubscriptionChanges {}

pub trait AllSubscriptions {
    /// Get All Subscriptions
    ///
    /// # Examples
    ///
    /// ```
    /// use libmygpo_rs::AuthenticatedClient;
    /// use libmygpo_rs::subscription::AllSubscriptions;
    ///
    /// # let username = std::env::var("GPODDER_NET_USERNAME").unwrap();
    /// # let password = std::env::var("GPODDER_NET_PASSWORD").unwrap();
    /// #
    /// let client = AuthenticatedClient::new(&username, &password);
    ///
    /// let subscriptions = client.get_all_subscriptions()?;
    /// #
    /// # Ok::<(), libmygpo_rs::Error>(())
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
    /// # See also
    /// https://gpoddernet.readthedocs.io/en/latest/api/reference/subscriptions.html#get-subscriptions-of-device
    fn get_subscriptions_of_device(&self) -> Result<Vec<String>, Error>;

    /// Upload Subscriptions of Device
    ///
    /// # See also
    /// https://gpoddernet.readthedocs.io/en/latest/api/reference/subscriptions.html#upload-subscriptions-of-device
    fn upload_subscriptions_of_device(&self, subscriptions: &[String]) -> Result<(), Error>;
}

pub trait SubscriptionChanges {
    /// Upload Subscription Changes
    ///
    /// # Parameters
    ///
    /// - `add`: A slice of subscription URLs as String, that should be added
    /// - `remove`: A slice of subscription URLs as String, that should be removed
    ///
    /// # See also
    ///
    /// - [API documentation](https://gpoddernet.readthedocs.io/en/latest/api/reference/subscriptions.html#upload-subscription-changes)
    fn upload_subscription_changes(
        &self,
        add: &[String],
        remove: &[String],
    ) -> Result<UploadSubscriptionChangesResponse, Error>;

    /// Get Subscription Changes
    ///
    /// # See also
    ///
    /// - [API documentation](https://gpoddernet.readthedocs.io/en/latest/api/reference/subscriptions.html#get-subscription-changes)
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
