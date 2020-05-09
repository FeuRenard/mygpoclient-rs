extern crate mygpoclient;

use std::collections::HashMap;
use std::env;

use mygpoclient::client::DeviceClient;
use mygpoclient::error::Error;
use mygpoclient::settings::SaveAccountSettings;

#[test]
fn test_save_account_settings_device_client() -> Result<(), Error> {
    let client = get_device_client();
    let mut set = HashMap::new();
    set.insert(String::from("setting1"), String::from("value1"));
    set.insert(String::from("setting2"), String::from("value2"));
    let remove = vec![String::from("setting3"), String::from("setting4")];

    let settings = client.save_account_settings(set.clone(), remove.clone())?;
    assert!(set
        .iter()
        .all(|(key, value)| settings.get_key_value(key).unwrap() == (key, value)));
    assert!(remove.iter().all(|key| settings.get(key).is_none()));
    Ok(())
}

fn get_device_client() -> DeviceClient {
    let username = env::var("GPODDER_NET_USERNAME").unwrap();
    let password = env::var("GPODDER_NET_PASSWORD").unwrap();
    let deviceid = env::var("GPODDER_NET_DEVICEID").unwrap();

    DeviceClient::new(&username, &password, &deviceid)
}
