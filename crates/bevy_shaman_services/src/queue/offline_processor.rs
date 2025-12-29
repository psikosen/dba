use super::rabbitmq::RabbitMqClient;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::info;

/// Offline processing queue for long-running tasks
pub struct OfflineProcessor {
    client: RabbitMqClient,
    queue_name: String,
}

impl OfflineProcessor {
    /// Create a new offline processor
    pub async fn new(client: RabbitMqClient, queue_name: String) -> Result<Self> {
        // Declare a durable queue for offline processing
        client.declare_queue(&queue_name, true).await?;

        Ok(Self { client, queue_name })
    }

    /// Submit an offline job
    pub async fn submit_job(&self, job: OfflineJob) -> Result<String> {
        let job_id = uuid::Uuid::new_v4().to_string();
        let mut job_with_id = job;
        job_with_id.job_id = job_id.clone();

        let message = serde_json::to_vec(&job_with_id)?;
        self.client.publish_to_queue(&self.queue_name, &message).await?;

        info!("Submitted offline job: {} (type: {:?})", job_id, job_with_id.job_type);
        Ok(job_id)
    }

    /// Submit multiple offline jobs
    pub async fn submit_batch(&self, jobs: Vec<OfflineJob>) -> Result<Vec<String>> {
        let mut job_ids = Vec::new();

        for job in jobs {
            let job_id = self.submit_job(job).await?;
            job_ids.push(job_id);
        }

        info!("Submitted {} offline jobs", job_ids.len());
        Ok(job_ids)
    }

    /// Get the queue name
    pub fn queue_name(&self) -> &str {
        &self.queue_name
    }
}

/// Offline job structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfflineJob {
    pub job_id: String,
    pub job_type: OfflineJobType,
    pub payload: OfflineJobPayload,
    pub scheduled_for: Option<u64>, // Unix timestamp for delayed execution
    pub created_at: u64,
    pub timeout_seconds: u64,
}

impl OfflineJob {
    pub fn new(job_type: OfflineJobType, payload: OfflineJobPayload) -> Self {
        Self {
            job_id: String::new(), // Will be set when submitted
            job_type,
            payload,
            scheduled_for: None,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            timeout_seconds: 3600, // 1 hour default
        }
    }

    pub fn schedule_for(mut self, timestamp: u64) -> Self {
        self.scheduled_for = Some(timestamp);
        self
    }

    pub fn with_timeout(mut self, seconds: u64) -> Self {
        self.timeout_seconds = seconds;
        self
    }
}

/// Offline job types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OfflineJobType {
    /// Generate large reports
    ReportGeneration,
    /// Process historical data
    DataMigration,
    /// Run complex simulations
    Simulation,
    /// Generate AI-based content in bulk
    BulkAiGeneration,
    /// Analyze player behavior patterns
    BehaviorAnalysis,
    /// Generate dungeon variations
    DungeonGeneration,
    /// Process achievement calculations
    AchievementCalculation,
}

/// Offline job payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OfflineJobPayload {
    ReportGeneration {
        report_type: String,
        parameters: std::collections::HashMap<String, String>,
        output_format: String,
    },
    DataMigration {
        source: String,
        destination: String,
        record_count: u64,
    },
    Simulation {
        simulation_type: String,
        iterations: u32,
        parameters: std::collections::HashMap<String, String>,
    },
    BulkAiGeneration {
        content_type: String,
        count: u32,
        templates: Vec<String>,
    },
    BehaviorAnalysis {
        player_ids: Vec<String>,
        analysis_type: String,
        time_range: String,
    },
    DungeonGeneration {
        dungeon_type: String,
        variations: u32,
        difficulty_levels: Vec<String>,
    },
    AchievementCalculation {
        player_ids: Vec<String>,
        achievement_ids: Vec<String>,
        recalculate: bool,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offline_job_creation() {
        let job = OfflineJob::new(
            OfflineJobType::Simulation,
            OfflineJobPayload::Simulation {
                simulation_type: "combat".to_string(),
                iterations: 1000,
                parameters: std::collections::HashMap::new(),
            },
        )
        .with_timeout(7200);

        assert_eq!(job.timeout_seconds, 7200);
        assert!(matches!(job.job_type, OfflineJobType::Simulation));
    }

    #[test]
    fn test_scheduled_job() {
        let future_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 3600; // 1 hour from now

        let job = OfflineJob::new(
            OfflineJobType::ReportGeneration,
            OfflineJobPayload::ReportGeneration {
                report_type: "daily".to_string(),
                parameters: std::collections::HashMap::new(),
                output_format: "pdf".to_string(),
            },
        )
        .schedule_for(future_time);

        assert_eq!(job.scheduled_for, Some(future_time));
    }
}
