//! [Directory API](https://gpoddernet.readthedocs.io/en/latest/api/reference/directory.html)

use crate::client::{AuthenticatedClient, DeviceClient, PublicClient};
use crate::error::Error;
use crate::subscription::Podcast;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use url::form_urlencoded::byte_serialize;
use url::Url;

/// Podcast tag
#[derive(Deserialize, Serialize, Debug, Clone, Eq)]
pub struct Tag {
    /// more reader-friendly representation of tag
    pub title: String,
    /// actual tag, unique identifier
    pub tag: String,
    /// number of podcasts using this tag
    pub usage: u16,
}

/// Podcast episode
#[derive(Deserialize, Serialize, Debug, Clone, Eq)]
pub struct Episode {
    /// title
    pub title: String,
    /// media url
    pub url: Url,
    /// podcast title
    pub podcast_title: String,
    /// podcast feed url
    pub podcast_url: Url,
    /// description
    pub description: String,
    /// website
    pub website: Option<Url>,
    /// gpodder internal link
    pub mygpo_link: Url,
    /// release date
    pub released: NaiveDateTime,
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
    /// - `count`: number of podcasts to return
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
    fn retrieve_podcasts_for_tag(&self, tag: &str, count: u8) -> Result<Vec<Podcast>, Error>;
}

/// see [`retrieve_podcast_data`](./trait.RetrievePodcastData.html#tymethod.retrieve_podcast_data)
pub trait RetrievePodcastData {
    /// Returns information for the podcast with the given URL or Error if there is no podcast with this URL.
    ///
    /// # Parameters
    ///
    /// - `url`: podcast feed url
    ///
    /// # Examples
    ///
    /// ```
    /// use mygpoclient::client::PublicClient;
    /// use mygpoclient::directory::RetrievePodcastData;
    /// use url::Url;
    ///
    /// let url = Url::parse("http://feeds.feedburner.com/coverville").unwrap();
    /// let podcast = PublicClient::default().retrieve_podcast_data(url)?;
    ///
    /// # Ok::<(), mygpoclient::error::Error>(())
    /// ```
    ///
    /// # See also
    ///
    /// - [gpodder.net API Documentation](https://gpoddernet.readthedocs.io/en/latest/api/reference/directory.html#retrieve-podcast-data)
    fn retrieve_podcast_data(&self, url: Url) -> Result<Podcast, Error>;
}

/// see [`retrieve_episode_data`](./trait.RetrieveEpisodeData.html#tymethod.retrieve_episode_data)
pub trait RetrieveEpisodeData {
    /// Returns information for the episode with the given url that belongs to the given podcast
    ///
    /// # Parameters
    ///
    /// - `url`: media url of episode
    /// - `podcast`: podcast feed url
    ///
    /// # Examples
    ///
    /// ```
    /// use mygpoclient::client::PublicClient;
    /// use mygpoclient::directory::RetrieveEpisodeData;
    /// use url::Url;
    ///
    /// let url = Url::parse("https://www.podtrac.com/pts/redirect.mp3/audio.wnyc.org/otm/otm011520_podextra.mp3").unwrap();
    /// let podcast = Url::parse("http://feeds.wnyc.org/onthemedia?format=xml").unwrap();
    /// let episode = PublicClient::default().retrieve_episode_data(url, podcast)?;
    ///
    /// # Ok::<(), mygpoclient::error::Error>(())
    /// ```
    ///
    /// # See also
    ///
    /// - [gpodder.net API Documentation](https://gpoddernet.readthedocs.io/en/latest/api/reference/directory.html#retrieve-episode-data)
    fn retrieve_episode_data(&self, podcast: Url, url: Url) -> Result<Episode, Error>;
}

