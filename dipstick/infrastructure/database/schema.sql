-- NUJ Social Media Ethics Monitor - Database Schema (SurrealDB/VerisimDB equivalent)
-- Captures the table/stream definitions that map directly to SurrealDB collections and VerisimDB time-series
-- Purpose: Track social media platform policy changes and member guidance

-- ====================
-- CORE TABLES
-- ====================

-- Platforms being monitored
CREATE TABLE platforms (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(100) NOT NULL UNIQUE,
    display_name VARCHAR(255) NOT NULL,
    url TEXT NOT NULL,
    api_endpoint TEXT,
    api_enabled BOOLEAN DEFAULT false,
    scraping_enabled BOOLEAN DEFAULT true,
    monitoring_active BOOLEAN DEFAULT true,
    check_frequency_minutes INTEGER DEFAULT 60,
    policy_urls JSONB NOT NULL DEFAULT '[]'::jsonb,
    terms_urls JSONB NOT NULL DEFAULT '[]'::jsonb,
    community_guidelines_urls JSONB NOT NULL DEFAULT '[]'::jsonb,
    metadata JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by VARCHAR(255),
    CONSTRAINT valid_check_frequency CHECK (check_frequency_minutes > 0)
);

CREATE INDEX idx_platforms_active ON platforms(monitoring_active) WHERE monitoring_active = true;
CREATE INDEX idx_platforms_name ON platforms(name);

-- Platform credentials (encrypted)
CREATE TABLE platform_credentials (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    platform_id UUID NOT NULL REFERENCES platforms(id) ON DELETE CASCADE,
    credential_type VARCHAR(50) NOT NULL, -- 'api_key', 'oauth_token', 'bearer_token'
    encrypted_value TEXT NOT NULL,
    expires_at TIMESTAMPTZ,
    last_used_at TIMESTAMPTZ,
    is_active BOOLEAN DEFAULT true,
    metadata JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT unique_platform_credential_type UNIQUE(platform_id, credential_type)
);

CREATE INDEX idx_platform_credentials_active ON platform_credentials(platform_id, is_active);

