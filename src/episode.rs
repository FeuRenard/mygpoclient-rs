use crate::AuthenticatedClient;
use crate::Error;
use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

/// Episode Action Types as used in [EpisodeAction]s
///
/// [gpodder.net API Documentation]: https://gpoddernet.readthedocs.io/en/latest/api/reference/events.html#episode-action-types
#[serde(rename_all = "lowercase", tag = "action")]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum EpisodeActionType {
    Download,
    Delete,
    Play {
        position: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        started: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        total: Option<u32>,
    },
    New,
    Flattr,
}

/// Represents an episode-related event
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct EpisodeAction {
    pub podcast: String,
    pub episode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    #[serde(flatten)]
    pub action: EpisodeActionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,
}

// TODO see UploadSubscriptionChangesResponse
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct UploadEpisodeActionsResponse {
    pub timestamp: u64,
    pub update_urls: Vec<(String, String)>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct GetEpisodeActionsResponse {
    pub actions: Vec<EpisodeAction>,
    pub timestamp: u64,
}

/// Episode Actions API
///
/// The episode actions API is used to synchronize episode-related events between individual devices. Clients can send and store events on the webservice which makes it available to other clients. The following types of actions are currently accepted by the API: download, play, delete, new. Additional types can be requested on the Mailing List.
///
/// Example use cases
/// - Clients can send download and delete events so that other clients know where a file has already been downloaded.
/// - Clients can send play events with position information so that other clients know where to start playback.
/// - Clients can send new states to reset previous events. This state needs to be interpreted by receiving clients and does not delete any information on the webservice.
///
/// [gpodder.net API Documentation]: https://gpoddernet.readthedocs.io/en/latest/api/reference/events.html
pub trait EpisodeActions: UploadEpisodeActions + GetEpisodeActions {}

pub trait UploadEpisodeActions {
    /// Upload Episode Actions
    ///
    /// Upload changed episode actions. As actions are saved on a per-user basis (not per-device), the API endpoint is the same for every device. For logging purposes, the client can send the device ID to the server, so it appears in the episode action log on the website.
    ///
    /// [gpodder.net API Documentation]: https://gpoddernet.readthedocs.io/en/latest/api/reference/events.html#upload-episode-actions
    ///
    /// # Examples
    ///
    /// ```
    /// use mygpoclient::AuthenticatedClient;
    /// use mygpoclient::episode::EpisodeAction;
    /// use mygpoclient::episode::UploadEpisodeActions;
    /// use chrono::prelude::*;
    ///
    /// # let username = std::env::var("GPODDER_NET_USERNAME").unwrap();
    /// # let password = std::env::var("GPODDER_NET_PASSWORD").unwrap();
    /// #
    /// let client = AuthenticatedClient::new(&username, &password);
    ///
    /// let episode_action_1 = EpisodeAction::new_download("http://example.com/feed.rss".to_owned(), "http://example.com/files/s01e20.mp3".to_owned(), Some(Utc.ymd(2009,12,12).and_hms(9,0,0)));
    /// let episode_action_2 = EpisodeAction::new_play("http://example.org/podcast.php".to_owned(), "http://ftp.example.org/foo.ogg".to_owned(), None,120,15,500);
    /// let episode_actions = vec!(episode_action_1, episode_action_2);
    ///
    /// let response = client.upload_episode_actions(&episode_actions)?;
    /// #
    /// # Ok::<(), mygpoclient::Error>(())
    /// ```
    fn upload_episode_actions(
        &self,
        actions: &[EpisodeAction],
    ) -> Result<UploadEpisodeActionsResponse, Error>;
}

pub trait GetEpisodeActions {
    fn get_episode_actions(
        &self,
        podcast: Option<&str>,
        since: Option<u64>,
        aggregated: bool,
    ) -> Result<GetEpisodeActionsResponse, Error>;
}

impl EpisodeAction {
    fn new(
        podcast: String,
        episode: String,
        timestamp: Option<DateTime<Utc>>,
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

    /// Create new [download] event, so that other clients know where a file has already been downloaded.
    pub fn new_download(
        podcast: String,
        episode: String,
        timestamp: Option<DateTime<Utc>>,
    ) -> EpisodeAction {
        Self::new(podcast, episode, timestamp, EpisodeActionType::Download)
    }

    /// Create new [delete] event so that other clients know where a previously downloaded file has been deleted.
    pub fn new_delete(
        podcast: String,
        episode: String,
        timestamp: Option<DateTime<Utc>>,
    ) -> EpisodeAction {
        Self::new(podcast, episode, timestamp, EpisodeActionType::Delete)
    }

    /// Create new [new] event, to reset previous events. This state needs to be interpreted by receiving clients and does not delete any information on the webservice.
    pub fn new_new(
        podcast: String,
        episode: String,
        timestamp: Option<DateTime<Utc>>,
    ) -> EpisodeAction {
        Self::new(podcast, episode, timestamp, EpisodeActionType::New)
    }

    /// Create new [play] event with [position] information (in seconds) so that other clients know where to start playback.
    pub fn new_play_stop(
        podcast: String,
        episode: String,
        timestamp: Option<DateTime<Utc>>,
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

    /// Create new [play] event with [position], [started] and [total] information (in seconds) so that other clients know where to start playback.
    pub fn new_play(
        podcast: String,
        episode: String,
        timestamp: Option<DateTime<Utc>>,
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
        podcast: Option<&str>,
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
        let query_paramter_since: (&str, &str) = ("since", since_string.as_ref());

        if !since_string.is_empty() {
            query_parameters.push(&query_paramter_since);
        }

        let podcast_string = match podcast {
            Some(p) => p.to_string(),
            None => String::new(),
        };
        let query_paramter_podcast: (&str, &str) = ("podcast", podcast_string.as_ref());

        if !podcast_string.is_empty() {
            query_parameters.push(&query_paramter_podcast);
        }

        Ok(self
            .get_with_query(
                &format!("https://gpodder.net/api/2/episodes/{}.json", self.username),
                &query_parameters,
            )?
            .json()?)
    }
}

impl EpisodeActions for AuthenticatedClient {}
