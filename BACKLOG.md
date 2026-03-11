# TODO

Remove items from this list once completed.

Tasks organized by bounded context, roughly priority-ordered within each section.

## Identity Context (Core)

- [ ] Evaluate splitting step files and feature files — remove need for section comments
- [ ] Session tracking — add/remove SessionId references for traceability
- [ ] Digital Twin aggregate — configuration, behavior rules, lifecycle tied to Professional Identity
  - [ ] `ConfigureDigitalTwin` command — escalation thresholds, opportunity preferences, tone

## Engagement Context (Supporting)

- [ ] Interaction aggregate — `StartInteraction`, `EndInteraction`, `EscalateInteraction`
- [ ] Escalation aggregate — `AcceptEscalation`, `DismissEscalation`
- [ ] Resume generation — `RequestResume`, `ResumeGenerated` (derived from Professional Identity)

## Discovery Context (Supporting)

- [ ] Opportunity model — `RequestOpportunitySearch`, `OpportunityDiscovered`
- [ ] Matching engine — bidirectional matching (Owners ↔ Visitors)

## Infrastructure

- [ ] Persistence layer — event sourcing or repository pattern
- [ ] Application service / command handlers
