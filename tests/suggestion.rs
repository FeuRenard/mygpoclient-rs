extern crate mygpoclient;

use std::env;

use mygpoclient::client::DeviceClient;
use mygpoclient::error::Error;
use mygpoclient::suggestion::RetrieveSuggestedPodcasts;

#[test]
fn test_retrieve_suggested_podcasts_device_client() -> Result<(), Error> {
    let username = env::var("GPODDER_NET_USERNAME").unwrap();
    let password = env::var("GPODDER_NET_PASSWORD").unwrap();
    let deviceid = env::var("GPODDER_NET_DEVICEID").unwrap();

    let max_results = 3;
    let client = DeviceClient::new(&username, &password, &deviceid);
    let suggestions = client.retrieve_suggested_podcasts(max_results)?;
    assert!(suggestions.len() <= max_results as usize);

    Ok(())
}
