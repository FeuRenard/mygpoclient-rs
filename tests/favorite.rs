extern crate mygpoclient;

use std::env;

use mygpoclient::client::DeviceClient;
use mygpoclient::error::Error;
use mygpoclient::favorite::GetFavoriteEpisodes;

#[test]
fn test_get_favorite_episodes_device_client() -> Result<(), Error> {
    let username = env::var("GPODDER_NET_USERNAME").unwrap();
    let password = env::var("GPODDER_NET_PASSWORD").unwrap();
    let deviceid = env::var("GPODDER_NET_DEVICEID").unwrap();

    let client = DeviceClient::new(&username, &password, &deviceid);
    client.get_favorite_episodes()?;

    Ok(())
}