/// see [`podcast_toplist`](./trait.PodcastToplist.html#tymethod.podcast_toplist)
pub trait PodcastToplist {
    /// Returns list of top podcasts
    ///
    /// # Parameters
    ///
    /// - `number`: maximum number of podcasts to return
    /// - `scale_logo`: provides a link to a scaled logo for each podcast. Has to be a positive number up to 256 and defaults to 64.
    ///
    /// # Examples
    ///
    /// ```
    /// use mygpoclient::client::PublicClient;
    /// use mygpoclient::directory::PodcastToplist;
    ///
    /// let max_results = 10;
    /// let podcasts = PublicClient::default().podcast_toplist(max_results, None)?;
    /// assert_eq!(max_results as usize, podcasts.len());
    ///
    /// # Ok::<(), mygpoclient::error::Error>(())
    /// ```
    ///
    /// # See also
    ///
    /// - [gpodder.net API Documentation](https://gpoddernet.readthedocs.io/en/latest/api/reference/directory.html#podcast-toplist)
    fn podcast_toplist(&self, number: u8, scale_logo: Option<u16>) -> Result<Vec<Podcast>, Error>;
}

/// see [`podcast_search`](./trait.PodcastSearch.html#tymethod.podcast_search)
pub trait PodcastSearch {
    /// Carries out a service-wide search for podcasts that match the given query. Returns a list of podcasts.
    ///
    /// # Parameters
    ///
    /// - `q`: search query
    /// - `scale_logo`: provides a link to a scaled logo for each podcast. Has to be a positive number up to 256 and defaults to 64.
    ///
    /// # Examples
    ///
    /// ```
    /// use mygpoclient::client::PublicClient;
    /// use mygpoclient::directory::PodcastSearch;
    ///
    /// let podcasts = PublicClient::default().podcast_search("raumzeit", None)?;
    /// assert!(podcasts.len() > 0);
    ///
    /// # Ok::<(), mygpoclient::error::Error>(())
    /// ```
    ///
    /// # See also
    ///
    /// - [gpodder.net API Documentation](https://gpoddernet.readthedocs.io/en/latest/api/reference/directory.html#podcast-search)
    fn podcast_search(&self, q: &str, scale_logo: Option<u16>) -> Result<Vec<Podcast>, Error>;
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
    fn retrieve_podcasts_for_tag(&self, tag: &str, count: u8) -> Result<Vec<Podcast>, Error> {
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
    fn retrieve_podcasts_for_tag(&self, tag: &str, count: u8) -> Result<Vec<Podcast>, Error> {
        self.public_client.retrieve_podcasts_for_tag(tag, count)
    }
}

impl RetrievePodcastsForTag for DeviceClient {
    fn retrieve_podcasts_for_tag(&self, tag: &str, count: u8) -> Result<Vec<Podcast>, Error> {
        self.authenticated_client
            .retrieve_podcasts_for_tag(tag, count)
    }
}

impl RetrievePodcastData for PublicClient {
    fn retrieve_podcast_data(&self, url: Url) -> Result<Podcast, Error> {
        Ok(self
            .get_with_query(
                "https://gpodder.net/api/2/data/podcast.json",
                &[&("url", url.as_str())],
            )?
            .json()?)
    }
}

impl RetrievePodcastData for AuthenticatedClient {
    fn retrieve_podcast_data(&self, url: Url) -> Result<Podcast, Error> {
        self.public_client.retrieve_podcast_data(url)
    }
}

impl RetrievePodcastData for DeviceClient {
    fn retrieve_podcast_data(&self, url: Url) -> Result<Podcast, Error> {
        self.authenticated_client.retrieve_podcast_data(url)
    }
}

impl RetrieveEpisodeData for PublicClient {
    fn retrieve_episode_data(&self, url: Url, podcast: Url) -> Result<Episode, Error> {
        Ok(self
            .get_with_query(
                "https://gpodder.net/api/2/data/episode.json",
                &[&("url", url.as_str()), &("podcast", podcast.as_str())],
            )?
            .json()?)
    }
}

impl RetrieveEpisodeData for AuthenticatedClient {
    fn retrieve_episode_data(&self, url: Url, podcast: Url) -> Result<Episode, Error> {
        self.public_client.retrieve_episode_data(url, podcast)
    }
}

impl RetrieveEpisodeData for DeviceClient {
    fn retrieve_episode_data(&self, url: Url, podcast: Url) -> Result<Episode, Error> {
        self.authenticated_client
            .retrieve_episode_data(url, podcast)
    }
}

impl PodcastToplist for PublicClient {
    fn podcast_toplist(&self, number: u8, scale_logo: Option<u16>) -> Result<Vec<Podcast>, Error> {
        let url = &format!("https://gpodder.net/toplist/{}.json", number);

        if let Some(size) = scale_logo {
            Ok(self
                .get_with_query(url, &[&("scale_logo", size.to_string())])?
                .json()?)
        } else {
            Ok(self.get(url)?.json()?)
        }
    }
}

impl PodcastToplist for AuthenticatedClient {
    fn podcast_toplist(&self, number: u8, scale_logo: Option<u16>) -> Result<Vec<Podcast>, Error> {
        self.public_client.podcast_toplist(number, scale_logo)
    }
}

impl PodcastToplist for DeviceClient {
    fn podcast_toplist(&self, number: u8, scale_logo: Option<u16>) -> Result<Vec<Podcast>, Error> {
        self.authenticated_client
            .podcast_toplist(number, scale_logo)
    }
}

impl PodcastSearch for PublicClient {
    fn podcast_search(&self, q: &str, scale_logo: Option<u16>) -> Result<Vec<Podcast>, Error> {
        let mut query_parameters: Vec<&(&str, &str)> = Vec::new();

        let query_parameter_since = ("q", q);
        query_parameters.push(&query_parameter_since);

        let scale_logo_string = match scale_logo {
            Some(size) => size.to_string(),
            None => String::new(),
        };
        let query_parameter_scale_logo: (&str, &str) = ("scale_logo", scale_logo_string.as_ref());

        if !scale_logo_string.is_empty() {
            query_parameters.push(&query_parameter_scale_logo);
        }

        Ok(self
            .get_with_query("https://gpodder.net/search.json", &query_parameters)?
            .json()?)
    }
}

impl PodcastSearch for AuthenticatedClient {
    fn podcast_search(&self, q: &str, scale_logo: Option<u16>) -> Result<Vec<Podcast>, Error> {
        self.public_client.podcast_search(q, scale_logo)
    }
}

impl PodcastSearch for DeviceClient {
    fn podcast_search(&self, q: &str, scale_logo: Option<u16>) -> Result<Vec<Podcast>, Error> {
        self.authenticated_client.podcast_search(q, scale_logo)
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

impl PartialEq for Episode {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
    }
}

impl Ord for Episode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.url.cmp(&other.url)
    }
}

