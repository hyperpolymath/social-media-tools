-- Virtuoso Initialization Script for NUJ Monitor
-- Creates RDF graphs and ontology for social media policy tracking

-- Enable RDF support
DB.DBA.RDF_QUAD_URI_L_TYPED ('http://nuj.org.uk/monitor', 0);

-- Create named graphs
SPARQL CREATE GRAPH <http://nuj.org.uk/monitor/platforms>;
SPARQL CREATE GRAPH <http://nuj.org.uk/monitor/policies>;
SPARQL CREATE GRAPH <http://nuj.org.uk/monitor/changes>;
SPARQL CREATE GRAPH <http://nuj.org.uk/monitor/guidance>;
SPARQL CREATE GRAPH <http://nuj.org.uk/monitor/meta>;

-- Define custom ontology prefixes
SPARQL
PREFIX nuj: <http://nuj.org.uk/monitor/ontology/>
PREFIX platform: <http://nuj.org.uk/monitor/platform/>
PREFIX policy: <http://nuj.org.uk/monitor/policy/>
PREFIX change: <http://nuj.org.uk/monitor/change/>

INSERT DATA {
  GRAPH <http://nuj.org.uk/monitor/meta> {
    nuj:Ontology a owl:Ontology ;
      rdfs:label "NUJ Social Media Monitor Ontology" ;
      rdfs:comment "Ontology for tracking social media platform policy changes" .

    # Classes
    nuj:Platform a owl:Class ;
      rdfs:label "Social Media Platform" .

    nuj:PolicyDocument a owl:Class ;
      rdfs:label "Policy Document" .

    nuj:PolicyChange a owl:Class ;
      rdfs:label "Policy Change Event" .

    nuj:Guidance a owl:Class ;
      rdfs:label "Member Guidance" .

    # Properties
    nuj:hasPolicy a owl:ObjectProperty ;
      rdfs:domain nuj:Platform ;
      rdfs:range nuj:PolicyDocument .

    nuj:detectedChange a owl:ObjectProperty ;
      rdfs:domain nuj:PolicyDocument ;
      rdfs:range nuj:PolicyChange .

    nuj:severity a owl:DatatypeProperty ;
      rdfs:domain nuj:PolicyChange ;
      rdfs:range xsd:string .

    nuj:confidenceScore a owl:DatatypeProperty ;
      rdfs:domain nuj:PolicyChange ;
      rdfs:range xsd:decimal .

    nuj:requiresNotification a owl:DatatypeProperty ;
      rdfs:domain nuj:PolicyChange ;
      rdfs:range xsd:boolean .

    nuj:generatedGuidance a owl:ObjectProperty ;
      rdfs:domain nuj:PolicyChange ;
      rdfs:range nuj:Guidance .
  }
};

-- Insert initial platform data as RDF
SPARQL
INSERT DATA {
  GRAPH <http://nuj.org.uk/monitor/platforms> {
    platform:twitter a nuj:Platform ;
      rdfs:label "X (Twitter)" ;
      nuj:apiEnabled true ;
      nuj:monitoringActive true ;
      nuj:policyURL <https://twitter.com/en/tos> ;
      nuj:checkFrequencyMinutes 60 .

    platform:facebook a nuj:Platform ;
      rdfs:label "Facebook" ;
      nuj:apiEnabled true ;
      nuj:monitoringActive true ;
      nuj:policyURL <https://www.facebook.com/terms> ;
      nuj:checkFrequencyMinutes 60 .

    platform:instagram a nuj:Platform ;
      rdfs:label "Instagram" ;
      nuj:apiEnabled true ;
      nuj:monitoringActive true ;
      nuj:policyURL <https://help.instagram.com/581066165581870> ;
      nuj:checkFrequencyMinutes 60 .

    platform:linkedin a nuj:Platform ;
      rdfs:label "LinkedIn" ;
      nuj:apiEnabled true ;
      nuj:monitoringActive true ;
      nuj:policyURL <https://www.linkedin.com/legal/user-agreement> ;
      nuj:checkFrequencyMinutes 60 .

    platform:tiktok a nuj:Platform ;
      rdfs:label "TikTok" ;
      nuj:apiEnabled false ;
      nuj:monitoringActive true ;
      nuj:scrapingEnabled true ;
      nuj:policyURL <https://www.tiktok.com/legal/page/global/terms-of-service/en> ;
      nuj:checkFrequencyMinutes 120 .

    platform:youtube a nuj:Platform ;
      rdfs:label "YouTube" ;
      nuj:apiEnabled true ;
      nuj:monitoringActive true ;
      nuj:policyURL <https://www.youtube.com/t/terms> ;
      nuj:checkFrequencyMinutes 60 .

    platform:bluesky a nuj:Platform ;
      rdfs:label "Bluesky" ;
      nuj:apiEnabled true ;
      nuj:monitoringActive true ;
      nuj:policyURL <https://bsky.social/about/support/tos> ;
      nuj:checkFrequencyMinutes 30 .
  }
};

-- Create full-text indexes for SPARQL searches
DB.DBA.RDF_OBJ_FT_RULE_ADD (null, null, 'All');
DB.DBA.VT_INC_INDEX_DB_DBA_RDF_OBJ ();

-- Enable inference
rdfs_rule_set ('http://nuj.org.uk/monitor/rules', 'http://nuj.org.uk/monitor/meta');

CHECKPOINT;

SELECT 'Virtuoso initialization complete' as status;
