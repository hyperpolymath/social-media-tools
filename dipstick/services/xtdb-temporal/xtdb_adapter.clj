;; XTDB Temporal Database Adapter for NUJ Monitor
;; Provides bitemporal query support for policy change tracking

(ns nuj-monitor.xtdb-adapter
  (:require [xtdb.api :as xt]
            [clojure.instant :as inst]))

;; XTDB Node Configuration
(def xtdb-node-config
  {:xtdb/tx-log {:kv-store {:xtdb/module 'xtdb.rocksdb/->kv-store
                            :db-dir "data/tx-log"}}
   :xtdb/document-store {:kv-store {:xtdb/module 'xtdb.rocksdb/->kv-store
                                     :db-dir "data/documents"}}
   :xtdb/index-store {:kv-store {:xtdb/module 'xtdb.rocksdb/->kv-store
                                  :db-dir "data/indexes"}}})

;; Initialize XTDB node
(defonce xtdb-node (xt/start-node xtdb-node-config))

;; Document schemas
(defn platform-doc [id name display-name api-enabled monitoring-active]
  {:xt/id id
   :type :platform
   :name name
   :display-name display-name
   :api-enabled api-enabled
   :monitoring-active monitoring-active
   :created-at (java.util.Date.)})

(defn policy-change-doc [id platform-id policy-doc-id severity confidence
                         summary impact requires-notification]
  {:xt/id id
   :type :policy-change
   :platform-id platform-id
   :policy-document-id policy-doc-id
   :severity severity
   :confidence-score confidence
   :change-summary summary
   :impact-assessment impact
   :requires-notification requires-notification
   :detected-at (java.util.Date.)})

(defn guidance-draft-doc [id title summary content status ai-generated]
  {:xt/id id
   :type :guidance-draft
   :title title
   :summary summary
   :content content
   :status status
   :ai-generated ai-generated
   :drafted-at (java.util.Date.)})

;; Write operations
(defn put-document!
  "Insert or update a document with bitemporal validity"
  ([doc]
   (xt/submit-tx xtdb-node [[::xt/put doc]]))
  ([doc valid-time]
   (xt/submit-tx xtdb-node [[::xt/put doc valid-time]])))

(defn delete-document!
  "Delete a document (creates tombstone, preserves history)"
  [id]
  (xt/submit-tx xtdb-node [[::xt/delete id]]))

(defn evict-document!
  "Permanently remove document and all history (GDPR compliance)"
  [id]
  (xt/submit-tx xtdb-node [[::xt/evict id]]))

;; Query operations
(defn get-document
  "Get current version of document"
  [id]
  (xt/entity (xt/db xtdb-node) id))

(defn get-document-at-time
  "Get document as it was at a specific valid-time"
  [id valid-time]
  (xt/entity (xt/db xtdb-node valid-time) id))

(defn query-recent-changes
  "Query recent policy changes with temporal support"
  [days]
  (let [since (inst/read-instant-date
                (str (- (System/currentTimeMillis) (* days 24 60 60 1000))))]
    (xt/q (xt/db xtdb-node)
          '{:find [?e ?severity ?confidence ?summary ?detected-at]
            :where [[?e :type :policy-change]
                    [?e :severity ?severity]
                    [?e :confidence-score ?confidence]
                    [?e :change-summary ?summary]
                    [?e :detected-at ?detected-at]
                    [(> ?detected-at since)]]
            :in [since]}
          since)))

(defn query-platform-changes
  "Get all changes for a platform across time"
  [platform-id]
  (xt/q (xt/db xtdb-node)
        '{:find [?e ?severity ?detected-at]
          :where [[?e :type :policy-change]
                  [?e :platform-id platform-id]
                  [?e :severity ?severity]
                  [?e :detected-at ?detected-at]]
          :in [platform-id]
          :order-by [[?detected-at :desc]]}
        platform-id))