impl PartialOrd for Episode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for Episode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.url.hash(state);
    }
}

impl fmt::Display for Episode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.title, self.url)
    }
}

#[cfg(test)]
mod tests {
    use super::Episode;
    use super::Tag;
    use chrono::NaiveDate;
    use std::cmp::Ordering;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use url::Url;

    #[test]
    fn equal_tag_means_equal_hash() {
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
    fn not_equal_tags_have_non_equal_ordering() {
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
    fn display_tag() {
        let tag = Tag {
            title: String::from("TAG"),
            tag: String::from("xyz"),
            usage: 0,
        };

        assert_eq!("xyz: TAG".to_owned(), format!("{}", tag));
    }

    #[test]
    fn equal_episode_means_equal_hash() {
        let episode1 = Episode {
            title: String::from("TWiT 245: No Hitler For You"),
            url: Url::parse("http://www.podtrac.com/pts/redirect.mp3/aolradio.podcast.aol.com/twit/twit0245.mp3").unwrap(),
            podcast_title: String::from("this WEEK in TECH - MP3 Edition"),
            podcast_url: Url::parse("http://leo.am/podcasts/twit").unwrap(),
            description: String::from("[...]"),
            website: Some(Url::parse("http://www.podtrac.com/pts/redirect.mp3/aolradio.podcast.aol.com/twit/twit0245.mp3").unwrap()),
            mygpo_link: Url::parse("http://gpodder.net/episode/1046492").unwrap(),
            released: NaiveDate::from_ymd(2010, 12, 25).and_hms(0, 30, 0),
        };
        let episode2 = Episode {
            title: String::from("Climate Change, News Corp, and the Australian Fires"),
            url: Url::parse("http://www.podtrac.com/pts/redirect.mp3/aolradio.podcast.aol.com/twit/twit0245.mp3").unwrap(),
            podcast_title: String::from("On the Media"),
            podcast_url: Url::parse("http://feeds.wnyc.org/onthemedia?format=xml").unwrap(),
            description: String::from("[...]"),
            website: Some(Url::parse("http://www.wnycstudios.org/story/climate-change-news-corp-and-australian-fires/").unwrap()),
            mygpo_link: Url::parse("http://gpodder.net/podcast/on-the-media-1/climate-change-news-corp-and-the-australian-fires").unwrap(),
            released: NaiveDate::from_ymd(2020, 1, 15).and_hms(17, 0, 0),
        };

        assert_eq!(episode1, episode2);
        assert_eq!(episode1.partial_cmp(&episode2), Some(Ordering::Equal));

        let mut hasher1 = DefaultHasher::new();
        episode1.hash(&mut hasher1);

        let mut hasher2 = DefaultHasher::new();
        episode2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn not_equal_episodes_have_non_equal_ordering() {
        let episode1 = Episode {
            title: String::from("TWiT 245: No Hitler For You"),
            url: Url::parse("http://www.podtrac.com/pts/redirect.mp3/aolradio.podcast.aol.com/twit/twit0245.mp3").unwrap(),
            podcast_title: String::from("this WEEK in TECH - MP3 Edition"),
            podcast_url: Url::parse("http://leo.am/podcasts/twit").unwrap(),
            description: String::from("[...]"),
            website: Some(Url::parse("http://www.podtrac.com/pts/redirect.mp3/aolradio.podcast.aol.com/twit/twit0245.mp3").unwrap()),
            mygpo_link: Url::parse("http://gpodder.net/episode/1046492").unwrap(),
            released: NaiveDate::from_ymd(2010, 12, 25).and_hms(0, 30, 0),
        };
        let episode2 = Episode {
            title: String::from("Climate Change, News Corp, and the Australian Fires"),
            url: Url::parse("https://www.podtrac.com/pts/redirect.mp3/audio.wnyc.org/otm/otm011520_podextra.mp3").unwrap(),
            podcast_title: String::from("On the Media"),
            podcast_url: Url::parse("http://feeds.wnyc.org/onthemedia?format=xml").unwrap(),
            description: String::from("[...]"),
            website: Some(Url::parse("http://www.wnycstudios.org/story/climate-change-news-corp-and-australian-fires/").unwrap()),
            mygpo_link: Url::parse("http://gpodder.net/podcast/on-the-media-1/climate-change-news-corp-and-the-australian-fires").unwrap(),
            released: NaiveDate::from_ymd(2020, 1, 15).and_hms(17, 0, 0),
        };

        assert_ne!(episode1, episode2);
        assert_eq!(episode1.partial_cmp(&episode2), Some(Ordering::Less));

        let mut hasher1 = DefaultHasher::new();
        episode1.hash(&mut hasher1);

        let mut hasher2 = DefaultHasher::new();
        episode2.hash(&mut hasher2);

        assert_ne!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn display_episode() {
        let episode = Episode {
            title: String::from("TWiT 245: No Hitler For You"),
            url: Url::parse("http://www.podtrac.com/pts/redirect.mp3/aolradio.podcast.aol.com/twit/twit0245.mp3").unwrap(),
            podcast_title: String::from("this WEEK in TECH - MP3 Edition"),
            podcast_url: Url::parse("http://leo.am/podcasts/twit").unwrap(),
            description: String::from("[...]"),
            website: Some(Url::parse("http://www.podtrac.com/pts/redirect.mp3/aolradio.podcast.aol.com/twit/twit0245.mp3").unwrap()),
            mygpo_link: Url::parse("http://gpodder.net/episode/1046492").unwrap(),
            released: NaiveDate::from_ymd(2010, 12, 25).and_hms(0, 30, 0),
        };

        assert_eq!("TWiT 245: No Hitler For You: http://www.podtrac.com/pts/redirect.mp3/aolradio.podcast.aol.com/twit/twit0245.mp3".to_owned(), format!("{}", episode));
    }
}
