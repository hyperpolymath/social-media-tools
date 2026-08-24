# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- Real fact-checking API integrations (Google Fact Check, Snopes, PolitiFact)
- Multi-language support
- Real-time claim monitoring
- Webhook support for notifications
- PDF/CSV export functionality
- Enhanced ML models with fine-tuning
- Mobile applications
- Advanced analytics dashboard

## [0.1.0] - 2024-01-15

### Added

#### Backend
- FastAPI RESTful API with async support
- ArangoDB multi-model database integration
  - Document storage for claims, sources, users
  - Graph relationships for knowledge graph
  - Collections: claims, sources, fact_checks, users, api_keys
- XTDB temporal database for claim history tracking
- Dragonfly cache for high-performance caching
- NLP pipeline with spaCy
  - Entity extraction
  - Sentiment analysis with TextBlob
  - Text complexity analysis
  - Language detection
- Credibility scoring algorithm
  - Source credibility scoring
  - Claim credibility scoring
  - Bias detection
- Fact-checking service framework (mock implementations)
- JWT authentication system
- API key management
- Rate limiting with slowapi
- Prometheus metrics integration
- Comprehensive error handling
- Logging with loguru
- Test suite with pytest
  - Unit tests for NLP processor
  - Unit tests for credibility scorer
  - Integration tests for API endpoints
  - Test fixtures and mocks

#### Frontend
- React 18 application with TypeScript
- Vite build system
- TailwindCSS styling
- React Query for data fetching
- React Router for navigation
- Pages:
  - Home page with feature overview
  - Verify page for claim submission
  - Claim detail page with full analysis
  - About page with project information
- Components:
  - Layout with header and footer
  - VerificationResult display component
- API client service
- TypeScript types and interfaces
- Responsive design
- Form validation with react-hook-form

#### Browser Extension
- Manifest V3 Chrome/Firefox extension
- Background service worker
- Content scripts for social media integration
  - Twitter/X support
  - Facebook placeholder
  - Instagram placeholder
- Popup interface for quick verification
- Context menu integration
- Cross-platform compatibility

#### Infrastructure
- Podman containerization
  - Backend Containerfile with multi-stage build
  - Frontend Containerfile with nginx
- Podman Compose orchestration
  - ArangoDB service
  - XTDB service
  - Dragonfly service
  - Backend API service
  - Frontend service
- GitHub Actions CI/CD
  - Backend tests and linting
  - Frontend tests and type checking
  - Container building
  - Security scanning with Trivy
  - Deployment workflow
- Development scripts
  - start-dev.sh
  - stop-dev.sh

#### Documentation
- Comprehensive README with quick start
- API documentation (docs/API.md)
- Architecture documentation (docs/ARCHITECTURE.md)
- Deployment guide (docs/DEPLOYMENT.md)
- Contributing guidelines (CONTRIBUTING.md)
- Security policy (SECURITY.md)
- Code of Conduct (CODE_OF_CONDUCT.md)
- Maintainers guide (MAINTAINERS.md)
- Development summary
- MIT License
- .gitignore for Python, Node, containers

#### Configuration
- Environment variable configuration
- Poetry dependency management for Python
- npm package management for frontend
- TypeScript configuration
- ESLint and Prettier setup
- Pytest configuration
- Docker/Podman health checks

### Security
- JWT token-based authentication
- bcrypt password hashing
- API key authentication support
- Rate limiting (100 req/min default)
- CORS configuration
- Input validation with Pydantic
- SQL injection prevention (NoSQL)
- XSS prevention in frontend
- Security headers in nginx
- Non-root container users
- Secrets management via environment variables

### Developer Experience
- Type safety
  - Python type hints throughout
  - Full TypeScript typing in frontend
- Code quality tools
  - Black code formatting
  - Ruff linting
  - mypy type checking
  - ESLint for TypeScript
- Testing infrastructure
  - pytest with coverage
  - Test fixtures and mocks
  - Integration test framework
- Development workflow
  - Hot reload in development
  - Fast builds with Vite
  - One-command startup
- Documentation
  - Inline code comments
  - API documentation
  - Architecture diagrams
  - Deployment guides

## Version History

### Version Numbering

We use [Semantic Versioning](https://semver.org/):

- **MAJOR** version for incompatible API changes
- **MINOR** version for backwards-compatible functionality additions
- **PATCH** version for backwards-compatible bug fixes

### Release Schedule

- **Major releases**: Annually (breaking changes)
- **Minor releases**: Quarterly (new features)
- **Patch releases**: As needed (bug fixes, security)

### Upgrade Guides

For breaking changes, see `docs/UPGRADE.md` (to be created).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for how to contribute to this project.

## Links

- [Repository](https://github.com/hyperpolymath/social-media-polygraph)
- [Issue Tracker](https://github.com/hyperpolymath/social-media-polygraph/issues)
- [Security Policy](SECURITY.md)
- [Code of Conduct](CODE_OF_CONDUCT.md)