-- Policy documents tracked
CREATE TABLE policy_documents (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    platform_id UUID NOT NULL REFERENCES platforms(id) ON DELETE CASCADE,
    document_type VARCHAR(100) NOT NULL, -- 'terms', 'privacy', 'community_guidelines', 'content_policy'
    url TEXT NOT NULL,
    title VARCHAR(500),
    language VARCHAR(10) DEFAULT 'en',
    version VARCHAR(100),
    is_current BOOLEAN DEFAULT true,
    discovered_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    first_seen_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_seen_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    archived_at TIMESTAMPTZ,
    checksum VARCHAR(64), -- SHA256 hash
    metadata JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_policy_documents_platform ON policy_documents(platform_id, is_current);
CREATE INDEX idx_policy_documents_type ON policy_documents(document_type);
CREATE INDEX idx_policy_documents_checksum ON policy_documents(checksum);
CREATE INDEX idx_policy_documents_url ON policy_documents USING gin(to_tsvector('english', url));

-- Policy snapshots (VerisimDB time-series stream definition)
CREATE TABLE policy_snapshots (
    id UUID DEFAULT uuid_generate_v4(),
    policy_document_id UUID NOT NULL REFERENCES policy_documents(id) ON DELETE CASCADE,
    captured_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    content_text TEXT NOT NULL,
    content_html TEXT,
    content_markdown TEXT,
    word_count INTEGER,
    char_count INTEGER,
    checksum VARCHAR(64) NOT NULL,
    previous_snapshot_id UUID REFERENCES policy_snapshots(id),
    diff_summary JSONB,
    capture_method VARCHAR(50) NOT NULL, -- 'api', 'scraper', 'archive', 'manual'
    metadata JSONB DEFAULT '{}'::jsonb,
    PRIMARY KEY (id, captured_at)
);

CREATE INDEX idx_policy_snapshots_document ON policy_snapshots(policy_document_id, captured_at DESC);
CREATE INDEX idx_policy_snapshots_checksum ON policy_snapshots(checksum);

-- Policy changes detected
CREATE TABLE policy_changes (
    id UUID DEFAULT uuid_generate_v4(),
    policy_document_id UUID NOT NULL REFERENCES policy_documents(id) ON DELETE CASCADE,
    previous_snapshot_id UUID REFERENCES policy_snapshots(id),
    current_snapshot_id UUID REFERENCES policy_snapshots(id),
    detected_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    change_type VARCHAR(50) NOT NULL, -- 'addition', 'deletion', 'modification', 'structural'
    severity VARCHAR(20) DEFAULT 'unknown', -- 'critical', 'high', 'medium', 'low', 'unknown'
    confidence_score DECIMAL(3,2) DEFAULT 0.00 CHECK (confidence_score >= 0 AND confidence_score <= 1),
    affected_sections JSONB DEFAULT '[]'::jsonb,
    change_summary TEXT,
    impact_assessment TEXT,
    requires_member_notification BOOLEAN DEFAULT false,
    notification_sent_at TIMESTAMPTZ,
    reviewed_by VARCHAR(255),
    reviewed_at TIMESTAMPTZ,
    review_notes TEXT,
    false_positive BOOLEAN DEFAULT false,
    metadata JSONB DEFAULT '{}'::jsonb,
    PRIMARY KEY (id, detected_at)
);

CREATE INDEX idx_policy_changes_document ON policy_changes(policy_document_id, detected_at DESC);
CREATE INDEX idx_policy_changes_severity ON policy_changes(severity, requires_member_notification);
CREATE INDEX idx_policy_changes_unreviewed ON policy_changes(reviewed_at) WHERE reviewed_at IS NULL;
CREATE INDEX idx_policy_changes_notifications ON policy_changes(requires_member_notification, notification_sent_at)
    WHERE requires_member_notification = true AND notification_sent_at IS NULL;

-- NLP analysis results
CREATE TABLE nlp_analyses (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    policy_change_id UUID NOT NULL,
    detected_at TIMESTAMPTZ NOT NULL,
    analysis_type VARCHAR(100) NOT NULL, -- 'sentiment', 'topic', 'entity', 'impact'
    model_name VARCHAR(255),
    model_version VARCHAR(100),
    result JSONB NOT NULL,
    confidence_score DECIMAL(3,2) CHECK (confidence_score >= 0 AND confidence_score <= 1),
    processing_time_ms INTEGER,
    metadata JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (policy_change_id, detected_at) REFERENCES policy_changes(id, detected_at) ON DELETE CASCADE
);

CREATE INDEX idx_nlp_analyses_change ON nlp_analyses(policy_change_id);
CREATE INDEX idx_nlp_analyses_type ON nlp_analyses(analysis_type);

-- ====================
-- GUIDANCE & COMMUNICATION
-- ====================

-- Member guidance drafts
CREATE TABLE guidance_drafts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(500) NOT NULL,
    summary TEXT,
    content_markdown TEXT NOT NULL,
    content_html TEXT,
    draft_type VARCHAR(50) DEFAULT 'regular', -- 'regular', 'urgent', 'update', 'advisory'
    status VARCHAR(50) DEFAULT 'draft', -- 'draft', 'review', 'approved', 'published', 'archived'
    related_changes JSONB DEFAULT '[]'::jsonb, -- Array of policy_change IDs
    target_platforms JSONB DEFAULT '[]'::jsonb, -- Array of platform IDs
    generated_by VARCHAR(50), -- 'ai', 'human', 'hybrid'
    ai_model VARCHAR(255),
    drafted_by VARCHAR(255),
    drafted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    reviewed_by VARCHAR(255),
    reviewed_at TIMESTAMPTZ,
    approved_by VARCHAR(255),
    approved_at TIMESTAMPTZ,
    published_at TIMESTAMPTZ,
    archived_at TIMESTAMPTZ,
    metadata JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_guidance_drafts_status ON guidance_drafts(status, drafted_at DESC);
CREATE INDEX idx_guidance_drafts_approval ON guidance_drafts(approved_at) WHERE approved_at IS NOT NULL;
CREATE INDEX idx_guidance_drafts_review ON guidance_drafts(status) WHERE status = 'review';

-- Member segments for targeted communication
CREATE TABLE member_segments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    criteria JSONB NOT NULL, -- Filtering criteria
    member_count INTEGER DEFAULT 0,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Guidance publications
CREATE TABLE guidance_publications (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    guidance_draft_id UUID NOT NULL REFERENCES guidance_drafts(id) ON DELETE CASCADE,
    segment_id UUID REFERENCES member_segments(id),
    publication_channel VARCHAR(100) NOT NULL, -- 'email', 'website', 'api', 'slack'
    scheduled_for TIMESTAMPTZ NOT NULL,
    published_at TIMESTAMPTZ,
    recipients_count INTEGER DEFAULT 0,
    successful_deliveries INTEGER DEFAULT 0,
    failed_deliveries INTEGER DEFAULT 0,
    bounced_deliveries INTEGER DEFAULT 0,
    opened_count INTEGER DEFAULT 0,
    clicked_count INTEGER DEFAULT 0,
    grace_period_ends_at TIMESTAMPTZ, -- 5-minute grace period for rollback
    can_rollback BOOLEAN DEFAULT true,
    rolled_back_at TIMESTAMPTZ,
    rollback_reason TEXT,
    metadata JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_guidance_publications_draft ON guidance_publications(guidance_draft_id);
CREATE INDEX idx_guidance_publications_scheduled ON guidance_publications(scheduled_for, published_at);
CREATE INDEX idx_guidance_publications_rollback ON guidance_publications(can_rollback, grace_period_ends_at)
    WHERE can_rollback = true AND rolled_back_at IS NULL;

-- Individual delivery tracking
CREATE TABLE delivery_events (
    id UUID DEFAULT uuid_generate_v4(),
    publication_id UUID NOT NULL REFERENCES guidance_publications(id) ON DELETE CASCADE,
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    event_type VARCHAR(50) NOT NULL, -- 'sent', 'delivered', 'bounced', 'opened', 'clicked', 'complained'
    recipient_hash VARCHAR(64) NOT NULL, -- Hashed email for privacy
    metadata JSONB DEFAULT '{}'::jsonb,
    PRIMARY KEY (id, occurred_at)
);

CREATE INDEX idx_delivery_events_publication ON delivery_events(publication_id, occurred_at DESC);
CREATE INDEX idx_delivery_events_type ON delivery_events(event_type, occurred_at DESC);

-- ====================
-- SAFETY & AUDIT
-- ====================

-- Approval workflow
CREATE TABLE approval_requests (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    request_type VARCHAR(100) NOT NULL, -- 'guidance_publication', 'policy_override', 'emergency_action'
    related_entity_id UUID NOT NULL,
    requested_by VARCHAR(255) NOT NULL,
    requested_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    approved_by VARCHAR(255),
    approved_at TIMESTAMPTZ,
    rejected_by VARCHAR(255),
    rejected_at TIMESTAMPTZ,
    rejection_reason TEXT,
    status VARCHAR(50) DEFAULT 'pending', -- 'pending', 'approved', 'rejected', 'expired'
    expires_at TIMESTAMPTZ,
    metadata JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_approval_requests_status ON approval_requests(status, requested_at DESC);
CREATE INDEX idx_approval_requests_pending ON approval_requests(status, expires_at) WHERE status = 'pending';

-- Audit log for all system actions
CREATE TABLE audit_log (
    id UUID DEFAULT uuid_generate_v4(),
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    actor VARCHAR(255) NOT NULL,
    action VARCHAR(255) NOT NULL,
    entity_type VARCHAR(100),
    entity_id UUID,
    changes JSONB,
    ip_address INET,
    user_agent TEXT,
    session_id VARCHAR(255),
    success BOOLEAN DEFAULT true,
    error_message TEXT,
    metadata JSONB DEFAULT '{}'::jsonb,
    PRIMARY KEY (id, occurred_at)
);

CREATE INDEX idx_audit_log_actor ON audit_log(actor, occurred_at DESC);
CREATE INDEX idx_audit_log_entity ON audit_log(entity_type, entity_id, occurred_at DESC);
CREATE INDEX idx_audit_log_action ON audit_log(action, occurred_at DESC);
CREATE INDEX idx_audit_log_failures ON audit_log(success, occurred_at DESC) WHERE success = false;

-- System health metrics
CREATE TABLE system_metrics (
    id UUID DEFAULT uuid_generate_v4(),
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    service_name VARCHAR(100) NOT NULL,
    metric_name VARCHAR(255) NOT NULL,
    metric_value DECIMAL,
    metric_unit VARCHAR(50),
    tags JSONB DEFAULT '{}'::jsonb,
    PRIMARY KEY (id, recorded_at)
);

CREATE INDEX idx_system_metrics_service ON system_metrics(service_name, metric_name, recorded_at DESC);

-- ====================
-- CONFIGURATION & SETTINGS
-- ====================

-- System configuration
CREATE TABLE system_config (
    key VARCHAR(255) PRIMARY KEY,
    value JSONB NOT NULL,
    description TEXT,
    config_type VARCHAR(50) DEFAULT 'general', -- 'general', 'security', 'notification', 'monitoring'
    is_sensitive BOOLEAN DEFAULT false,
    updated_by VARCHAR(255),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- User roles and permissions
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) NOT NULL UNIQUE,
    username VARCHAR(255) NOT NULL UNIQUE,
    full_name VARCHAR(255),
    role VARCHAR(50) NOT NULL DEFAULT 'viewer', -- 'admin', 'editor', 'reviewer', 'viewer'
    is_active BOOLEAN DEFAULT true,
    last_login_at TIMESTAMPTZ,
    password_hash TEXT, -- For local auth (or NULL if using external auth)
    preferences JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_email ON users(email) WHERE is_active = true;
CREATE INDEX idx_users_role ON users(role) WHERE is_active = true;

-- User sessions
CREATE TABLE user_sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash VARCHAR(64) NOT NULL UNIQUE,
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL,
    last_activity_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    is_active BOOLEAN DEFAULT true
);

CREATE INDEX idx_user_sessions_user ON user_sessions(user_id, is_active);
CREATE INDEX idx_user_sessions_expires ON user_sessions(expires_at) WHERE is_active = true;

-- ====================
-- TRIGGERS & FUNCTIONS
-- ====================

-- Updated timestamp trigger function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Apply updated_at trigger to relevant tables
CREATE TRIGGER update_platforms_updated_at BEFORE UPDATE ON platforms
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_platform_credentials_updated_at BEFORE UPDATE ON platform_credentials
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_policy_documents_updated_at BEFORE UPDATE ON policy_documents
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_guidance_drafts_updated_at BEFORE UPDATE ON guidance_drafts
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_member_segments_updated_at BEFORE UPDATE ON member_segments
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_guidance_publications_updated_at BEFORE UPDATE ON guidance_publications
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_approval_requests_updated_at BEFORE UPDATE ON approval_requests
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_system_config_updated_at BEFORE UPDATE ON system_config
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Audit log trigger function
CREATE OR REPLACE FUNCTION create_audit_log()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO audit_log (
        actor,
        action,
        entity_type,
        entity_id,
        changes
    ) VALUES (
        COALESCE(current_setting('app.current_user', true), 'system'),
        TG_OP,
        TG_TABLE_NAME,
        COALESCE(NEW.id, OLD.id),
        CASE
            WHEN TG_OP = 'DELETE' THEN jsonb_build_object('old', to_jsonb(OLD))
            WHEN TG_OP = 'UPDATE' THEN jsonb_build_object('old', to_jsonb(OLD), 'new', to_jsonb(NEW))
            WHEN TG_OP = 'INSERT' THEN jsonb_build_object('new', to_jsonb(NEW))
        END
    );
    RETURN COALESCE(NEW, OLD);
END;
$$ LANGUAGE plpgsql;

-- Apply audit logging to sensitive tables
CREATE TRIGGER audit_guidance_publications AFTER INSERT OR UPDATE OR DELETE ON guidance_publications
    FOR EACH ROW EXECUTE FUNCTION create_audit_log();

CREATE TRIGGER audit_approval_requests AFTER INSERT OR UPDATE OR DELETE ON approval_requests
    FOR EACH ROW EXECUTE FUNCTION create_audit_log();

CREATE TRIGGER audit_users AFTER INSERT OR UPDATE OR DELETE ON users
    FOR EACH ROW EXECUTE FUNCTION create_audit_log();

-- ====================
-- VIEWS
-- ====================

-- Recent policy changes requiring review
CREATE VIEW recent_unreviewed_changes AS
SELECT
    pc.id,
    pc.detected_at,
    pd.platform_id,
    p.display_name AS platform_name,
    pd.document_type,
    pc.change_type,
    pc.severity,
    pc.confidence_score,
    pc.change_summary,
    pc.requires_member_notification
FROM policy_changes pc
JOIN policy_documents pd ON pc.policy_document_id = pd.id
JOIN platforms p ON pd.platform_id = p.id
WHERE pc.reviewed_at IS NULL
    AND pc.false_positive = false
    AND pc.detected_at > NOW() - INTERVAL '30 days'
ORDER BY pc.detected_at DESC;

-- Pending approvals
CREATE VIEW pending_approvals AS
SELECT
    ar.id,
    ar.request_type,
    ar.requested_by,
    ar.requested_at,
    ar.expires_at,
    CASE
        WHEN ar.request_type = 'guidance_publication' THEN gd.title
        ELSE NULL
    END AS guidance_title
FROM approval_requests ar
LEFT JOIN guidance_drafts gd ON ar.related_entity_id = gd.id AND ar.request_type = 'guidance_publication'
WHERE ar.status = 'pending'
    AND (ar.expires_at IS NULL OR ar.expires_at > NOW())
ORDER BY ar.requested_at ASC;

-- Platform monitoring status
CREATE VIEW platform_status AS
SELECT
    p.id,
    p.name,
    p.display_name,
    p.monitoring_active,
    COUNT(DISTINCT pd.id) AS policy_document_count,
    MAX(ps.captured_at) AS last_snapshot_at,
    COUNT(DISTINCT CASE WHEN pc.detected_at > NOW() - INTERVAL '30 days' THEN pc.id END) AS changes_last_30_days,
    COUNT(DISTINCT CASE WHEN pc.detected_at > NOW() - INTERVAL '7 days' THEN pc.id END) AS changes_last_7_days
FROM platforms p
LEFT JOIN policy_documents pd ON p.id = pd.platform_id AND pd.is_current = true
LEFT JOIN policy_snapshots ps ON pd.id = ps.policy_document_id
LEFT JOIN policy_changes pc ON pd.id = pc.policy_document_id
GROUP BY p.id, p.name, p.display_name, p.monitoring_active
ORDER BY p.display_name;

-- Guidance publication metrics
CREATE VIEW guidance_metrics AS
SELECT
    gp.id,
    gd.title,
    gp.scheduled_for,
    gp.published_at,
    gp.recipients_count,
    gp.successful_deliveries,
    gp.failed_deliveries,
    CASE
        WHEN gp.recipients_count > 0
        THEN ROUND((gp.successful_deliveries::DECIMAL / gp.recipients_count) * 100, 2)
        ELSE 0
    END AS delivery_rate_percent,
    CASE
        WHEN gp.successful_deliveries > 0
        THEN ROUND((gp.opened_count::DECIMAL / gp.successful_deliveries) * 100, 2)
        ELSE 0
    END AS open_rate_percent,
    CASE
        WHEN gp.opened_count > 0
        THEN ROUND((gp.clicked_count::DECIMAL / gp.opened_count) * 100, 2)
        ELSE 0
    END AS click_rate_percent
FROM guidance_publications gp
JOIN guidance_drafts gd ON gp.guidance_draft_id = gd.id
WHERE gp.published_at IS NOT NULL
ORDER BY gp.published_at DESC;

-- ====================
-- INITIAL DATA
-- ====================

-- Insert default system configuration
INSERT INTO system_config (key, value, description, config_type) VALUES
    ('approval_required', 'true', 'Require human approval before publishing guidance', 'security'),
    ('grace_period_minutes', '5', 'Minutes before publication can no longer be rolled back', 'security'),
    ('max_concurrent_collections', '10', 'Maximum number of simultaneous platform collections', 'monitoring'),
    ('default_check_frequency', '60', 'Default minutes between platform checks', 'monitoring'),
    ('min_confidence_threshold', '0.70', 'Minimum confidence score for automated actions', 'security'),
    ('notification_email_from', 'monitor@nuj.org.uk', 'From address for member notifications', 'notification'),
    ('test_group_emails', '["comms@nuj.org.uk"]', 'Test group for pre-publish validation', 'notification'),
    ('enable_ai_drafting', 'true', 'Allow AI to generate guidance drafts', 'general'),
    ('ai_model', 'gpt-4', 'AI model for guidance generation', 'general'),
    ('data_retention_days', '365', 'Days to retain policy snapshots', 'general')
ON CONFLICT (key) DO NOTHING;

-- Insert common platforms
INSERT INTO platforms (name, display_name, url, api_enabled, policy_urls, terms_urls, community_guidelines_urls) VALUES
    ('twitter', 'X (Twitter)', 'https://x.com', true,
     '["https://x.com/en/tos"]'::jsonb,
     '["https://x.com/en/tos"]'::jsonb,
     '["https://help.x.com/en/rules-and-policies/twitter-rules"]'::jsonb),

    ('facebook', 'Facebook', 'https://facebook.com', true,
     '["https://www.facebook.com/policies_center/"]'::jsonb,
     '["https://www.facebook.com/terms"]'::jsonb,
     '["https://www.facebook.com/communitystandards/"]'::jsonb),

    ('instagram', 'Instagram', 'https://instagram.com', true,
     '["https://help.instagram.com/581066165581870"]'::jsonb,
     '["https://www.instagram.com/legal/terms/"]'::jsonb,
     '["https://www.instagram.com/community-guidelines/"]'::jsonb),

    ('linkedin', 'LinkedIn', 'https://linkedin.com', true,
     '["https://www.linkedin.com/legal/user-agreement"]'::jsonb,
     '["https://www.linkedin.com/legal/user-agreement"]'::jsonb,
     '["https://www.linkedin.com/legal/professional-community-policies"]'::jsonb),

    ('tiktok', 'TikTok', 'https://tiktok.com', false,
     '["https://www.tiktok.com/legal/page/global/terms-of-service/en"]'::jsonb,
     '["https://www.tiktok.com/legal/page/global/terms-of-service/en"]'::jsonb,
     '["https://www.tiktok.com/community-guidelines"]'::jsonb),

    ('youtube', 'YouTube', 'https://youtube.com', true,
     '["https://www.youtube.com/t/terms"]'::jsonb,
     '["https://www.youtube.com/t/terms"]'::jsonb,
     '["https://www.youtube.com/howyoutubeworks/policies/community-guidelines/"]'::jsonb),

    ('bluesky', 'Bluesky', 'https://bsky.app', true,
     '["https://bsky.social/about/support/tos"]'::jsonb,
     '["https://bsky.social/about/support/tos"]'::jsonb,
     '["https://bsky.social/about/support/community-guidelines"]'::jsonb)
ON CONFLICT (name) DO NOTHING;

-- Insert default admin user (password should be changed immediately)
INSERT INTO users (email, username, full_name, role, password_hash) VALUES
    ('admin@nuj.org.uk', 'admin', 'System Administrator', 'admin', crypt('changeme', gen_salt('bf')))
ON CONFLICT (email) DO NOTHING;

-- Insert default member segments
INSERT INTO member_segments (name, description, criteria) VALUES
    ('all_members', 'All NUJ members', '{"all": true}'::jsonb),
    ('journalists', 'Working journalists', '{"role": "journalist"}'::jsonb),
    ('freelancers', 'Freelance members', '{"role": "freelancer"}'::jsonb),
    ('staff_journalists', 'Staff journalists', '{"role": "staff_journalist"}'::jsonb),
    ('photographers', 'Photography specialists', '{"role": "photographer"}'::jsonb),
    ('test_group', 'Communications team test group', '{"emails": ["comms@nuj.org.uk"]}'::jsonb)
ON CONFLICT (name) DO NOTHING;

-- ====================
-- COMMENTS
-- ====================

COMMENT ON TABLE platforms IS 'Social media platforms being monitored for policy changes';
COMMENT ON TABLE policy_documents IS 'Policy documents tracked per platform (terms, privacy, community guidelines)';
COMMENT ON TABLE policy_snapshots IS 'VerisimDB stream capturing point-in-time policy content';
COMMENT ON TABLE policy_changes IS 'Detected changes between policy snapshots with NLP analysis';
COMMENT ON TABLE guidance_drafts IS 'Member guidance documents (AI-generated or manual) awaiting approval';
COMMENT ON TABLE guidance_publications IS 'Published guidance with delivery tracking and rollback capability';
COMMENT ON TABLE approval_requests IS 'Workflow for human approval of sensitive actions';
COMMENT ON TABLE audit_log IS 'Complete audit trail of all system actions for compliance';

COMMENT ON COLUMN platforms.check_frequency_minutes IS 'How often to check this platform for changes (in minutes)';
COMMENT ON COLUMN policy_changes.confidence_score IS 'NLP confidence that this is a real change (0.0-1.0)';
COMMENT ON COLUMN policy_changes.severity IS 'Impact severity: critical, high, medium, low, unknown';
COMMENT ON COLUMN guidance_publications.grace_period_ends_at IS '5-minute window for rollback before email delivery';
COMMENT ON COLUMN system_config.is_sensitive IS 'If true, value should be encrypted and never logged';
