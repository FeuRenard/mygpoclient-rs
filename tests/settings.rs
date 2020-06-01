extern crate mygpoclient;

use std::collections::HashMap;
use std::env;
use url::Url;

use mygpoclient::client::DeviceClient;
use mygpoclient::error::Error;
use mygpoclient::settings::GetAccountSettings;
use mygpoclient::settings::GetEpisodeSettings;
use mygpoclient::settings::GetPodcastSettings;
use mygpoclient::settings::SaveAccountSettings;
use mygpoclient::settings::SaveEpisodeSettings;
use mygpoclient::settings::SavePodcastSettings;

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

#[test]
fn test_save_podcast_settings_device_client() -> Result<(), Error> {
    let client = get_device_client();
    let mut set = HashMap::new();
    set.insert(String::from("setting1"), String::from("value1"));
    set.insert(String::from("setting2"), String::from("value2"));
    let remove = vec![String::from("setting3"), String::from("setting4")];

    let settings = client.save_podcast_settings(
        set.clone(),
        remove.clone(),
        Url::parse("http://goinglinux.com/mp3podcast.xml").unwrap(),
    )?;
    assert!(set
        .iter()
        .all(|(key, value)| settings.get_key_value(key).unwrap() == (key, value)));
    assert!(remove.iter().all(|key| settings.get(key).is_none()));
    Ok(())
}

#[test]
fn test_save_episode_settings_device_client() -> Result<(), Error> {
    let client = get_device_client();
    let mut set = HashMap::new();
    set.insert(String::from("setting1"), String::from("value1"));
    set.insert(String::from("setting2"), String::from("value2"));
    let remove = vec![String::from("setting3"), String::from("setting4")];

    let settings = client.save_episode_settings(
        set.clone(),
        remove.clone(),
        Url::parse("http://example.com/feed1.rss").unwrap(),
        Url::parse("http://example.com/files/s01e20.mp3").unwrap(),
    )?;
    assert!(set
        .iter()
        .all(|(key, value)| settings.get_key_value(key).unwrap() == (key, value)));
    assert!(remove.iter().all(|key| settings.get(key).is_none()));
    Ok(())
}

#[test]
fn test_get_account_settings_device_client() -> Result<(), Error> {
    let client = get_device_client();
    client.get_account_settings()?;
    Ok(())
}

#[test]
fn test_get_podcast_settings_device_client() -> Result<(), Error> {
    let client = get_device_client();
    client.get_podcast_settings(Url::parse("http://goinglinux.com/mp3podcast.xml").unwrap())?;
    Ok(())
}

#[test]
fn test_get_episode_settings_device_client() -> Result<(), Error> {
    let client = get_device_client();
    client.get_episode_settings(
        Url::parse("http://example.com/feed1.rss").unwrap(),
        Url::parse("http://example.com/files/s01e20.mp3").unwrap(),
    )?;
    Ok(())
}

fn get_device_client() -> DeviceClient {
    let username = env::var("GPODDER_NET_USERNAME").unwrap();
    let password = env::var("GPODDER_NET_PASSWORD").unwrap();
    let deviceid = env::var("GPODDER_NET_DEVICEID").unwrap();

    DeviceClient::new(&username, &password, &deviceid)
}
