# Maintainers

This document defines the governance structure and maintainer roles for the NUJ Social Media Ethics Monitor project.

## Current Maintainers

### Core Team (Perimeter 1)

#### Project Lead
**Jonathan** (NUJ)
- Role: Product owner, strategic direction
- Responsibilities: Stakeholder liaison, budget approval, feature prioritization
- Contact: [Contact via NUJ]
- Since: 2025-11-22
- Permissions: Full repository access, release authority

#### Technical Lead
**[Vacant]**
- Role: Technical architecture, code quality
- Responsibilities: Architecture decisions, code review, technical debt management
- Since: [Not yet appointed]
- Permissions: Write access to main, merge authority

### Trusted Contributors (Perimeter 2)

**[Vacant - recruiting]**
- We are seeking 5-10 trusted contributors
- Requirements:
  - 3+ merged pull requests
  - 6+ months active participation
  - Demonstrated understanding of codebase
  - Alignment with union values

### Community Contributors (Perimeter 3)

All contributors welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Roles and Responsibilities

### Project Lead
- Define product roadmap
- Approve major architectural changes
- Manage stakeholder relationships (NUJ, comms team, ICT)
- Budget allocation (£400/year OpenAI API)
- Represent project to NUJ membership
- Final arbiter on conflicts

### Technical Lead
- Review and merge pull requests
- Maintain code quality standards
- Coordinate technical discussions
- Mentor contributors
- Ensure test coverage
- Manage CI/CD pipeline
- Security vulnerability response

### Service Maintainers

#### Collector Service (Rust)
**[Vacant]**
- Rust expertise required
- Responsibilities: Platform monitoring, scraping, change detection

#### Analyzer Service (ReScript + Deno)
**[Vacant]**
- ReScript/Deno + NLP expertise required
- Responsibilities: GPT-4 integration, self-tuning ML, guidance generation

#### Publisher Service (Deno)
**[Vacant]**
- Deno expertise required
- Responsibilities: Email delivery, safety guardrails, rollback system

#### Dashboard Service (Elixir)
**[Vacant]**
- Elixir/Phoenix expertise required
- Responsibilities: Real-time UI, approval workflows, user management

#### Database Team
**[Vacant]**
- Database expertise (Virtuoso, XTDB, Dragonfly)
- Responsibilities: Schema management, query optimization, backups

#### Infrastructure Team
**[Vacant]**
- DevOps expertise required
- Responsibilities: Selur/Chainguard orchestration, monitoring, deployments

### Specialists

#### Security Officer
**[Vacant]**
- Responsibilities:
  - Vulnerability response (security@nuj.org.uk)
  - Security audits
  - Dependency management
  - GDPR compliance

#### Documentation Lead
**[Vacant]**
- Responsibilities:
  - Keep documentation current
  - Onboarding guides
  - API documentation
  - User manuals for comms team

#### Community Manager
**[Vacant]**
- Responsibilities:
  - Code of Conduct enforcement (conduct@nuj.org.uk)
  - Contributor onboarding
  - GitHub Discussions moderation
  - Community health metrics

## Decision-Making Process

### Consensus-Seeking

We use **lazy consensus**:
1. Proposal made (GitHub issue or discussion)
2. Minimum 72-hour comment period
3. If no objections, proposal accepted
4. Maintainers can call for vote if needed

### Voting

When consensus fails:
- **Technical decisions**: Technical Lead decides
- **Product decisions**: Project Lead decides
- **Major architectural changes**: Requires 2/3 Core Team approval
- **Code of Conduct enforcement**: Requires unanimous Core Team agreement

### Veto Power

Project Lead has veto power on:
- Decisions affecting budget
- Decisions affecting NUJ relationship
- Decisions affecting member privacy/safety

Veto must be exercised within 7 days and requires written justification.

## Contribution Tiers (TPCF)

### Perimeter 3 → Perimeter 2 (Community → Trusted)

Requirements:
- [ ] 3+ merged pull requests
- [ ] 6+ months active participation
- [ ] No Code of Conduct violations
- [ ] Demonstrated technical competence
- [ ] Alignment with union values
- [ ] Reference from existing Perimeter 2+ member

Process:
1. Self-nomination or nomination by existing maintainer
2. 14-day community discussion period
3. Approval by 2/3 of Perimeter 1 members

Benefits:
- Pull request review authority
- Can trigger CI/CD pipelines
- Listed in MAINTAINERS.md
- Invited to private maintainer discussions

### Perimeter 2 → Perimeter 1 (Trusted → Core)

Requirements:
- [ ] 12+ months as Perimeter 2
- [ ] 20+ merged pull requests
- [ ] Demonstrated leadership
- [ ] Deep codebase knowledge
- [ ] Community trust
- [ ] Union membership or strong alignment with union values

Process:
1. Nomination by existing Perimeter 1 member
2. 30-day community discussion period
3. Unanimous approval by existing Perimeter 1 members
4. Background check (NUJ requirement)

Benefits:
- Write access to main branch
- Release management authority
- Full repository administration
- Voting rights on major decisions
- Stipend consideration (if budget available)

## Stepping Down

Maintainers may step down:
- **Voluntary**: Email project lead with 30-day notice
- **Inactive**: No activity for 6+ months → automatic demotion to previous tier
- **Code of Conduct violation**: Immediate removal upon investigation

Emeritus status available for long-term contributors who step down in good standing.

## Maintainer Emeritus

Former maintainers who contributed significantly:
- Retain credit in MAINTAINERS.md
- Listed in humans.txt
- Can return to active status with community approval
- No active permissions

## Recruiting

We are actively recruiting for:
- [ ] Technical Lead (Perimeter 1)
- [ ] Rust expert (Perimeter 2)
- [ ] ReScript/Deno/NLP expert (Perimeter 2)
- [ ] Deno expert (Perimeter 2)
- [ ] Elixir/Phoenix expert (Perimeter 2)
- [ ] Database administrator (Perimeter 2)
- [ ] DevOps engineer (Perimeter 2)
- [ ] Security expert (Specialist)
- [ ] Documentation writer (Specialist)

Interested? Email: contributors@nuj.org.uk

## Compensation

### Current Status: Volunteer

All roles are currently unpaid volunteer positions.

### Future Possibilities

If revenue targets achieved (£30k/year):
- Stipends for Core Team (£500-2000/year)
- Bug bounty program
- Conference travel support
- Hardware/tool reimbursement

Governed by NUJ's financial policies.

## Meetings

### Core Team Sync
- **Frequency**: Weekly (30 min)
- **Format**: Video call
- **Agenda**: Blockers, decisions, upcoming work

### All-Maintainers Meeting
- **Frequency**: Monthly (60 min)
- **Format**: Video call + notes in GitHub Discussions
- **Agenda**: Roadmap, community health, technical debt

### Community Office Hours
- **Frequency**: Biweekly (60 min)
- **Format**: Open video call
- **Purpose**: Q&A, mentoring, collaboration

All meetings publicly logged in GitHub Discussions.

## Contact

- **General inquiries**: maintainers@nuj.org.uk
- **Security issues**: security@nuj.org.uk
- **Code of Conduct**: conduct@nuj.org.uk
- **Contribution questions**: contributors@nuj.org.uk

## Changes to This Document

Amendments require:
1. Pull request proposing changes
2. 14-day comment period
3. Approval by 2/3 of Core Team

**Version**: 1.0
**Last updated**: 2025-11-22
**Next review**: 2026-05-22 (6 months)
