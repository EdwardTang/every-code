use optillm_mars::MarsCoordinator;
use optillm_mars::types::MarsEvent;
use std::time::Duration;
use tokio::sync::mpsc;

pub(crate) struct MarsExecutor {
    pub(crate) timeout: Duration,
}

impl MarsExecutor {
    pub(crate) fn new(timeout: Duration) -> Self {
        Self { timeout }
    }

    pub(crate) async fn run(&self, query: &str) -> anyhow::Result<(Vec<MarsEvent>, String)> {
        let mut coordinator = MarsCoordinator::new(Default::default());

        let (tx, mut rx) = mpsc::channel::<MarsEvent>(128);
        // Note: optillm-mars currently runs with placeholder solutions and sends events
        // through an internal channel. In future work we will integrate ModelClient
        // and plumb these events through the TUI with strict ordering.
        let run = tokio::time::timeout(self.timeout, coordinator.run(query));
        let result = run.await??;

        // Drain any events we received.
        let mut events = Vec::new();
        while let Ok(event) = rx.try_recv() {
            events.push(event);
        }

        Ok((events, result.answer))
    }
}

