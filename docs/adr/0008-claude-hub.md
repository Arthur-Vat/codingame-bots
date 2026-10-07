# 0008. A Claude conversation is the hub

- Status: accepted
- Date: 2026-10-07

## Context

The owner wants one place to follow the work, approve changes and receive notifications, rather than switching between GitHub and several Claude sessions. Claude Code Projects (one coordinating conversation with parallel worker threads and an Overview of pull requests) fits this, but it is a beta that has not reached the owner's account yet.

## Decision

- Until Projects is available: one long-running claude.ai conversation is the hub. It attaches this repository, opens pull requests, and is where the owner approves merges.
- When Projects becomes available: move to a project named "CodinGame Bot Lab" with this repository, the instructions in `docs/WORKFLOW.md`, and the Overview's merge button.

## Consequences

- Long conversations get summarized over time, so durable knowledge must live in the repository: `docs/`, the ADRs and the journal. The conversation is never the only record of a decision.
- GitHub notifications for this repository should be set to "Participating" so they do not duplicate the hub.
