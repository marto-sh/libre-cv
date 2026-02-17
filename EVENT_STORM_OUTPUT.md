# Event Storming Output

This document captures the output of the Event Storming session for libre-cv.

## 1. Domain Events

### Professional Identity lifecycle
- `ProfessionalIdentityDrafted` — Owner started building their Professional Identity
- `ProfessionalIdentityPublished` — Professional Identity deemed complete enough; Digital Twin comes to life
- `ProfessionalIdentityUpdated` — Professional Identity modified after publishing
- `ProfessionalIdentityUnpublished` — Digital Twin goes offline

### Digital Twin configuration
- `DigitalTwinConfigured` — Owner customized the Digital Twin's behavior (Escalation thresholds, Opportunity seeking preferences, tone, etc.)

### Interaction lifecycle
- `InteractionStarted` — a Visitor or the Digital Twin (outbound) began an Interaction
- `InteractionEscalated` — the Digital Twin decided to involve the Owner
- `InteractionEnded` — the Interaction concluded

### Escalation lifecycle
- `EscalationAccepted` — Owner engaged with the escalated Interaction
- `EscalationDismissed` — Owner declined to engage with the Escalation

### Opportunity
- `OpportunitySearchRequested` — Owner explicitly asked the Digital Twin to seek specific Opportunities
- `OpportunityDiscovered` — Digital Twin identified a relevant Opportunity (from a one-off search, continuous seeking, or a Visitor Interaction)

### Resume
- `ResumeRequested` — a Visitor asked for a Resume with specific criteria
- `ResumeGenerated` — the Digital Twin produced a Resume shaped by the Visitor's criteria

## 2. Commands

| Command | Triggered by | Event |
|---|---|---|
| `DraftProfessionalIdentity` | Owner | `ProfessionalIdentityDrafted` |
| `PublishProfessionalIdentity` | Owner | `ProfessionalIdentityPublished` |
| `UpdateProfessionalIdentity` | Owner | `ProfessionalIdentityUpdated` |
| `UnpublishProfessionalIdentity` | Owner | `ProfessionalIdentityUnpublished` |
| `ConfigureDigitalTwin` | Owner | `DigitalTwinConfigured` |
| `StartInteraction` | Visitor or Twin | `InteractionStarted` |
| `EscalateInteraction` | Twin (autonomous) | `InteractionEscalated` |
| `EndInteraction` | Visitor, Twin, or Owner | `InteractionEnded` |
| `AcceptEscalation` | Owner | `EscalationAccepted` |
| `DismissEscalation` | Owner | `EscalationDismissed` |
| `RequestOpportunitySearch` | Owner | `OpportunitySearchRequested` |
| *(continuous seeking)* | Twin (background) | `OpportunityDiscovered` |
| `RequestResume` | Visitor | `ResumeRequested` |
| *(automatic generation)* | Twin | `ResumeGenerated` |

## 3. Aggregates

- **Professional Identity** — `DraftProfessionalIdentity`, `PublishProfessionalIdentity`, `UpdateProfessionalIdentity`, `UnpublishProfessionalIdentity`
- **Digital Twin** — `ConfigureDigitalTwin`, plus background Opportunity seeking
- **Interaction** — `StartInteraction`, `EscalateInteraction`, `EndInteraction`, `RequestResume`
- **Escalation** — `AcceptEscalation`, `DismissEscalation` (separate from Interaction because the Owner acts on it independently)


## 4. Emergent Bounded Contexts

- **Identity Context**
  - Aggregates: Professional Identity, Digital Twin
  - Setting up and managing your digital self: your data (Professional Identity) and your Digital Twin's behavior (configuration). The Digital Twin's lifecycle is tied to the Professional Identity (published = active, unpublished = offline).

- **Engagement Context**
  - Aggregates: Interaction, Escalation, Resume
  - Exchanges between Visitors and the Digital Twin. Includes the Escalation bridge to the Owner, and Resume generation within Interactions. Consumes data from the Identity Context.

- **Discovery Context**
  - A matching engine that works in both directions: finding Opportunities for Owners, and helping Visitors find relevant Digital Twins. Once a match is made, it feeds into the Engagement Context.
