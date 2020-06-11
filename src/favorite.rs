//! [Favorites API](https://gpoddernet.readthedocs.io/en/latest/api/reference/favorites.html)

use crate::client::AuthenticatedClient;
use crate::client::DeviceClient;
use crate::directory::Episode;
use crate::error::Error;

/// see [`get_favorite_episodes`](./trait.GetFavoriteEpisodes.html#tymethod.get_favorite_episodes)
pub trait GetFavoriteEpisodes {
    /// Get Favorite Episodes
    ///
    /// # Examples
    ///
    /// ```
    /// use mygpoclient::client::AuthenticatedClient;
    /// use mygpoclient::favorite::GetFavoriteEpisodes;
    ///
    /// # let username = std::env::var("GPODDER_NET_USERNAME").unwrap();
    /// # let password = std::env::var("GPODDER_NET_PASSWORD").unwrap();
    /// #
    /// let client = AuthenticatedClient::new(&username, &password);
    ///
    /// client.get_favorite_episodes()?;
    /// # Ok::<(), mygpoclient::error::Error>(())
    /// ```
    ///
    /// # See also
    ///
    /// - [gpodder.net API Documentation](https://gpoddernet.readthedocs.io/en/latest/api/reference/favorites.html#get-favorite-episodes)
    fn get_favorite_episodes(&self) -> Result<Vec<Episode>, Error>;
}

impl GetFavoriteEpisodes for AuthenticatedClient {
    fn get_favorite_episodes(&self) -> Result<Vec<Episode>, Error> {
        Ok(self
            .get(&format!(
                "https://gpodder.net/api/2/favorites/{}.json",
                self.username
            ))?
            .json()?)
    }
}

impl GetFavoriteEpisodes for DeviceClient {
    fn get_favorite_episodes(&self) -> Result<Vec<Episode>, Error> {
        self.authenticated_client.get_favorite_episodes()
    }
}
