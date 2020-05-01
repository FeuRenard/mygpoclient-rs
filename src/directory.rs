//! [Directory API](https://gpoddernet.readthedocs.io/en/latest/api/reference/directory.html)

use crate::client::{AuthenticatedClient, DeviceClient, PublicClient};
use crate::error::Error;
use crate::subscription::Subscription;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use url::form_urlencoded::byte_serialize;

/// Podcast tags
#[derive(Deserialize, Serialize, Debug, Clone, Eq)]
pub struct Tag {
    /// more reader-friendly representation of tag
    pub title: String,
    /// actual tag, unique identifier
    pub tag: String,
    /// number of podcasts using this tag
    pub usage: u16,
}

/// see [`retrieve_top_tags`](./trait.RetrieveTopTags.html#tymethod.retrieve_top_tags)
pub trait RetrieveTopTags {
    /// Retrieve Top Tags
    ///
    /// # Parameters
    ///
    /// - `count`: number of tags to return
    ///
    /// # Examples
    ///
    /// ```
    /// use mygpoclient::client::PublicClient;
    /// use mygpoclient::directory::RetrieveTopTags;
    ///
    /// let tags = PublicClient::default().retrieve_top_tags(10)?;
    /// assert_eq!(10, tags.len());
    ///
    /// # Ok::<(), mygpoclient::error::Error>(())
    /// ```
    ///
    /// # See also
    ///
    /// - [gpodder.net API Documentation](https://gpoddernet.readthedocs.io/en/latest/api/reference/directory.html#retrieve-top-tags)
    fn retrieve_top_tags(&self, count: u8) -> Result<Vec<Tag>, Error>;
}

/// see [`retrieve_podcasts_for_tag`](./trait.RetrievePodcastsForTag.html#tymethod.retrieve_podcasts_for_tag)
pub trait RetrievePodcastsForTag {
    /// Retrieve Podcasts for Tag
    ///
    /// # Parameters
    ///
    /// - `tag`: podcast tag
    /// - `count`: number of tags to return
    ///
    /// # Examples
    ///
    /// ```
    /// use mygpoclient::client::PublicClient;
    /// use mygpoclient::directory::RetrievePodcastsForTag;
    ///
    /// let max_results = 3;
    /// let podcasts = PublicClient::default().retrieve_podcasts_for_tag("new", max_results)?;
    /// assert!(podcasts.len() <= max_results as usize);
    ///
    /// # Ok::<(), mygpoclient::error::Error>(())
    /// ```
    ///
    /// # See also
    ///
    /// - [gpodder.net API Documentation](https://gpoddernet.readthedocs.io/en/latest/api/reference/directory.html#retrieve-podcasts-for-tag)
    fn retrieve_podcasts_for_tag(&self, tag: &str, count: u8) -> Result<Vec<Subscription>, Error>;
}

impl RetrieveTopTags for PublicClient {
    fn retrieve_top_tags(&self, count: u8) -> Result<Vec<Tag>, Error> {
        Ok(self
            .get(&format!(
                "https://gpodder.net/api/2/tags/{}.json",
                count.to_string()
            ))?
            .json()?)
    }
}

impl RetrieveTopTags for AuthenticatedClient {
    fn retrieve_top_tags(&self, count: u8) -> Result<Vec<Tag>, Error> {
        self.public_client.retrieve_top_tags(count)
    }
}

impl RetrieveTopTags for DeviceClient {
    fn retrieve_top_tags(&self, count: u8) -> Result<Vec<Tag>, Error> {
        self.authenticated_client.retrieve_top_tags(count)
    }
}

impl RetrievePodcastsForTag for PublicClient {
    fn retrieve_podcasts_for_tag(&self, tag: &str, count: u8) -> Result<Vec<Subscription>, Error> {
        let tag_urlencoded: String = byte_serialize(tag.as_bytes()).collect();
        Ok(self
            .get(&format!(
                "https://gpodder.net/api/2/tag/{}/{}.json",
                tag_urlencoded,
                count.to_string()
            ))?
            .json()?)
    }
}

impl RetrievePodcastsForTag for AuthenticatedClient {
    fn retrieve_podcasts_for_tag(&self, tag: &str, count: u8) -> Result<Vec<Subscription>, Error> {
        self.public_client.retrieve_podcasts_for_tag(tag, count)
    }
}

impl RetrievePodcastsForTag for DeviceClient {
    fn retrieve_podcasts_for_tag(&self, tag: &str, count: u8) -> Result<Vec<Subscription>, Error> {
        self.authenticated_client
            .retrieve_podcasts_for_tag(tag, count)
    }
}

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        self.tag == other.tag
    }
}

impl Ord for Tag {
    fn cmp(&self, other: &Self) -> Ordering {
        self.tag.cmp(&other.tag)
    }
}

impl PartialOrd for Tag {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for Tag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.tag.hash(state);
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.tag, self.title)
    }
}

#[cfg(test)]
mod tests {
    use super::Tag;
    use std::cmp::Ordering;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    #[test]
    fn equal_device_means_equal_hash() {
        let tag1 = Tag {
            title: String::from("TAG"),
            tag: String::from("tag"),
            usage: 0,
        };
        let tag2 = Tag {
            title: String::from("GAT"),
            tag: String::from("tag"),
            usage: 100,
        };

        assert_eq!(tag1, tag2);
        assert_eq!(tag1.partial_cmp(&tag2), Some(Ordering::Equal));

        let mut hasher1 = DefaultHasher::new();
        tag1.hash(&mut hasher1);

        let mut hasher2 = DefaultHasher::new();
        tag2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn not_equal_devices_have_non_equal_ordering() {
        let tag1 = Tag {
            title: String::from("TAG"),
            tag: String::from("abc"),
            usage: 0,
        };
        let tag2 = Tag {
            title: String::from("TAG"),
            tag: String::from("xyz"),
            usage: 0,
        };

        assert_ne!(tag1, tag2);
        assert_eq!(tag1.partial_cmp(&tag2), Some(Ordering::Less));

        let mut hasher1 = DefaultHasher::new();
        tag1.hash(&mut hasher1);

        let mut hasher2 = DefaultHasher::new();
        tag2.hash(&mut hasher2);

        assert_ne!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn display() {
        let tag = Tag {
            title: String::from("TAG"),
            tag: String::from("xyz"),
            usage: 0,
        };

        assert_eq!("xyz: TAG".to_owned(), format!("{}", tag));
    }
}
