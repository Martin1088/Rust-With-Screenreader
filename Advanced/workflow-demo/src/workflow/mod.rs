use std::time::Duration;
use chrono::{DateTime, Utc};
use temporalio_macros::{workflow, workflow_methods};
use temporalio_sdk::{ActivityOptions, WorkflowContext, WorkflowContextView, WorkflowResult};
use crate::{days, EnrollPatient, PrehabResult, PromResponse, ReviewDecision};
use crate::activities::{NotifyCoordinatorInput, PrehabActivities, SendPromInput, SendReminderInput};
use temporalio_sdk::SyncWorkflowContext;

#[workflow]
pub struct PrehabWorkflow {
    pseudonym: String,
    surgery_date: DateTime<Utc>,
    received_proms: Vec<PromResponse>,
    review_decision: Option<ReviewDecision>,
}

#[workflow_methods]
impl PrehabWorkflow {
    #[init]
    fn new(_ctx: &WorkflowContextView, input: EnrollPatient) -> Self {
        Self {
            pseudonym: input.pseudonym,
            surgery_date: input.surgery_date,
            received_proms: Vec::new(),
            review_decision: None,
        }
    }

    #[run]
    pub async fn run(
        ctx: &mut WorkflowContext<Self>,
    ) -> WorkflowResult<PrehabResult> {
        let pseudonym = ctx.state(|s| s.pseudonym.clone());

        tracing::info!(pseudonym = %pseudonym, "Prähab-Workflow gestartet");
        request_and_wait_for_prom(ctx, &pseudonym, "baseline", 3).await?;
        for week in 1..=3 {
            ctx.timer(days(7)).await;
            let q = format!("weekly_{}", week);
            request_and_wait_for_prom(ctx, &pseudonym, &q, 2).await?;
        }
        ctx.timer(days(7)).await;
        request_and_wait_for_prom(ctx, &pseudonym, "final_assessment", 3).await?;
        let final_score = ctx.state(|s| {
            s.received_proms
                .iter()
                .find(|p| p.questionnaire == "final_assessment")
                .map(|p| p.score)
                .unwrap_or(0)
        });

        if final_score < 30 {
            tracing::warn!(
                pseudonym = %pseudonym,
                final_score,
                "Kritischer Score — warte auf Koordinator-Review"
            );

            ctx.start_activity(
                PrehabActivities::notify_coordinator,
                NotifyCoordinatorInput {
                    pseudonym: pseudonym.clone(),
                    reason: format!("Final assessment score {}", final_score),
                },
                ActivityOptions::start_to_close_timeout(Duration::from_secs(30)),
            )
                .await?;
            ctx.wait_condition(|s| s.review_decision.is_some()).await;

            let decision = ctx.state(|s| s.review_decision.clone().unwrap());

            if !decision.proceed {
                return Ok(PrehabResult::SurgeryDeferred {
                    reason: decision.reviewer_note,
                });
            }
        }

        Ok(PrehabResult::Completed { final_score })
    }
    #[signal(name = "prom_completed")]
    pub fn on_prom_completed(
        &mut self,
        _ctx: &mut SyncWorkflowContext<Self>,
        prom: PromResponse,
    ) {
        tracing::info!(
            questionnaire = %prom.questionnaire,
            score = prom.score,
            "PROM-Signal empfangen"
        );
        self.received_proms.push(prom);
    }

    #[signal(name = "coordinator_review")]
    pub fn on_coordinator_review(
        &mut self,
        _ctx: &mut SyncWorkflowContext<Self>,
        decision: ReviewDecision,
    ) {
        tracing::info!(
            proceed = decision.proceed,
            "Koordinator-Entscheidung empfangen"
        );
        self.review_decision = Some(decision);
    }
}

pub async fn request_and_wait_for_prom(
    ctx: &mut WorkflowContext<PrehabWorkflow>,
    pseudonym: &str,
    questionnaire: &str,
    timeout_days: u64,
) -> WorkflowResult<()> {
    ctx.start_activity(
        PrehabActivities::send_prom_request,
        SendPromInput {
            pseudonym: pseudonym.into(),
            questionnaire: questionnaire.into(),
        },
        ActivityOptions::start_to_close_timeout(Duration::from_secs(30)),
    )
        .await?;
    
    let q = questionnaire.to_string();
    let mut timer = ctx.timer(days(timeout_days));
    let mut signal = ctx.wait_condition(move |s| {
        s.received_proms.iter().any(|p| p.questionnaire == q)
    });

    temporalio_sdk::workflows::select! {
        _ = signal => {
            tracing::info!(questionnaire, "PROM rechtzeitig empfangen");
            return Ok(());
        }
        _ = timer => {
            tracing::warn!(questionnaire, "⏰ Timeout — sende Erinnerung");
        }
    }
    ctx.start_activity(
        PrehabActivities::send_reminder,
        SendReminderInput {
            pseudonym: pseudonym.into(),
            questionnaire: questionnaire.into(),
        },
        ActivityOptions::start_to_close_timeout(Duration::from_secs(30)),
    )
        .await?;

    let q = questionnaire.to_string();
    ctx.wait_condition(move |s| {
        s.received_proms.iter().any(|p| p.questionnaire == q)
    })
        .await;

    Ok(())
}
