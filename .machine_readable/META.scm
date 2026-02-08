;; SPDX-License-Identifier: PMPL-1.0-or-later
;; Social Media Tools - Meta Information
;; Updated: 2026-02-08

(meta
  (metadata
    (version "1.0")
    (name "social-media-tools")
    (type (quote monorepo))
    (last-updated "2026-02-08"))
  (languages
    ("rust" "rescript"))
  (architecture-decisions
    ((id "ADR-001")
     (title "Monorepo structure for social media tools")
     (status (quote accepted))
     (rationale "Shared analysis libraries and consistent interfaces across tools"))
    ((id "ADR-002")
     (title "Rust + ReScript stack")
     (status (quote accepted))
     (rationale "Rust for performance-critical analysis, ReScript for type-safe UI components")))
  (development-practices
    ((practice "RSR compliance")
     (status (quote active))
     (description "Repository follows hyperpolymath RSR template standards")))
  (design-rationale
    ((area "Content analysis")
     (rationale "Focus on scalable, privacy-respecting analysis tools for researchers"))))
