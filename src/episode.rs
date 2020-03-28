//! [Episode Actions API](https://gpoddernet.readthedocs.io/en/latest/api/reference/events.html)
//!
//! The episode actions API is used to synchronize episode-related events between individual devices. Clients can send and store events on the webservice which makes it available to other clients. The following types of actions are currently accepted by the API: download, play, delete, new. Additional types can be requested on the Mailing List.
//!
//! Example use cases
//! - Clients can send download and delete events so that other clients know where a file has already been downloaded.
//! - Clients can send play events with position information so that other clients know where to start playback.
//! - Clients can send new states to reset previous events. This state needs to be interpreted by receiving clients and does not delete any information on the webservice.

use crate::client::AuthenticatedClient;
use crate::error::Error;
use chrono::naive::NaiveDateTime;
use serde::Deserialize;
use serde::Serialize;
use url::Url;

/// Type of an [EpisodeAction](./struct.EpisodeAction.html)
///
/// [gpodder.net API Documentation]: https://gpoddernet.readthedocs.io/en/latest/api/reference/events.html#episode-action-types
#[serde(rename_all = "lowercase", tag = "action")]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum EpisodeActionType {
    /// download event, so that other clients know where a file has already been downloaded
    Download,
    /// delete event, so that other clients know where a file has already been deleted
    Delete,
    /// play event, so that other clients know where to start playback
    Play {
        /// the position (in seconds) at which the client stopped playback
        position: u32,
        /// the position (in seconds) at which the client started playback. Requires total to be set.
        #[serde(skip_serializing_if = "Option::is_none")]
        started: Option<u32>,
        /// the total length of the file in seconds. Requires started to be set.
        #[serde(skip_serializing_if = "Option::is_none")]
        total: Option<u32>,
    },
    /// Resets previous events. This state needs to be interpreted by receiving clients and does not delete any information on the webservice.
    New,
    /// (undocumented)
    Flattr,
}

/// Episode-related event
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct EpisodeAction {
    /// feed URL to the podcast feed the episode belongs to
    pub podcast: Url,
    /// media URL of the episode
    pub episode: Url,
    /// device ID on which the action has taken place
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    /// see [EpisodeActionType](./enum.EpisodeActionType.html)
    #[serde(flatten)]
    pub action: EpisodeActionType,
    /// UTC timestamp when the action took place
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<NaiveDateTime>,
}

// TODO see UploadSubscriptionChangesResponse
/// Response to [`upload_episode_actions`](./trait.UploadEpisodeActions.html#tymethod.upload_episode_actions)
///
/// [gpodder.net API Documentation]: https://gpoddernet.readthedocs.io/en/latest/api/reference/events.html#upload-episode-actions
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct UploadEpisodeActionsResponse {
    /// the current timestamp; for retrieving changes since the last query
    pub timestamp: u64,
    /// list of URLs that have been rewritten (sanitized, see bug:747 and bug:862) as a list of tuples. The client SHOULD parse this list and update the local subscription and episode list accordingly (the server only sanitizes the URL, so the semantic “content” should stay the same and therefore the client can simply update the URL value locally and use it for future updates.
    ///
    /// URLs that are not allowed (currently all URLs that contain non-ASCII characters or don’t start with either http or https) are rewritten to the empty string and are ignored by the Webservice.
    pub update_urls: Vec<(Url, Url)>,
}

/// Response to [`get_episode_actions`](./trait.GetEpisodeActions.html#tymethod.get_episode_actions)
///
/// [gpodder.net API Documentation]: https://gpoddernet.readthedocs.io/en/latest/api/reference/events.html#get-episode-actions
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct GetEpisodeActionsResponse {
    /// see [EpisodeAction](./struct.EpisodeAction.html)
    pub actions: Vec<EpisodeAction>,
    /// new timestamp that the client SHOULD save and use for subsequent requests
    pub timestamp: u64,
}

