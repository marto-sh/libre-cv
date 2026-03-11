# TODO

Remove items from this list once completed.

Tasks organized by bounded context, roughly priority-ordered within each section.

## Identity Context (Core)

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

- [ ] Break value_objects.rs into separate files — one file per value object or logical group, so any struct can be found by file name
- [ ] Evaluate one-file-per-struct convention — decide if related items (e.g. Details + DetailError, Source + SectionLocator) should each get their own file or stay grouped by logical affinity

## Infrastructure

- [ ] Persistence layer — event sourcing or repository pattern
- [ ] Application service / command handlers
