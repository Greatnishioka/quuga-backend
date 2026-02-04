use std::sync::RwLock;

use async_trait::async_trait;
use anyhow::Result;

use crate::domain::video::repository::video_repository::VideoRepository;
use crate::domain::video::entity::video_entity::{Video, VideoId};

pub struct InMemoryRepo {
    videos: RwLock<Vec<Video>>,
}

impl InMemoryRepo {
    pub fn new() -> Self {
        Self {
            videos: RwLock::new(Vec::new()),
        }
    }
}

#[async_trait]
impl VideoRepository for InMemoryRepo {
    async fn find_by_id(&self, id: &VideoId) -> Result<Option<Video>> {
        let videos = self.videos.read().expect("in_memory_repo read lock poisoned");
        Ok(videos.iter().find(|v| &v.id == id).cloned())
    }
}
