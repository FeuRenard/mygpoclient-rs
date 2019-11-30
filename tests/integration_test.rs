extern crate libmygpo_rs;

use libmygpo_rs::subscription;
use libmygpo_rs::Error;

const USERNAME: &'static str = "<insert username>";
const PASSWORD: &'static str = "<insert password>";
const DEVICEID: &'static str = "randomdeviceid"; // TODO
const DUMMY_PODCAST_URL: &'static str = "http://ubuntupodcast.org/feed/";

#[test]
fn test_subscription() -> Result<(), Error> {
    let subscriptions = subscription::get(USERNAME, PASSWORD, DEVICEID)?;

    if subscriptions.contains(&get_dummy_url()) {
        add_and_assert_contains(remove_and_assert_gone(subscriptions)?)?;
    } else {
        remove_and_assert_gone(add_and_assert_contains(subscriptions)?)?;
    }

    Ok(())
}

fn add_and_assert_contains(mut subscriptions: Vec<String>) -> Result<Vec<String>, Error> {
    subscriptions.push(get_dummy_url());
    subscription::put(&subscriptions, USERNAME, PASSWORD, DEVICEID)?;

    let subscriptions_after_addition = subscription::get(USERNAME, PASSWORD, DEVICEID)?;
    assert!(subscriptions_after_addition.contains(&get_dummy_url()));

    assert_eq!(
        1,
        subscription::Subscription::get_all(USERNAME, PASSWORD)?
            .iter()
            .filter(|s| s.url == get_dummy_url())
            .count()
    );

    Ok(subscriptions_after_addition)
}

fn remove_and_assert_gone(subscriptions: Vec<String>) -> Result<Vec<String>, Error> {
    subscription::put(
        subscriptions
            .iter()
            .filter(|&s| s != DUMMY_PODCAST_URL)
            .cloned()
            .collect::<Vec<String>>()
            .as_ref(),
        USERNAME,
        PASSWORD,
        DEVICEID,
    )?;

    let subscriptions_after_removal = subscription::get(USERNAME, PASSWORD, DEVICEID)?;
    assert!(!subscriptions_after_removal.contains(&get_dummy_url()));
    Ok(subscriptions_after_removal)
}

fn get_dummy_url() -> String {
    String::from(DUMMY_PODCAST_URL)
}
