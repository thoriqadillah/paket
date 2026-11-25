use serde::{Deserialize, Serialize};

use super::identifier::{BackendId, PackageId};

/// The action the user asked a backend to perform on one package.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChangeAction {
    Install,
    Update,
    Downgrade,
    Remove,
    Reinstall,
    Obsolete,
}

/// A change the user selected, before a backend resolves dependencies.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChangeIntent {
    pub action: ChangeAction,
    pub target: PackageId,
}

/// Whether a resolved change was selected by the user or added by the solver.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChangeOrigin {
    Selected,
    Dependency,
}

/// A solver effect for one package, including dependency-driven changes.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResolvedChange {
    pub action: ChangeAction,
    pub target: PackageId,
    pub previous_version: Option<String>,
    pub candidate_version: Option<String>,
    pub download_size: Option<u64>,
    pub installed_size: Option<u64>,
    /// Absent when the backend cannot tell a selected change from a dependency one.
    pub origin: Option<ChangeOrigin>,
}

/// How much of a backend's solver output Paket can trust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PreviewQuality {
    NativeComplete,
    NativePartial,
    Estimated,
    Unavailable,
}

/// One backend's plan for a group of intents.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackendPlan {
    pub backend: BackendId,
    pub intents: Vec<ChangeIntent>,
    pub resolved: Vec<ResolvedChange>,
    pub preview: PreviewQuality,
}

/// An ordered group of independent backend plans.
///
/// Steps run one at a time; a batch never commits or rolls back across backends.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchPlan {
    pub steps: Vec<BackendPlan>,
}

/// Lifecycle state of a mutation job or batch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JobStatus {
    Draft,
    Planning,
    AwaitingReview,
    Queued,
    Running,
    Succeeded,
    Failed,
    CancelRequested,
    Cancelled,
    NeedsReconciliation,
}

/// Result of one backend step in a batch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StepStatus {
    Succeeded,
    Failed,
    /// The step did not run, e.g. after an earlier failure or a cancellation.
    Skipped,
}

/// A backend step's result. `applied` stays empty when the step did not run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackendOutcome {
    pub backend: BackendId,
    pub status: StepStatus,
    pub applied: Vec<ResolvedChange>,
}

/// Every step's result, including steps that did not run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchOutcome {
    pub steps: Vec<BackendOutcome>,
}

/// Overall result of a non-atomic batch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BatchSummary {
    Succeeded,
    PartiallySucceeded,
    Failed,
    /// No step succeeded or failed, so no result can be claimed.
    NotRun,
}

