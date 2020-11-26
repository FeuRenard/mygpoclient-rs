extern crate mygpoclient;

use std::env;

use mygpoclient::client::DeviceClient;
use mygpoclient::device::ListDevices;
use mygpoclient::error::Error;

#[test]
fn test_list_devices_device_client() -> Result<(), Error> {
    let username = env::var("GPODDER_NET_USERNAME").unwrap();
    let password = env::var("GPODDER_NET_PASSWORD").unwrap();
    let deviceid = env::var("GPODDER_NET_DEVICEID").unwrap();

    let client = DeviceClient::new(&username, &password, &deviceid);
    client.list_devices()?;

    Ok(())
}
