use serde::{Serialize, Deserialize};

pub enum GitHooks {
    #[serde(rename = "applypatch-msg")]
    ApplypatchMsg,

    #[serde(rename = "pre-applypatch")]
    PreApplypatch,

    #[serde(rename = "post-applypatch")]
    PostApplypatch,

    #[serde(rename = "pre-commit")]
    PreCommit,

    #[serde(rename = "pre-merge-commit")]
    PreMergeCommit,

    #[serde(rename = "prepare-commit-msg")]
    PrepareCommitMsg,

    #[serde(rename = "commit-msg")]
    CommitMsg,

    #[serde(rename = "post-commit")]
    PostCommit,

    #[serde(rename = "pre-rebase")]
    PreRebase,

    #[serde(rename = "post-checkout")]
    PostCheckout,

    #[serde(rename = "post-merge")]
    PostMerge,

    #[serde(rename = "pre-push")]
    PrePush,

    #[serde(rename = "pre-receive")]
    PreReceive,

    #[serde(rename = "update")]
    Update,

    #[serde(rename = "proc-receive")]
    ProcReceive,

    #[serde(rename = "post-receive")]
    PostReceive,

    #[serde(rename = "post-update")]
    PostUpdate,

    #[serde(rename = "reference-transaction")]
    ReferenceTransaction,

    #[serde(rename = "push-to-checkout")]
    PushToCheckout,

    #[serde(rename = "pre-auto-gc")]
    PreAutoGc,

    #[serde(rename = "post-rewrite")]
    PostRewrite,

    #[serde(rename = "sendemail-validate")]
    SendemailValidate,

    #[serde(rename = "fsmonitor-watchman")]
    FsmonitorWatchman,

    #[serde(rename = "p4-changelist")]
    P4Changelist,

    #[serde(rename = "p4-prepare-changelist")]
    P4PrepareChangelist,

    #[serde(rename = "p4-post-changelist")]
    P4PostChangelist,

    #[serde(rename = "p4-pre-submit")]
    P4PreSubmit,

    #[serde(rename = "post-index-change")]
    PostIndexChange,
}
