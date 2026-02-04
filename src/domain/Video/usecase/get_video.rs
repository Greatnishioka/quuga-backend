use std::sync::Arc;
use anyhow::{Result, anyhow};
use crate::domain::video::repository::video_repository::VideoRepository;
use crate::domain::video::entity::video_entity::{Video, VideoId};

pub struct GetVideoUseCase<R: VideoRepository> {
    repo: Arc<R>,
}

impl<R: VideoRepository> GetVideoUseCase<R> {
    pub fn new(repo: Arc<R>) -> Self { Self { repo } }

    pub async fn execute(&self, id: &VideoId) -> Result<Video> {
        match self.repo.find_by_id(id).await? {
            Some(v) => Ok(v),
            None => Err(anyhow!("video not found")),
        }
    }
}
