# Ubiquitous Language Glossary

This document is a single source of truth for the terminology used in the libre-cv domain. It should be developed collaboratively by developers, domain experts, and stakeholders. These terms must be used consistently in all conversations, documentation, and code.

| Term | Definition | Business Rules / Invariants |
| --- | --- | --- |
| **Digital Twin** | An interactable representation of a professional that can act on behalf of its Owner. It is built on top of a Professional Identity. | A Digital Twin belongs to exactly one Owner. It must be accessible by both humans and AI agents. |
| **Professional Identity** | Structured, queryable layers of professional information — Experiences, Projects, Skills, and Expectations — materialized by an LLM from Session ground truth. | Forms the content of a Digital Twin. The Owner decides what is included. Curation can be re-run as LLMs improve, without the Owner redoing work. |
| **Session** | A conversation between the Owner and an AI agent. Sessions are the ground truth for the Professional Identity — they capture the richest signal about the professional. | Immutable once recorded. The Professional Identity is derived from Sessions, never the other way around. |
| **Experience** | A professional engagement or context: a job, freelance contract, open-source involvement, volunteer position, or any other professional setting. The "where/when." | Part of the Professional Identity. Cross-references Skills applied and Projects undertaken within it. |
| **Project** | A specific piece of work with concrete outcomes. The "what." Can be part of an Experience or standalone. | Part of the Professional Identity. Cross-references Skills applied. |
| **Skill** | A capability the Owner has, with Details providing evidence and context. | Part of the Professional Identity. Cross-references the Experiences and Projects where it was applied. |
| **Expectation** | What the Owner expects from their professional opportunities. Can be a Constraint or a Preference. | Part of the Professional Identity. |
| **Constraint** | A non-negotiable Expectation. A dealbreaker. | Example: "I won't work in defense." |
| **Preference** | A desired but flexible Expectation. The Owner can compromise. | Example: "I prefer remote work." |
| **Detail** | A single traceable piece of information within an Experience, Project, Skill, or Expectation. Each Detail traces back to the specific Session section(s) it was derived from. | Materialized by LLM curation. Agents query Details for quick answers, and drill down to source Sessions when needed. |
| **Owner** | The tech professional who creates, controls, and owns a Digital Twin. | An Owner has full control over their Digital Twin and its Professional Identity. |
| **Visitor** | Any human or agent that discovers, evaluates, or interacts with a Digital Twin. | A Visitor interacts with the Digital Twin, not directly with the Owner (unless the Digital Twin decides to bridge that connection). |
| **Interaction** | An exchange between a Visitor and a Digital Twin. This can range from querying information to a full conversation. | The Digital Twin always engages in the Interaction on behalf of the Owner. |
| **Escalation** | The decision by a Digital Twin to involve the Owner in an Interaction. | Escalation is based on the Owner's Professional Identity. |
| **Opportunity** | A professional prospect (job, project, partnership, speaking engagement, etc.) relevant to an Owner. An Opportunity can be presented by a Visitor during an Interaction, or proactively discovered by the Digital Twin. | Relevance is determined based on the Owner's Professional Identity. |
| **Resume** | A summary of an Owner's professional experience and skills. | A Resume is always derived from a Professional Identity. |