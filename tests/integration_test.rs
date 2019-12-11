extern crate libmygpo_rs;

use std::env;

use libmygpo_rs::subscription;
use libmygpo_rs::Error;

const DEVICEID: &'static str = "randomdeviceid"; // TODO
const DUMMY_PODCAST_URL: &'static str = "http://ubuntupodcast.org/feed/";

#[test]
fn test_subscription() -> Result<(), Error> {
    let username = env::var("GPODDER_NET_USERNAME").unwrap();
    let password = env::var("GPODDER_NET_PASSWORD").unwrap();

    let subscriptions = subscription::get(username.as_str(), password.as_str(), DEVICEID)?;

    if subscriptions.contains(&get_dummy_url()) {
        add_and_assert_contains(
            remove_and_assert_gone(subscriptions, username.as_str(), password.as_str())?,
            username.as_str(),
            password.as_str(),
        )?;
    } else {
        remove_and_assert_gone(
            add_and_assert_contains(subscriptions, username.as_str(), password.as_str())?,
            username.as_str(),
            password.as_str(),
        )?;
    }

    Ok(())
}

fn add_and_assert_contains(
    mut subscriptions: Vec<String>,
    username: &str,
    password: &str,
) -> Result<Vec<String>, Error> {
    subscriptions.push(get_dummy_url());
    subscription::put(&subscriptions, username, password, DEVICEID)?;

    let subscriptions_after_addition = subscription::get(username, password, DEVICEID)?;
    assert!(subscriptions_after_addition.contains(&get_dummy_url()));

    assert_eq!(
        1,
        subscription::Subscription::get_all(username, password)?
            .iter()
            .filter(|s| s.url == get_dummy_url())
            .count()
    );

    Ok(subscriptions_after_addition)
}

fn remove_and_assert_gone(
    subscriptions: Vec<String>,
    username: &str,
    password: &str,
) -> Result<Vec<String>, Error> {
    subscription::put(
        subscriptions
            .iter()
            .filter(|&s| s != DUMMY_PODCAST_URL)
            .cloned()
            .collect::<Vec<String>>()
            .as_ref(),
        username,
        password,
        DEVICEID,
    )?;

    let subscriptions_after_removal = subscription::get(username, password, DEVICEID)?;
    assert!(!subscriptions_after_removal.contains(&get_dummy_url()));
    Ok(subscriptions_after_removal)
}

fn get_dummy_url() -> String {
    String::from(DUMMY_PODCAST_URL)
}
