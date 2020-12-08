use rspotify::client::Spotify;
use rspotify::model::user::PrivateUser;
use anyhow::Error;
use async_trait::async_trait;

#[async_trait]
pub trait StreamingProvider<T> {
    async fn gather_data(&self) -> Result<T, Error>;
    fn convert_to_query(&self, item: T) -> Vec<(String, String)>;
    async fn build_queries(&self) -> Result<Vec<(String, String)>, Error>;
}