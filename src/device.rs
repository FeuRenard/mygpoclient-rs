use crate::client::{AuthenticatedClient, DeviceClient};
use crate::Error;
use serde::Deserialize;

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
pub enum DeviceType {
    desktop,
    laptop,
    mobile,
    server,
    other,
}

#[derive(Deserialize)]
pub struct Device {
    pub id: String,
    pub caption: String,
    pub r#type: DeviceType,
    pub subscriptions: u16,
}

/// [Device API](https://gpoddernet.readthedocs.io/en/latest/api/reference/devices.html)
pub trait Devices: ListDevices {}
// TODO https://gpoddernet.readthedocs.io/en/latest/api/reference/devices.html#update-device-data
// TODO https://gpoddernet.readthedocs.io/en/latest/api/reference/devices.html#get-device-updates

pub trait ListDevices {
    /// List Devices
    ///
    /// # Returns
    ///
    /// A `Result` which is
    ///
    /// - `Ok`: A `Vec<Device>` of all devices
    /// - `Err`: A wrapped JSON or network error
    ///
    /// # Examples
    ///
    /// ```
    /// use libmygpo_rs::AuthenticatedClient;
    /// use libmygpo_rs::device::ListDevices;
    ///
    /// # let username = std::env::var("GPODDER_NET_USERNAME").unwrap();
    /// # let password = std::env::var("GPODDER_NET_PASSWORD").unwrap();
    /// #
    /// let client = AuthenticatedClient::new(&username, &password);
    ///
    /// let devices = client.list_devices()?;
    ///
    /// # Ok::<(), libmygpo_rs::Error>(())
    /// ```
    ///
    /// # See also
    ///
    /// - [gpodder.net API Documentation](https://gpoddernet.readthedocs.io/en/latest/api/reference/devices.html#list-devices)
    fn list_devices(&self) -> Result<Vec<Device>, Error>;
}

impl ListDevices for AuthenticatedClient {
    fn list_devices(&self) -> Result<Vec<Device>, Error> {
        Ok(self
            .get(&format!(
                "https://gpodder.net/api/2/devices/{}.json",
                self.username
            ))?
            .json()?)
    }
}

impl ListDevices for DeviceClient {
    fn list_devices(&self) -> Result<Vec<Device>, Error> {
        self.authenticated_client.list_devices()
    }
}
