use super::rabbitmq::RabbitMqClient;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::info;

/// Background job queue for async processing
pub struct BackgroundJobQueue {
    client: RabbitMqClient,
    queue_name: String,
}

impl BackgroundJobQueue {
    /// Create a new background job queue
    pub async fn new(client: RabbitMqClient, queue_name: String) -> Result<Self> {
        // Declare the queue as durable
        client.declare_queue(&queue_name, true).await?;

        Ok(Self { client, queue_name })
    }

    /// Submit a background job
    pub async fn submit_job(&self, job: BackgroundJob) -> Result<String> {
        let job_id = uuid::Uuid::new_v4().to_string();
        let mut job_with_id = job;
        job_with_id.job_id = job_id.clone();

        let message = serde_json::to_vec(&job_with_id)?;
        self.client.publish_to_queue(&self.queue_name, &message).await?;

        info!("Submitted background job: {} (type: {:?})", job_id, job_with_id.job_type);
        Ok(job_id)
    }

    /// Submit multiple jobs in batch
    pub async fn submit_batch(&self, jobs: Vec<BackgroundJob>) -> Result<Vec<String>> {
        let mut job_ids = Vec::new();

        for job in jobs {
            let job_id = self.submit_job(job).await?;
            job_ids.push(job_id);
        }

        info!("Submitted {} background jobs", job_ids.len());
        Ok(job_ids)
    }

    /// Get the queue name
    pub fn queue_name(&self) -> &str {
        &self.queue_name
    }
}

/// Background job structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundJob {
    pub job_id: String,
    pub job_type: JobType,
    pub payload: JobPayload,
    pub priority: u8,
    pub created_at: u64,
    pub max_retries: u32,
}

impl BackgroundJob {
    pub fn new(job_type: JobType, payload: JobPayload) -> Self {
        Self {
            job_id: String::new(), // Will be set when submitted
            job_type,
            payload,
            priority: 5, // Default priority
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            max_retries: 3,
        }
    }

    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }

    pub fn with_max_retries(mut self, retries: u32) -> Self {
        self.max_retries = retries;
        self
    }
}

/// Job types for background processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobType {
    /// Run dungeon simulation
    DungeonSimulation,
    /// Calculate player analytics
    PlayerAnalytics,
    /// Process game statistics
    GameStatistics,
    /// Generate AI content
    AiContentGeneration,
    /// Export game data
    DataExport,
    /// Cleanup old sessions
    SessionCleanup,
    /// Process achievements
    AchievementProcessing,
}

/// Job payload data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobPayload {
    DungeonSimulation {
        dungeon_id: String,
        level: u32,
        iterations: u32,
    },
    PlayerAnalytics {
        player_id: String,
        time_range: String,
    },
    GameStatistics {
        stat_type: String,
        aggregation_period: String,
    },
    AiContentGeneration {
        content_type: String,
        parameters: std::collections::HashMap<String, String>,
    },
    DataExport {
        export_type: String,
        filters: Vec<String>,
    },
    SessionCleanup {
        older_than_hours: u64,
    },
    AchievementProcessing {
        player_id: String,
        achievement_ids: Vec<String>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_background_job_creation() {
        let job = BackgroundJob::new(
            JobType::DungeonSimulation,
            JobPayload::DungeonSimulation {
                dungeon_id: "forest_temple".to_string(),
                level: 1,
                iterations: 100,
            },
        )
        .with_priority(8)
        .with_max_retries(5);

        assert_eq!(job.priority, 8);
        assert_eq!(job.max_retries, 5);
        assert!(matches!(job.job_type, JobType::DungeonSimulation));
    }
}
