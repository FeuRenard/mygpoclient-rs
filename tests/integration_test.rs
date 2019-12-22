extern crate libmygpo_rs;

use std::env;

use libmygpo_rs::Client;
use libmygpo_rs::Error;

const DEVICEID: &'static str = "randomdeviceid"; // TODO
const DUMMY_PODCAST_URL: &'static str = "http://ubuntupodcast.org/feed/";

#[test]
fn test_subscription() -> Result<(), Error> {
    let username = env::var("GPODDER_NET_USERNAME").unwrap();
    let password = env::var("GPODDER_NET_PASSWORD").unwrap();

    let client = Client::new(&username, &password);

    let subscriptions = client.get_subscriptions(DEVICEID)?;

    if subscriptions.contains(&get_dummy_url()) {
        add_and_assert_contains(remove_and_assert_gone(subscriptions, &client)?, &client)?;
    } else {
        remove_and_assert_gone(add_and_assert_contains(subscriptions, &client)?, &client)?;
    }

    Ok(())
}

fn add_and_assert_contains(
    mut subscriptions: Vec<String>,
    client: &Client,
) -> Result<Vec<String>, Error> {
    subscriptions.push(get_dummy_url());
    client.put_subscriptions(&subscriptions, DEVICEID)?;

    let subscriptions_after_addition = client.get_subscriptions(DEVICEID)?;
    assert!(subscriptions_after_addition.contains(&get_dummy_url()));

    assert_eq!(
        1,
        client
            .get_all_subscriptions()?
            .iter()
            .filter(|s| s.url == get_dummy_url())
            .count()
    );

    Ok(subscriptions_after_addition)
}

fn remove_and_assert_gone(
    subscriptions: Vec<String>,
    client: &Client,
) -> Result<Vec<String>, Error> {
    client.put_subscriptions(
        subscriptions
            .iter()
            .filter(|&s| s != DUMMY_PODCAST_URL)
            .cloned()
            .collect::<Vec<String>>()
            .as_ref(),
        DEVICEID,
    )?;

    let subscriptions_after_removal = client.get_subscriptions(DEVICEID)?;
    assert!(!subscriptions_after_removal.contains(&get_dummy_url()));
    Ok(subscriptions_after_removal)
}

fn get_dummy_url() -> String {
    DUMMY_PODCAST_URL.to_owned()
}

#[test]
fn test_suggestion() -> Result<(), Error> {
    let username = env::var("GPODDER_NET_USERNAME").unwrap();
    let password = env::var("GPODDER_NET_PASSWORD").unwrap();

    let client = Client::new(&username, &password);

    let max_number_of_suggestions = 3;
    let suggestions = client.get_suggestions(max_number_of_suggestions)?;

    assert!(suggestions.len() <= max_number_of_suggestions as usize);

    Ok(())
}

#[test]
fn test_subscription_changes() -> Result<(), Error> {
    let username = env::var("GPODDER_NET_USERNAME").unwrap();
    let password = env::var("GPODDER_NET_PASSWORD").unwrap();

    let client = Client::new(&username, &password);

    let subscriptions = client.get_subscriptions(DEVICEID)?;

    let is_remove_first = subscriptions.contains(&get_dummy_url());
    let last_timestamp = if is_remove_first {
        remove_changes(&client)?;
        add_changes(&client)?
    } else {
        add_changes(&client)?;
        remove_changes(&client)?
    };

    let changes = client.get_subscription_changes(DEVICEID, last_timestamp)?;

    let add_or_remove_empty = if is_remove_first {
        &changes.remove
    } else {
        &changes.add
    };
    let add_or_remove_one = if is_remove_first {
        &changes.add
    } else {
        &changes.remove
    };

    assert!(add_or_remove_empty.is_empty());
    assert_eq!(
        1,
        add_or_remove_one
            .iter()
            .filter(|&url| *url == get_dummy_url())
            .count()
    );

    Ok(())
}

fn add_changes(client: &Client) -> Result<u64, Error> {
    let dummy_podcast_url_with_spaces = format!("{}  ", DUMMY_PODCAST_URL);
    let add = vec![dummy_podcast_url_with_spaces.clone()];
    let remove = vec![];

    let response = client.upload_subscription_changes(&add, &remove, DEVICEID)?;

    assert_eq!(
        1,
        response
            .update_urls
            .iter()
            .filter(|&update_url| *update_url
                == (dummy_podcast_url_with_spaces.clone(), get_dummy_url()))
            .count()
    );

    Ok(response.timestamp)
}

fn remove_changes(client: &Client) -> Result<u64, Error> {
    let add = vec![];
    let remove = vec![get_dummy_url()];

    let response = client.upload_subscription_changes(&add, &remove, DEVICEID)?;

    assert!(response.update_urls.is_empty());

    Ok(response.timestamp)
}
