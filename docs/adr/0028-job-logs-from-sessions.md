# 0028. Job logs readable from Claude's sessions

- Status: accepted (owner, 2026-10-11)
- Date: 2026-10-11
- Scope: framework

## Context

Claude works in cloud sessions whose outbound network is set by the owner in the session's environment. Until 2026-10-11 that setting let a session reach GitHub's API but not the hosts GitHub sends a job's log from: the API answers a log request with a redirect to Azure Blob Storage (`productionresultssa7.blob.core.windows.net` on 2026-10-11). Workflows were therefore designed for what a session could read: annotations, pull request comments and commits ([ADR 0016](0016-self-play-training.md), Context). A failure that a workflow did not put in an annotation could only be read by the owner; the cause of three CI failures of 2026-10-10 was confirmed only once logs became readable.

On 2026-10-11 the owner set the environment's network access to a custom list: the default list of package registries, plus the hosts GitHub serves job logs from. The owner weighed the risk first. A session runs code from the repository and reads text from outside, such as pull request comments and logs, and every host it can reach is a host it could send data to. Cloud storage hosts are shared by many owners. The owner judged the risk acceptable because the repository is public and the environment holds no secrets.

## Decision

1. **Claude's cloud sessions may read GitHub Actions job logs.** The environment's network allows GitHub's log hosts beside the default package registries. Skill `github-api` holds the recipe.
2. **Annotations stay the first place to read a result.** Workflows keep reporting failures as annotations with an excerpt of the output (`scripts/annotate.sh`), and results as annotations, comments or commits. A session reads the full log only when those are not enough, because a log costs far more context than its annotations.
3. **The setting is reconsidered before the environment gets a secret or the repository goes private.** Either change raises the cost of a leak. The owner decides again then, and a new record supersedes this one if the setting changes.

## Consequences

- Claude can find the cause of a failure without asking the owner for the log, including failures in steps that annotate nothing.
- Each host added to the network list is another place data could go. That is accepted only while the repository is public and the environment has no secrets.
- Before adding a secret to the environment, or making the repository private, the network list is reviewed first.
- Artifacts are not covered by this decision.
