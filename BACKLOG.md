# TODO

Tasks organized by bounded context, roughly priority-ordered within each section.

## Identity Context (Core)

- [ ] Headline & Summary — set/clear headline and summary on Professional Identity
- [ ] Project CRUD — add, update, remove projects (standalone or linked to an Experience)
  - [ ] Detail management on Projects — add, update, remove details
  - [ ] Skill ↔ Project cross-references — bidirectional linking
- [ ] Expectation CRUD — add, update, remove expectations (Constraint or Preference)
  - [ ] Detail management on Expectations — add, update, remove details
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

## Modeling

- [ ] Reclassify entities in value_objects.rs — Experience, Project, Skill, and Expectation have identity (id fields) and are entities, not value objects. Move them to a dedicated module or rename the file to reflect the distinction.

## Infrastructure

- [ ] Persistence layer — event sourcing or repository pattern
- [ ] Application service / command handlers
