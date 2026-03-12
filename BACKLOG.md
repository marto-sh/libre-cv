# Backlog

<!-- Remove items from this list once completed. -->

Tasks organized by bounded context, roughly priority-ordered within each section.

## Identity Context (Core)

- [ ] Digital Twin — escalation rules (design trigger types, add/remove/update rules)
- [ ] Digital Twin — opportunity seeking preferences (seeking mode, criteria filters; depends on Discovery Context)

## Engagement Context (Supporting)

- [ ] Interaction aggregate — `StartInteraction`, `EndInteraction`, `EscalateInteraction`
- [ ] Escalation aggregate — `AcceptEscalation`, `DismissEscalation`
- [ ] Resume generation — `RequestResume`, `ResumeGenerated` (derived from Professional Identity)

## Discovery Context (Supporting)

- [ ] Opportunity model — `RequestOpportunitySearch`, `OpportunityDiscovered`
- [ ] Matching engine — bidirectional matching (Owners ↔ Visitors)

## Infrastructure

- [ ] Persistence layer for Professional Identity — evaluate VCS-based persistence (sessions are the raw source of truth; event sourcing rejected as too complex for the benefit — see [literature review](https://github.com/marto-sh/guidelines/blob/main/docs/research/event-sourcing/literature-review.md))
- [ ] Application service / command handlers
