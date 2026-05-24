use std::time::Duration;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
pub mod activities;
pub mod workflow;

pub const TASK_QUEUE: &str = "prehab-queue";

/// Demo-Zeitskala: in der Live-Demo nutzen wir Sekunden statt Tage,
/// damit das Publikum den vollen Lebenszyklus sieht.
/// Auf `true` setzen für realistische Tage.
pub const REALISTIC_TIME: bool = false;

pub fn days(n: u64) -> Duration {
    if REALISTIC_TIME {
        Duration::from_secs(n * 24 * 60 * 60)
    } else {
        Duration::from_secs(n * 2) // 1 Tag = 2 Sekunden in der Demo
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnrollPatient {
    pub pseudonym: String,
    pub surgery_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromResponse {
    pub questionnaire: String,
    pub score: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewDecision {
    pub proceed: bool,
    pub reviewer_note: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrehabResult {
    Completed { final_score: i32 },
    SurgeryDeferred { reason: String },
    Abandoned { reason: String },
}

