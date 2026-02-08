;; SPDX-License-Identifier: PMPL-1.0-or-later
;; Social Media Tools - Project State
;; Updated: 2026-02-08

(state
  (metadata
    (version "1.0")
    (name "social-media-tools")
    (last-updated "2026-02-08")
    (status (quote active)))
  (project-context
    (purpose "Social media analysis and verification tools for researchers and content moderators")
    (type (quote monorepo))
    (completion-percentage 30))
  (components
    ((name "dipstick")
     (status (quote active))
     (completion 40)
     (description "Social media content quality analysis tool"))
    ((name "polygraph")
     (status (quote active))
     (completion 20)
     (description "Fact-checking and verification framework")))
  (current-position
    (phase (quote early-development))
    (milestone "Core analysis engine"))
  (route-to-mvp
    ((milestone "Complete dipstick analysis pipeline") (priority (quote high)) (status (quote in-progress)))
    ((milestone "Implement polygraph verification rules") (priority (quote high)) (status (quote planned)))
    ((milestone "Add data source integrations") (priority (quote medium)) (status (quote planned)))
    ((milestone "Build reporting dashboard") (priority (quote medium)) (status (quote planned))))
  (blockers-and-issues
    ())
  (critical-next-actions
    ("Finish content analysis algorithms")
    ("Design verification rule DSL")
    ("Add API client for major platforms")))
