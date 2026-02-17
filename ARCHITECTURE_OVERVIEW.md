# Architecture Overview

## 1. Subdomain Classification

### Core Domain
> The heart of the business and what gives libre-cv its competitive advantage. This is where most of the custom software development effort should be focused.
> - **Identity**: The Professional Identity and Digital Twin. Building a rich, structured, interactable representation of a professional that acts on their behalf. This is what makes libre-cv unique.

### Supporting Subdomains
> Necessary for the product to function, but not the differentiator.
> - **Engagement**: Interactions, Escalations, and Resumes. The mechanics of exchanging with Visitors and bridging to the Owner.
> - **Discovery**: Bidirectional matching — finding Opportunities for Owners and helping Visitors find relevant Digital Twins.

### Generic Subdomains
> Solved problems — prefer off-the-shelf solutions.
> - *None identified yet. Candidates may emerge during implementation (e.g. authentication, notifications).*

## 2. Bounded Contexts

- **Identity Context** (Core) — Professional Identity, Digital Twin
- **Engagement Context** (Supporting) — Interaction, Escalation, Resume
- **Discovery Context** (Supporting) — Bidirectional matching engine

*(For a detailed map of the relationships between these contexts, see `CONTEXT_MAP.md`)*
