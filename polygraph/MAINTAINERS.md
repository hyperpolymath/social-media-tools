# Maintainers

This document lists the maintainers of the Social Media Polygraph project and defines the governance structure.

## Current Maintainers

### Lead Maintainers

- **[Your Name]** (@yourusername)
  - **Role**: Project Lead, Architecture
  - **Focus**: Overall vision, architecture decisions, major releases
  - **Timezone**: UTC-8
  - **Contact**: lead@yourdomain.com

### Core Maintainers

<!-- Add core maintainers as project grows -->

### Component Maintainers

#### Backend (Python/FastAPI)
- **[Maintainer Name]** (@username) - API & Services
- **[Maintainer Name]** (@username) - ML/NLP Pipeline
- **[Maintainer Name]** (@username) - Database Integration

#### Frontend (React/TypeScript)
- **[Maintainer Name]** (@username) - UI Components
- **[Maintainer Name]** (@username) - State Management

#### Infrastructure
- **[Maintainer Name]** (@username) - CI/CD & Deployment
- **[Maintainer Name]** (@username) - Container Orchestration

#### Browser Extension
- **[Maintainer Name]** (@username) - Extension Development

## Governance Model

### Tri-Perimeter Contribution Framework (TPCF)

This project uses a **graduated trust model** with three perimeters:

#### Perimeter 1: Core (Maintainers Only)

**Access Level**: Write access to main/production branches

**Responsibilities**:
- Review and merge pull requests
- Make architectural decisions
- Manage releases and versioning
- Security vulnerability response
- Community moderation

**Requirements**:
- 6+ months of active contribution
- Deep understanding of codebase
- Demonstrated judgment and responsibility
- Unanimous approval from existing core maintainers

**Current Members**: Listed above as Lead/Core Maintainers

#### Perimeter 2: Trusted Contributors

**Access Level**: Write access to development branches, auto-approved CI/CD

**Responsibilities**:
- Major feature development
- Significant refactoring
- Documentation improvements
- Mentoring new contributors

**Requirements**:
- 3+ months of active contribution
- 10+ merged pull requests
- Demonstrated code quality and testing practices
- Approval from 2+ core maintainers

**Current Members**: (To be populated as project grows)

#### Perimeter 3: Community Sandbox

**Access Level**: Fork-based contributions, public issues/discussions

**Responsibilities**:
- Bug reports and feature requests
- Pull requests (require review)
- Documentation fixes
- Community support

**Requirements**: None - open to all

**Process**:
1. Fork repository
2. Make changes in your fork
3. Submit pull request
4. Address review feedback
5. Maintainer merges when approved

## Decision Making

### Consensus-Based

- **Small changes**: Any maintainer can merge after review
- **Medium changes**: Requires approval from component maintainer
- **Large changes**: Requires approval from 2+ core maintainers
- **Breaking changes**: Requires approval from lead maintainer + 2+ core
- **Governance changes**: Requires unanimous approval from all core maintainers

### Conflict Resolution

1. **Discussion**: Try to reach consensus through discussion
2. **Mediation**: Lead maintainer mediates if needed
3. **Vote**: Core maintainers vote (simple majority)
4. **Final Decision**: Lead maintainer has final say in deadlocks

## Maintainer Responsibilities

### Code Review

- Respond to PRs within 3 business days
- Provide constructive, actionable feedback
- Test changes locally when needed
- Ensure CI/CD passes before merging

### Security

- Respond to security reports within 48 hours
- Coordinate security fixes and disclosures
- Monitor dependency vulnerabilities
- Keep security policy up to date

### Community

- Welcome new contributors
- Answer questions in issues/discussions
- Enforce Code of Conduct fairly
- Recognize and appreciate contributions

### Release Management

- Follow semantic versioning (SemVer)
- Maintain CHANGELOG.md
- Test releases thoroughly
- Coordinate with dependent projects

### Documentation

- Keep README and guides up to date
- Document breaking changes clearly
- Maintain API documentation
- Write migration guides when needed

## Becoming a Maintainer

### Path to Trusted Contributor (Perimeter 2)

1. **Contribute regularly** for 3+ months
2. **Submit quality PRs**: Well-tested, documented, follows guidelines
3. **Engage with community**: Help others, review PRs, participate in discussions
4. **Request nomination**: Ask an existing maintainer or self-nominate
5. **Review period**: Core maintainers review contributions
6. **Approval**: 2+ core maintainers approve
7. **Onboarding**: Access granted, added to this document

### Path to Core Maintainer (Perimeter 1)

1. **Serve as Trusted Contributor** for 6+ months
2. **Deep expertise**: Demonstrate mastery of component/area
3. **Leadership**: Mentor others, drive initiatives
4. **Request nomination**: Self-nominate or be nominated
5. **Review period**: All core maintainers review
6. **Approval**: Unanimous approval required
7. **Onboarding**: Write access, voting rights, added to this document

## Emeritus Status

Maintainers who step down remain listed as **Emeritus Maintainers**:

### Emeritus Maintainers

<!-- None yet -->

**Rights**:
- Honorary recognition
- Advisory role (non-binding)
- Can return to active status if desired

**Process to Return**:
1. Notify current core maintainers
2. Review recent changes
3. 1-month trial period
4. Core maintainers approve return

## Maintainer Expectations

### Time Commitment

- **Core Maintainers**: 5-10 hours/week
- **Component Maintainers**: 3-5 hours/week
- **Emeritus**: No commitment

### Availability

- Respond to critical issues within 24 hours
- Participate in monthly maintainer calls (when established)
- Give notice for extended absences (2+ weeks)

### Professional Conduct

- Follow Code of Conduct
- Maintain confidentiality of security issues
- Disclose conflicts of interest
- Represent project professionally

## Contact

- **General Maintainer Contact**: maintainers@yourdomain.com
- **Security Issues**: security@yourdomain.com (see SECURITY.md)
- **Code of Conduct Issues**: conduct@yourdomain.com (see CODE_OF_CONDUCT.md)

## Changes to This Document

This document is versioned and changes require:
- Pull request with clear rationale
- Approval from all core maintainers
- 7-day comment period for community feedback

**Version**: 1.0
**Last Updated**: 2024-01-15
**Next Review**: 2024-07-15
