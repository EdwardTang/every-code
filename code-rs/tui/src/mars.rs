use crate::app_event_sender::AppEventSender;
use crate::chatwidget::BackgroundOrderTicket;
use optillm_mars::types::MarsEvent;

pub(crate) fn handle_mars_event(
    app_event_tx: &AppEventSender,
    ticket: &BackgroundOrderTicket,
    event: MarsEvent,
) {
    match event {
        MarsEvent::ExplorationStarted { num_agents } => {
            app_event_tx.send_background_before_next_output_with_ticket(
                ticket,
                format!("[mars] exploration started ({num_agents} agents)"),
            );
        }
        MarsEvent::SolutionGenerated { agent_id, .. } => {
            app_event_tx.send_background_before_next_output_with_ticket(
                ticket,
                format!("[mars] agent {agent_id} produced a solution"),
            );
        }
        MarsEvent::VerificationStarted => {
            app_event_tx.send_background_before_next_output_with_ticket(
                ticket,
                "[mars] verification started",
            );
        }
        MarsEvent::SolutionVerified { score, .. } => {
            app_event_tx.send_background_before_next_output_with_ticket(
                ticket,
                format!("[mars] solution verified (score={score:.2})"),
            );
        }
        MarsEvent::AggregationStarted => {
            app_event_tx.send_background_before_next_output_with_ticket(
                ticket,
                "[mars] aggregation started",
            );
        }
        MarsEvent::SolutionsAggregated { .. } => {
            app_event_tx.send_background_before_next_output_with_ticket(
                ticket,
                "[mars] aggregation produced a combined solution",
            );
        }
        MarsEvent::ImprovementStarted { iteration } => {
            app_event_tx.send_background_before_next_output_with_ticket(
                ticket,
                format!("[mars] improvement iteration {iteration} started"),
            );
        }
        MarsEvent::SolutionImproved { .. } => {
            app_event_tx.send_background_before_next_output_with_ticket(
                ticket,
                "[mars] solution improved",
            );
        }
        MarsEvent::StrategyNetworkStarted => {
            app_event_tx.send_background_before_next_output_with_ticket(
                ticket,
                "[mars] strategy network started",
            );
        }
        MarsEvent::StrategyExtracted { .. } => {
            app_event_tx.send_background_before_next_output_with_ticket(
                ticket,
                "[mars] strategy extracted",
            );
        }
        MarsEvent::SynthesisStarted => {
            app_event_tx.send_background_before_next_output_with_ticket(ticket, "[mars] synthesis started");
        }
        MarsEvent::AnswerSynthesized { .. } => {
            app_event_tx
                .send_background_before_next_output_with_ticket(ticket, "[mars] answer synthesized");
        }
        MarsEvent::Completed { .. } => {
            app_event_tx.send_background_before_next_output_with_ticket(ticket, "[mars] completed");
        }
        MarsEvent::Error { message } => {
            app_event_tx.send_background_before_next_output_with_ticket(
                ticket,
                format!("[mars] error: {message}"),
            );
        }
    }
}

