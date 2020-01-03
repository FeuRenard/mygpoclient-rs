use crate::client::{AuthenticatedClient, DeviceClient};
use crate::Error;
use serde::{Deserialize, Serialize};

/// The type of the device
#[serde(rename_all = "lowercase")]
#[derive(Deserialize, Serialize)]
pub enum DeviceType {
    Desktop,
    Laptop,
    Mobile,
    Server,
    Other,
}

#[derive(Deserialize)]
pub struct Device {
    pub id: String,
    pub caption: String,
    // TODO #[serde(rename(serialize = "type", deserialize = "type"))]
    pub r#type: DeviceType,
    pub subscriptions: u16,
}

#[derive(Serialize)]
pub(crate) struct DeviceData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) caption: Option<String>,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) device_type: Option<DeviceType>,
}

/// [Device API](https://gpoddernet.readthedocs.io/en/latest/api/reference/devices.html)
pub trait Devices: ListDevices {}
// TODO https://gpoddernet.readthedocs.io/en/latest/api/reference/devices.html#get-device-updates

pub trait UpdateDeviceData {
    /// Update Device Data
    ///
    /// # Parameters
    ///
    /// - `caption`: The new human readable label for the device
    /// - `device_type`: see `DeviceType`
    ///
    /// # Returns
    ///
    /// A `Result` which is
    ///
    /// - `Ok`
    /// - `Err`: A wrapped JSON or network error
    ///
    /// # Examples
    ///
    /// ```
    /// use libmygpo_rs::DeviceClient;
    /// use libmygpo_rs::device::{DeviceType,UpdateDeviceData};
    ///
    /// # let username = std::env::var("GPODDER_NET_USERNAME").unwrap();
    /// # let password = std::env::var("GPODDER_NET_PASSWORD").unwrap();
    /// # let deviceid = std::env::var("GPODDER_NET_DEVICEID").unwrap();
    /// #
    /// let client = DeviceClient::new(&username, &password, &deviceid);
    ///
    /// client.update_device_data("My Phone".to_owned(), DeviceType::Mobile)?;
    /// # Ok::<(), libmygpo_rs::Error>(())
    /// ```
    ///
    /// # See also
    ///
    /// - [gpodder.net API Documentation](https://gpoddernet.readthedocs.io/en/latest/api/reference/devices.html#update-device-data)
    fn update_device_data<T: Into<Option<String>>, U: Into<Option<DeviceType>>>(
        &self,
        caption: T,
        device_type: U,
    ) -> Result<(), Error>;
}

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

impl UpdateDeviceData for DeviceClient {
    fn update_device_data<T: Into<Option<String>>, U: Into<Option<DeviceType>>>(
        &self,
        caption: T,
        device_type: U,
    ) -> Result<(), Error> {
        let input = DeviceData {
            caption: caption.into(),
            device_type: device_type.into(),
        };
        self.post(
            &format!(
                "https://gpodder.net/api/2/devices/{}/{}.json",
                self.authenticated_client.username, self.device_id
            ),
            &input,
        )?;
        Ok(())
    }
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
