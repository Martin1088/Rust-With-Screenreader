use serde::{Deserialize, Serialize};
use temporalio_macros::activities;
use temporalio_sdk::activities::{ActivityContext, ActivityError};

pub struct PrehabActivities;

#[activities]
impl PrehabActivities {
    #[activity]
    pub async fn send_prom_request(
        _ctx: ActivityContext,
        input: SendPromInput,
    ) -> Result<(), ActivityError> {
        tracing::info!(
            pseudonym = %input.pseudonym,
            questionnaire = %input.questionnaire,
            "PROM-Anfrage gesendet (würde echten Webhook an yeswecan!cer auslösen)"
        );
        Ok(())
    }

    #[activity]
    pub async fn send_reminder(
        _ctx: ActivityContext,
        input: SendReminderInput,
    ) -> Result<(), ActivityError> {
        tracing::warn!(
            pseudonym = %input.pseudonym,
            questionnaire = %input.questionnaire,
            "Erinnerung versendet (Patient hat noch nicht geantwortet)"
        );
        Ok(())
    }

    #[activity]
    pub async fn notify_coordinator(
        _ctx: ActivityContext,
        input: NotifyCoordinatorInput,
    ) -> Result<(), ActivityError> {
        tracing::warn!(
            pseudonym = %input.pseudonym,
            "Studienkoordinator informiert — manuelle Review benötigt"
        );
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendPromInput {
    pub pseudonym: String,
    pub questionnaire: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendReminderInput {
    pub pseudonym: String,
    pub questionnaire: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyCoordinatorInput {
    pub pseudonym: String,
    pub reason: String,
}
