# TODO

Tasks organized by bounded context, roughly priority-ordered within each section.

## Identity Context (Core)

- [x] Draft Professional Identity — `DraftProfessionalIdentity` command, `ProfessionalIdentityDrafted` event
- [x] Publish / Unpublish Professional Identity — `PublishProfessionalIdentity`, `UnpublishProfessionalIdentity` commands
- [ ] Update Professional Identity — enrich the aggregate with experience, skills, aspirations, preferences
- [ ] Domain error types — replace `&'static str` errors with proper domain error enum
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