/// see [`upload_episode_actions`](./trait.UploadEpisodeActions.html#tymethod.upload_episode_actions)
pub trait UploadEpisodeActions {
    /// Upload changed episode actions.
    ///
    /// As actions are saved on a per-user basis (not per-device), the API endpoint is the same for every device. For logging purposes, the client can send the device ID to the server, so it appears in the episode action log on the website.
    ///
    /// [gpodder.net API Documentation]: https://gpoddernet.readthedocs.io/en/latest/api/reference/events.html#upload-episode-actions
    ///
    /// # Examples
    ///
    /// ```
    /// use mygpoclient::client::AuthenticatedClient;
    /// use mygpoclient::episode::EpisodeAction;
    /// use mygpoclient::episode::UploadEpisodeActions;
    /// use chrono::prelude::*;
    /// use url::Url;
    ///
    /// # let username = std::env::var("GPODDER_NET_USERNAME").unwrap();
    /// # let password = std::env::var("GPODDER_NET_PASSWORD").unwrap();
    /// #
    /// let client = AuthenticatedClient::new(&username, &password);
    ///
    /// let download = EpisodeAction::new_download(Url::parse("http://example.com/feed1.rss").unwrap(), Url::parse("http://example.com/files/s01e20.mp3").unwrap(), Some(NaiveDate::from_ymd(2009,12,12).and_hms(9,0,0)));
    /// let play = EpisodeAction::new_play(Url::parse("http://example.org/podcast2.php").unwrap(), Url::parse("http://ftp.example.org/foo2.ogg").unwrap(), None,120,15,500);
    /// let delete = EpisodeAction::new_delete(Url::parse("http://example.com/feed3.rss").unwrap(), Url::parse("http://example.com/files/s03e20.mp3").unwrap(), None);
    /// let new = EpisodeAction::new_new(Url::parse("http://example.com/feed4.rss").unwrap(), Url::parse("http://example.com/files/s04e20.mp3").unwrap(), None);
    /// let play_stop = EpisodeAction::new_play_stop(Url::parse("http://example.org/podcast5.php").unwrap(), Url::parse("http://ftp.example.org/foo5.ogg").unwrap(), None, 120);
    /// let episode_actions = vec!(download, play, delete, new, play_stop);
    ///
    /// let response = client.upload_episode_actions(&episode_actions)?;
    /// #
    /// # Ok::<(), mygpoclient::error::Error>(())
    /// ```
    fn upload_episode_actions(
        &self,
        actions: &[EpisodeAction],
    ) -> Result<UploadEpisodeActionsResponse, Error>;
}

// TODO use Date(time?) instead of timestamps as integers
/// see [`get_episode_actions`](./trait.GetEpisodeActions.html#tymethod.get_episode_actions)
pub trait GetEpisodeActions {
    /// Get changed episode actions
    ///
    /// Timestamps: The result is a list of all episode actions that were uploaded since the timestamp given in the since parameter (regardless of the action timestamp itself). The timestamp SHOULD be the value returned by the previous episode retrieve request. If no since value is given, ALL episode actions for the given user are returned. Please note that this could be a potentially long list of episode actions, so clients SHOULD provide a since value whenever possible (e.g. when uploads have been taken place before).
    ///
    /// [gpodder.net API Documentation]: https://gpoddernet.readthedocs.io/en/latest/api/reference/events.html#get-episode-actions
    ///
    /// # Parameters
    ///
    /// - `podcast`: The URL of a Podcast feed; if set, only actions for episodes of the given podcast are returned
    /// - `since`: Only episode actions since the given timestamp are returned
    /// - `aggregated`: If true, only the latest actions is returned for each episode
    ///
    /// # Examples
    ///
    /// ```
    /// use mygpoclient::client::AuthenticatedClient;
    /// use mygpoclient::episode::GetEpisodeActions;
    /// use chrono::prelude::*;
    /// use url::Url;
    ///
    /// # let username = std::env::var("GPODDER_NET_USERNAME").unwrap();
    /// # let password = std::env::var("GPODDER_NET_PASSWORD").unwrap();
    /// #
    /// let client = AuthenticatedClient::new(&username, &password);
    ///
    /// let response = client.get_episode_actions(Some(Url::parse("http://example.com/feed.rss").unwrap()), None, false)?;
    /// #
    /// # Ok::<(), mygpoclient::error::Error>(())
    /// ```
    fn get_episode_actions(
        &self,
        podcast: Option<Url>,
        since: Option<u64>,
        aggregated: bool,
    ) -> Result<GetEpisodeActionsResponse, Error>;
}

impl EpisodeAction {
    fn new(
        podcast: Url,
        episode: Url,
        timestamp: Option<NaiveDateTime>,
        action: EpisodeActionType,
    ) -> EpisodeAction {
        EpisodeAction {
            podcast,
            episode,
            device: None,
            action,
            timestamp,
        }
    }

