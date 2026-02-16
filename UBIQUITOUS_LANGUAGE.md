# Ubiquitous Language Glossary

This document is a single source of truth for the terminology used in the libre-cv domain. It should be developed collaboratively by developers, domain experts, and stakeholders. These terms must be used consistently in all conversations, documentation, and code.

| Term | Definition | Business Rules / Invariants |
| --- | --- | --- |
| **Digital Twin** | An interactable representation of a professional that can act on behalf of its Owner. It is built on top of a Professional Identity. | A Digital Twin belongs to exactly one Owner. It must be accessible by both humans and AI agents. |
| **Professional Identity** | The full dataset that describes a professional: experience, skills, aspirations, preferences, and any other information they choose to share. | Forms the content of a Digital Twin. The Owner decides what is included. |
| **Owner** | The tech professional who creates, controls, and owns a Digital Twin. | An Owner has full control over their Digital Twin and its Professional Identity. |
| **Visitor** | Any human or agent that discovers, evaluates, or interacts with a Digital Twin. | A Visitor interacts with the Digital Twin, not directly with the Owner (unless the Digital Twin decides to bridge that connection). |
| **Interaction** | An exchange between a Visitor and a Digital Twin. This can range from querying information to a full conversation. | The Digital Twin always engages in the Interaction on behalf of the Owner. |
| **Escalation** | The decision by a Digital Twin to involve the Owner in an Interaction. | Escalation is based on the Owner's Professional Identity. |
| **Opportunity** | A professional prospect (job, project, partnership, speaking engagement, etc.) relevant to an Owner. An Opportunity can be presented by a Visitor during an Interaction, or proactively discovered by the Digital Twin. | Relevance is determined based on the Owner's Professional Identity. |
| **Resume** | A summary of an Owner's professional experience and skills. | A Resume is always derived from a Professional Identity. |