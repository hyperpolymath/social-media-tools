;; SPDX-License-Identifier: PMPL-1.0-or-later
;; Social Media Tools - Ecosystem Position
;; Updated: 2026-02-08

(ecosystem
  (metadata
    (version "1.0")
    (name "social-media-tools")
    (last-updated "2026-02-08"))
  (type (quote analysis-tools))
  (purpose "Social media content analysis and fact-checking")
  (position-in-ecosystem
    (role (quote research-tools))
    (tier (quote application)))
  (related-projects
    ((name "data-science-tools")
     (relationship (quote sibling-standard))
     (interaction "Shared analysis algorithms and data pipelines"))
    ((name "content-moderation")
     (relationship (quote potential-consumer))
     (interaction "May consume verification APIs"))))
