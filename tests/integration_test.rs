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