impl BatchOutcome {
    pub fn summary(&self) -> BatchSummary {
        let succeeded = self
            .steps
            .iter()
            .filter(|step| step.status == StepStatus::Succeeded)
            .count();
        let failed = self
            .steps
            .iter()
            .filter(|step| step.status == StepStatus::Failed)
            .count();

        if succeeded == 0 && failed == 0 {
            BatchSummary::NotRun
        } else if failed == 0 && succeeded == self.steps.len() {
            BatchSummary::Succeeded
        } else if succeeded > 0 {
            BatchSummary::PartiallySucceeded
        } else {
            BatchSummary::Failed
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::identifier::InstallationScope;

    fn id(backend: BackendId, native: &str) -> PackageId {
        PackageId {
            backend,
            scope: InstallationScope::System,
            native: native.to_string(),
        }
    }

    fn intent(action: ChangeAction, target: PackageId) -> ChangeIntent {
        ChangeIntent { action, target }
    }

    fn resolved(
        action: ChangeAction,
        target: PackageId,
        origin: Option<ChangeOrigin>,
    ) -> ResolvedChange {
        ResolvedChange {
            action,
            target,
            previous_version: None,
            candidate_version: None,
            download_size: None,
            installed_size: None,
            origin,
        }
    }

    fn outcome(backend: BackendId, status: StepStatus) -> BackendOutcome {
        BackendOutcome {
            backend,
            status,
            applied: Vec::new(),
        }
    }

    #[test]
    fn selected_intent_stays_separate_from_solver_dependency() {
        let selected = id(BackendId::Dnf5, "foo-1.0-1.fc42.x86_64");
        let dependency = id(BackendId::Dnf5, "bar-2.0-1.fc42.x86_64");
        let plan = BackendPlan {
            backend: BackendId::Dnf5,
            intents: vec![intent(ChangeAction::Install, selected.clone())],
            resolved: vec![
                resolved(
                    ChangeAction::Install,
                    selected.clone(),
                    Some(ChangeOrigin::Selected),
                ),
                resolved(
                    ChangeAction::Install,
                    dependency.clone(),
                    Some(ChangeOrigin::Dependency),
                ),
            ],
            preview: PreviewQuality::NativeComplete,
        };

        assert_eq!(plan.intents.len(), 1);
        assert_eq!(plan.resolved.len(), 2);
        assert_eq!(plan.intents[0].target, selected);
        assert_eq!(plan.resolved[1].target, dependency);
        assert_ne!(plan.intents[0].target, plan.resolved[1].target);
        assert_eq!(plan.preview, PreviewQuality::NativeComplete);

        let decoded: BackendPlan =
            serde_json::from_str(&serde_json::to_string(&plan).unwrap()).unwrap();
        assert_eq!(decoded, plan);
    }

    #[test]
    fn partial_preview_is_not_reported_as_complete() {
        let plan = BackendPlan {
            backend: BackendId::Apt,
            intents: vec![intent(
                ChangeAction::Remove,
                id(BackendId::Apt, "foo:amd64"),
            )],
            resolved: vec![resolved(
                ChangeAction::Remove,
                id(BackendId::Apt, "foo:amd64"),
                Some(ChangeOrigin::Selected),
            )],
            preview: PreviewQuality::NativePartial,
        };

        assert_ne!(plan.preview, PreviewQuality::NativeComplete);
        assert_ne!(plan.preview, PreviewQuality::Unavailable);
    }

    #[test]
    fn unavailable_preview_still_identifies_the_selected_target() {
        let selected = id(BackendId::Flatpak, "org.foo.App/x86_64/stable");
        let plan = BackendPlan {
            backend: BackendId::Flatpak,
            intents: vec![intent(ChangeAction::Install, selected.clone())],
            resolved: Vec::new(),
            preview: PreviewQuality::Unavailable,
        };

        assert_eq!(plan.preview, PreviewQuality::Unavailable);
        assert!(plan.resolved.is_empty());
        assert_eq!(plan.intents[0].target, selected);
    }

    #[test]
    fn multi_backend_batch_keeps_steps_separate() {
        let dnf = BackendPlan {
            backend: BackendId::Dnf5,
            intents: vec![intent(ChangeAction::Install, id(BackendId::Dnf5, "foo"))],
            resolved: vec![resolved(
                ChangeAction::Install,
                id(BackendId::Dnf5, "foo"),
                Some(ChangeOrigin::Selected),
            )],
            preview: PreviewQuality::NativeComplete,
        };
        let flatpak = BackendPlan {
            backend: BackendId::Flatpak,
            intents: vec![intent(
                ChangeAction::Install,
                id(BackendId::Flatpak, "org.foo.App"),
            )],
            resolved: vec![resolved(
                ChangeAction::Install,
                id(BackendId::Flatpak, "org.foo.App"),
                Some(ChangeOrigin::Selected),
            )],
            preview: PreviewQuality::NativeComplete,
        };
        let batch = BatchPlan {
            steps: vec![dnf.clone(), flatpak.clone()],
        };

        assert_eq!(batch.steps.len(), 2);
        assert_ne!(batch.steps[0].backend, batch.steps[1].backend);
        assert_ne!(batch.steps[0], batch.steps[1]);

        let decoded: BatchPlan =
            serde_json::from_str(&serde_json::to_string(&batch).unwrap()).unwrap();
        assert_eq!(decoded, batch);
    }

    #[test]
    fn multi_backend_partial_failure_marks_later_step_skipped() {
        let dnf = BackendPlan {
            backend: BackendId::Dnf5,
            intents: vec![intent(ChangeAction::Install, id(BackendId::Dnf5, "foo"))],
            resolved: vec![resolved(
                ChangeAction::Install,
                id(BackendId::Dnf5, "foo"),
                Some(ChangeOrigin::Selected),
            )],
            preview: PreviewQuality::NativeComplete,
        };
        let flatpak = BackendPlan {
            backend: BackendId::Flatpak,
            intents: vec![intent(
                ChangeAction::Install,
                id(BackendId::Flatpak, "org.foo.App"),
            )],
            resolved: Vec::new(),
            preview: PreviewQuality::Unavailable,
        };
        let batch = BatchPlan {
            steps: vec![dnf, flatpak],
        };

        let outcome = BatchOutcome {
            steps: vec![
                BackendOutcome {
                    backend: batch.steps[0].backend,
                    status: StepStatus::Succeeded,
                    applied: batch.steps[0].resolved.clone(),
                },
                outcome(batch.steps[1].backend, StepStatus::Failed),
                outcome(BackendId::Snap, StepStatus::Skipped),
            ],
        };

        assert_eq!(outcome.summary(), BatchSummary::PartiallySucceeded);
        assert_eq!(outcome.steps[1].status, StepStatus::Failed);
        assert_eq!(outcome.steps[2].status, StepStatus::Skipped);
        assert!(outcome.steps[2].applied.is_empty());
        assert_ne!(outcome.summary(), BatchSummary::Succeeded);

        let decoded: BatchOutcome =
            serde_json::from_str(&serde_json::to_string(&outcome).unwrap()).unwrap();
        assert_eq!(decoded, outcome);
    }

    #[test]
    fn batch_summary_never_invents_results_for_unrun_steps() {
        assert_eq!(
            BatchOutcome {
                steps: vec![outcome(BackendId::Dnf5, StepStatus::Succeeded)]
            }
            .summary(),
            BatchSummary::Succeeded
        );
        assert_eq!(
            BatchOutcome {
                steps: vec![
                    outcome(BackendId::Dnf5, StepStatus::Failed),
                    outcome(BackendId::Flatpak, StepStatus::Skipped),
                ]
            }
            .summary(),
            BatchSummary::Failed
        );
        assert_eq!(
            BatchOutcome {
                steps: vec![outcome(BackendId::Snap, StepStatus::Skipped)]
            }
            .summary(),
            BatchSummary::NotRun
        );
        assert_eq!(
            BatchOutcome { steps: Vec::new() }.summary(),
            BatchSummary::NotRun
        );
    }
}
