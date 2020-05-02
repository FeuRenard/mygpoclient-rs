extern crate mygpoclient;

use std::env;
use url::Url;

use mygpoclient::client::DeviceClient;
use mygpoclient::directory::PodcastSearch;
use mygpoclient::directory::PodcastToplist;
use mygpoclient::directory::RetrieveEpisodeData;
use mygpoclient::directory::RetrievePodcastData;
use mygpoclient::directory::RetrievePodcastsForTag;
use mygpoclient::directory::RetrieveTopTags;
use mygpoclient::error::Error;

#[test]
fn test_retrieve_top_tags_device_client() -> Result<(), Error> {
    let client = get_device_client();
    let max_results = 5;
    let tags = client.retrieve_top_tags(max_results)?;
    assert_eq!(max_results as usize, tags.len());

    Ok(())
}

#[test]
fn test_retrieve_podcasts_for_tag_device_client() -> Result<(), Error> {
    let client = get_device_client();
    let max_results = 5;
    let podcasts = client.retrieve_podcasts_for_tag("new", max_results)?;
    assert!(podcasts.len() <= max_results as usize);

    Ok(())
}

#[test]
fn test_retrieve_podcast_data_device_client() -> Result<(), Error> {
    let client = get_device_client();
    let url = Url::parse("http://feeds.feedburner.com/coverville").unwrap();
    client.retrieve_podcast_data(url)?;

    Ok(())
}

#[test]
fn test_retrieve_episode_data_device_client() -> Result<(), Error> {
    let client = get_device_client();
    let url = Url::parse(
        "https://www.podtrac.com/pts/redirect.mp3/audio.wnyc.org/otm/otm011520_podextra.mp3",
    )
    .unwrap();
    let podcast = Url::parse("http://feeds.wnyc.org/onthemedia?format=xml").unwrap();
    client.retrieve_episode_data(url, podcast)?;

    Ok(())
}

#[test]
fn test_podcast_toplist_device_client() -> Result<(), Error> {
    let client = get_device_client();
    let max_results = 5;
    let podcasts = client.podcast_toplist(max_results, Some(256))?;
    assert!(podcasts.len() <= max_results as usize);

    Ok(())
}

#[test]
fn test_podcast_search_device_client() -> Result<(), Error> {
    let client = get_device_client();
    let podcasts = client.podcast_search("raumzeit", Some(256))?;
    assert!(podcasts.len() > 0);

    Ok(())
}

fn get_device_client() -> DeviceClient {
    let username = env::var("GPODDER_NET_USERNAME").unwrap();
    let password = env::var("GPODDER_NET_PASSWORD").unwrap();
    let deviceid = env::var("GPODDER_NET_DEVICEID").unwrap();

    DeviceClient::new(&username, &password, &deviceid)
}
