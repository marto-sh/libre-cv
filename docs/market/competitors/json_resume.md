# JSON Resume

> **Website:** https://jsonresume.org/
> **Founded:** 2013
> **Creator:** Thomas Davis
> **License:** MIT (Open Source)
> **Status:** Active community project

## Project Overview

JSON Resume is an open-source initiative that defines a JSON schema standard for resume data. It provides a structured, machine-readable format for representing professional information, making resumes portable, versionable, and programmable.

The project was created to solve the problem of resume data being locked in proprietary formats and platforms. By providing an open standard, JSON Resume enables developers to build tools that can import, export, and transform resume data consistently.

## Product/Service Offering

### Core Components:

1. **JSON Schema:** Standardized format for resume data with defined fields
2. **Reference Implementation:** JavaScript library for working with JSON Resume data
3. **Theme System:** Collection of themes for rendering resumes in different formats
4. **CLI Tool:** Command-line interface for managing JSON Resume files
5. **Registry:** Public registry of JSON Resume themes and tools

### Key Features:

- **Structured Data:** Resumes stored as JSON with standardized fields
- **Portability:** Data can be easily moved between systems
- **Version Control:** Resume data can be tracked in Git
- **Programmability:** Data can be manipulated programmatically
- **Multi-Format Export:** Generate PDF, HTML, Markdown from single source
- **Theme Support:** Multiple visual themes available
- **Extensible:** Custom fields can be added while maintaining compatibility

**USP:** "An open source initiative to create a JSON-based standard for resumes" - JSON Resume provides the foundation for portable, programmable professional data.

## Technical Implementation

### Schema Structure:
```json
{
  "basics": {
    "name": "John Doe",
    "label": "Programmer",
    "image": "",
    "email": "john@gmail.com",
    "phone": "(912) 555-4321",
    "url": "https://johndoe.com",
    "summary": "A summary of John Doe...",
    "location": {
      "address": "2712 Broadway St",
      "postalCode": "CA 94115",
      "city": "San Francisco",
      "countryCode": "US",
      "region": "California"
    },
    "profiles": []
  },
  "work": [],
  "volunteer": [],
  "education": [],
  "awards": [],
  "certificates": [],
  "publications": [],
  "skills": [],
  "languages": [],
  "interests": [],
  "references": [],
  "projects": []
}
```

### Ecosystem:
- **100+ Themes:** Available for rendering resumes
- **Multiple Languages:** Implementations in JavaScript, Python, Ruby, Go, etc.
- **Integrations:** Plugins for various platforms and tools
- **Community Tools:** Resume parsers, validators, and converters

## Business Model

### Open Source Project:
- **Free to use:** No licensing costs
- **Community-driven:** Development through GitHub contributions
- **No monetization:** Pure open-source initiative
- **Sponsorship:** Some corporate sponsorship and donations

### Commercial Ecosystem:
- **Consulting Services:** Companies offer JSON Resume implementation services
- **Hosted Solutions:** Some providers offer managed JSON Resume services
- **Premium Themes:** Some theme developers sell premium designs
- **Integration Services:** Companies build JSON Resume support into their products

## Market Adoption

### Strengths

- **Open Standard:** Vendor-neutral format for professional data
- **Developer-Friendly:** Easy to work with programmatically
- **Portability:** Data can be moved between systems easily
- **Version Control:** Resume data can be tracked in Git repositories
- **Extensible:** Can be adapted to various use cases
- **Community Support:** Active developer community
- **Industry Recognition:** Used by major companies for resume processing

### Weaknesses

- **Technical Barrier:** Requires comfort with JSON and command-line tools
- **Limited Adoption:** Most professionals still use traditional formats
- **No GUI Editor:** Primarily developer-focused tooling
- **No Hosting:** Just a data format, not a complete solution
- **Schema Limitations:** Fixed structure may not fit all professional scenarios
- **No Agent Capabilities:** Purely a data format, not an active system

## Relevance to libre-cv

| Dimension | JSON Resume | libre-cv |
|---|---|---|
| Data Format | Structured JSON | Structured data with agent capabilities |
| Open Source | Yes (MIT) | Yes (libre philosophy) |
| Portability | High (JSON standard) | High (self-hosted) |
| Extensibility | Schema-based | Agent-based |
| Agent Capabilities | None | Autonomous agents |
| Business Model | Open source | Self-hosted framework |
| Target Audience | Developers | All professionals |

JSON Resume represents a crucial foundation that libre-cv can build upon:

### What libre-cv Can Learn from JSON Resume:

1. **Structured Data Approach:** The value of standardized, machine-readable professional data
2. **Portability Benefits:** How open data formats enable ecosystem growth
3. **Developer Adoption:** The importance of developer-friendly tooling
4. **Extensibility:** Balancing standardization with customization needs
5. **Community Building:** How open standards foster ecosystem development

### How libre-cv Extends JSON Resume:

1. **From Data to Identity:** Moves beyond static resume data to living professional identities
2. **Agent Capabilities:** Adds autonomous representation that can act on behalf of professionals
3. **Cross-Industry Appeal:** Makes structured professional data accessible to non-developers
4. **Complete Solution:** Provides both data format and execution framework
5. **Active Representation:** Professional identities that work proactively, not just sit as data

### Potential Integration:

libre-cv could potentially:
- **Support JSON Resume import/export** for compatibility
- **Extend the JSON Resume schema** for agent-specific capabilities
- **Contribute to JSON Resume ecosystem** with libre-cv tools
- **Use JSON Resume as a data layer** while adding agent functionality

JSON Resume validates the core insight that **professional data should be structured, portable, and programmable** - libre-cv takes this principle to its logical conclusion by making that data **active and autonomous** through agent representation.

The relationship could be seen as:
- **JSON Resume:** The data layer (what you are)
- **libre-cv:** The execution layer (what you can do)

Together, they represent a complete vision for the future of professional identity - structured data that can actively represent individuals across various professional contexts.