(defn query-changes-between-times
  "Bitemporal query: changes detected between two time points"
  [start-time end-time]
  (xt/q (xt/db xtdb-node end-time)
        '{:find [?e ?severity ?summary]
          :where [[?e :type :policy-change]
                  [?e :detected-at ?detected]
                  [?e :severity ?severity]
                  [?e :change-summary ?summary]
                  [(>= ?detected start-time)]
                  [(<= ?detected end-time)]]
          :in [start-time end-time]}
        start-time end-time))

(defn get-document-history
  "Get full history of a document (all versions)"
  [id]
  (xt/entity-history (xt/db xtdb-node) id :asc
                     {:with-docs? true}))

(defn query-critical-changes-timeline
  "Get timeline of critical changes"
  []
  (xt/q (xt/db xtdb-node)
        '{:find [?e ?platform-id ?summary (pull ?e [*])]
          :where [[?e :type :policy-change]
                  [?e :severity "critical"]
                  [?e :platform-id ?platform-id]
                  [?e :change-summary ?summary]]
          :order-by [[?detected-at :desc]]}))

;; Temporal analytics
(defn count-changes-by-severity-over-time
  "Count changes by severity in time buckets"
  [bucket-days]
  (xt/q (xt/db xtdb-node)
        '{:find [?severity (count ?e)]
          :where [[?e :type :policy-change]
                  [?e :severity ?severity]]
          :group-by [?severity]}))

(defn find-patterns-in-changes
  "Find recurring patterns in policy changes using temporal queries"
  []
  (xt/q (xt/db xtdb-node)
        '{:find [?platform-id (count ?e) (avg ?confidence)]
          :where [[?e :type :policy-change]
                  [?e :platform-id ?platform-id]
                  [?e :confidence-score ?confidence]]
          :group-by [?platform-id]}))

;; Audit trail support
(defn get-change-audit-trail
  "Get complete audit trail for a policy change"
  [change-id]
  (let [history (xt/entity-history (xt/db xtdb-node) change-id :asc
                                   {:with-docs? true})]
    (map (fn [{:keys [xtdb.api/tx-time xtdb.api/tx-id xtdb.api/valid-time
                      xtdb.api/content-hash xtdb.db/doc]}]
           {:tx-time tx-time
            :tx-id tx-id
            :valid-time valid-time
            :content-hash content-hash
            :document doc})
         history)))

;; Time travel query interface
(defn as-of-query
  "Run query as of a specific point in time"
  [query-map as-of-time]
  (xt/q (xt/db xtdb-node as-of-time) query-map))

(defn between-times-query
  "Run query for data valid between two times (bitemporal)"
  [query-map start-time end-time]
  (xt/q (xt/db xtdb-node end-time) query-map))

;; Export and backup
(defn export-all-data
  "Export all documents (for backup)"
  []
  (let [db (xt/db xtdb-node)]
    (xt/q db '{:find [(pull ?e [*])]
               :where [[?e :type _]]})))

(defn snapshot-at-time
  "Create snapshot of entire database at specific time"
  [snapshot-time]
  (let [db (xt/db xtdb-node snapshot-time)]
    (xt/q db '{:find [(pull ?e [*])]
               :where [[?e :xt/id _]]})))

;; Initialize with sample data
(defn initialize-sample-data! []
  (put-document! (platform-doc :twitter "twitter" "X (Twitter)" true true))
  (put-document! (platform-doc :facebook "facebook" "Facebook" true true))
  (put-document! (policy-change-doc
                   :change-001
                   :twitter
                   :policy-twitter-tos
                   "critical"
                   0.92
                   "New restrictions on journalist accounts"
                   "May impact news gathering"
                   true)))

;; Shutdown
(defn close-xtdb! []
  (.close xtdb-node))

;; Example usage
(comment
  ;; Initialize
  (initialize-sample-data!)

  ;; Query current state
  (get-document :twitter)

  ;; Time travel: what did Twitter policy look like 30 days ago?
  (get-document-at-time :twitter (inst/read-instant-date "2024-10-22T00:00:00Z"))

  ;; Get all critical changes
  (query-critical-changes-timeline)

  ;; Audit trail for a change
  (get-change-audit-trail :change-001)

  ;; Changes in last 7 days
  (query-recent-changes 7)

  ;; Shutdown
  (close-xtdb!)
)
