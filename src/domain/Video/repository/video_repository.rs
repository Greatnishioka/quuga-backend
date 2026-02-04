use async_trait::async_trait;
use anyhow::Result;
use crate::domain::video::entity::video_entity::{Video, VideoId};

#[async_trait]
pub trait VideoRepository: Send + Sync {
    async fn find_by_id(&self, id: &VideoId) -> Result<Option<Video>>;
}