    /// Create new [`Download`](./enum.EpisodeActionType.html#variant.Download) event, so that other clients know where a file has already been downloaded.
    pub fn new_download(
        podcast: Url,
        episode: Url,
        timestamp: Option<NaiveDateTime>,
    ) -> EpisodeAction {
        Self::new(podcast, episode, timestamp, EpisodeActionType::Download)
    }

    /// Create new [`Delete`](./enum.EpisodeActionType.html#variant.Delete) event so that other clients know where a previously downloaded file has been deleted.
    pub fn new_delete(
        podcast: Url,
        episode: Url,
        timestamp: Option<NaiveDateTime>,
    ) -> EpisodeAction {
        Self::new(podcast, episode, timestamp, EpisodeActionType::Delete)
    }

    /// Create new [`New`](./enum.EpisodeActionType.html#variant.New) event, to reset previous events. This state needs to be interpreted by receiving clients and does not delete any information on the webservice.
    pub fn new_new(podcast: Url, episode: Url, timestamp: Option<NaiveDateTime>) -> EpisodeAction {
        Self::new(podcast, episode, timestamp, EpisodeActionType::New)
    }

    /// Create new [`Play`](./enum.EpisodeActionType.html#variant.Play) event with [`position`](./enum.EpisodeActionType.html#variant.Play.field.position) information (in seconds) so that other clients know where to start playback.
    pub fn new_play_stop(
        podcast: Url,
        episode: Url,
        timestamp: Option<NaiveDateTime>,
        position: u32,
    ) -> EpisodeAction {
        EpisodeAction {
            podcast,
            episode,
            device: None,
            action: EpisodeActionType::Play {
                position,
                started: None,
                total: None,
            },
            timestamp,
        }
    }

    /// Create new [`Play`](./enum.EpisodeActionType.html#variant.Play) event with [`position`](./enum.EpisodeActionType.html#variant.Play.field.position), [`started`](./enum.EpisodeActionType.html#variant.Play.field.started) and [`total`](./enum.EpisodeActionType.html#variant.Play.field.total) information (in seconds) so that other clients know where to start playback.
    pub fn new_play(
        podcast: Url,
        episode: Url,
        timestamp: Option<NaiveDateTime>,
        position: u32,
        started: u32,
        total: u32,
    ) -> EpisodeAction {
        EpisodeAction {
            podcast,
            episode,
            device: None,
            action: EpisodeActionType::Play {
                position,
                started: Some(started),
                total: Some(total),
            },
            timestamp,
        }
    }
}

impl UploadEpisodeActions for AuthenticatedClient {
    fn upload_episode_actions(
        &self,
        actions: &[EpisodeAction],
    ) -> Result<UploadEpisodeActionsResponse, Error> {
        Ok(self
            .post(
                &format!("https://gpodder.net/api/2/episodes/{}.json", self.username),
                actions,
            )?
            .json()?)
    }
}

impl GetEpisodeActions for AuthenticatedClient {
    fn get_episode_actions(
        &self,
        podcast: Option<Url>,
        since: Option<u64>,
        aggregated: bool,
    ) -> Result<GetEpisodeActionsResponse, Error> {
        let mut query_parameters: Vec<&(&str, &str)> = Vec::new();

        let aggregated_string = aggregated.to_string();
        let query_parameter_aggregated = ("aggregated", aggregated_string.as_ref());
        query_parameters.push(&query_parameter_aggregated);

        let since_string = match since {
            Some(s) => s.to_string(),
            None => String::new(),
        };
        let query_parameter_since: (&str, &str) = ("since", since_string.as_ref());

        if !since_string.is_empty() {
            query_parameters.push(&query_parameter_since);
        }

        let podcast_string = match podcast {
            Some(p) => p.to_string(),
            None => String::new(),
        };
        let query_parameter_podcast: (&str, &str) = ("podcast", podcast_string.as_ref());

        if !podcast_string.is_empty() {
            query_parameters.push(&query_parameter_podcast);
        }

        Ok(self
            .get_with_query(
                &format!("https://gpodder.net/api/2/episodes/{}.json", self.username),
                &query_parameters,
            )?
            .json()?)
    }
}
