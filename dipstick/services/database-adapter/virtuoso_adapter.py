"""
Virtuoso Triple Store Adapter for NUJ Monitor
Provides RDF/SPARQL interface for semantic policy tracking
"""

from typing import Optional, List, Dict, Any
from dataclasses import dataclass
from datetime import datetime
import httpx
from rdflib import Graph, Namespace, Literal, URIRef
from rdflib.namespace import RDF, RDFS, XSD, OWL

# NUJ Monitor ontology namespace
NUJ = Namespace("https://nuj.org.uk/monitor/ontology/")
PLATFORM = Namespace("https://nuj.org.uk/monitor/platform/")
POLICY = Namespace("https://nuj.org.uk/monitor/policy/")
CHANGE = Namespace("https://nuj.org.uk/monitor/change/")

@dataclass
class VirtuosoConfig:
    host: str = "localhost"
    port: int = 1111
    http_port: int = 8890
    username: str = "dba"
    password: str = "dba"
    default_graph: str = "https://nuj.org.uk/monitor/default-graph"

class VirtuosoAdapter:
    """Adapter for Virtuoso RDF triple store"""

    def __init__(self, config: VirtuosoConfig):
        self.config = config
        self.sparql_endpoint = f"https://{config.host}:{config.http_port}/sparql"
        self.client = httpx.AsyncClient()

    async def execute_sparql(self, query: str, **kwargs) -> Dict[str, Any]:
        """Execute SPARQL query"""
        response = await self.client.post(
            self.sparql_endpoint,
            data={
                "query": query,
                "format": "application/sparql-results+json",
                **kwargs
            },
            auth=(self.config.username, self.config.password)
        )
        response.raise_for_status()
        return response.json()

    async def insert_platform(
        self,
        platform_id: str,
        name: str,
        display_name: str,
        api_enabled: bool,
        monitoring_active: bool,
        policy_urls: List[str]
    ) -> bool:
        """Insert platform as RDF triples"""
        platform_uri = PLATFORM[platform_id]

        query = f"""
        PREFIX nuj: <{NUJ}>
        PREFIX platform: <{PLATFORM}>
        PREFIX rdfs: <{RDFS}>

        INSERT DATA {{
          GRAPH <https://nuj.org.uk/monitor/platforms> {{
            {platform_uri.n3()} a nuj:Platform ;
              rdfs:label "{display_name}" ;
              nuj:name "{name}" ;
              nuj:apiEnabled {str(api_enabled).lower()} ;
              nuj:monitoringActive {str(monitoring_active).lower()} ;
              nuj:createdAt "{datetime.utcnow().isoformat()}"^^xsd:dateTime .
          }}
        }}
        """

        await self.execute_sparql(query)
        return True

    async def insert_policy_change(
        self,
        change_id: str,
        platform_id: str,
        policy_document_id: str,
        severity: str,
        confidence_score: float,
        change_summary: str,
        requires_notification: bool,
        detected_at: datetime
    ) -> bool:
        """Insert policy change as RDF"""
        change_uri = CHANGE[change_id]
        platform_uri = PLATFORM[platform_id]
        policy_uri = POLICY[policy_document_id]

        query = f"""
        PREFIX nuj: <{NUJ}>
        PREFIX change: <{CHANGE}>
        PREFIX policy: <{POLICY}>
        PREFIX platform: <{PLATFORM}>

        INSERT DATA {{
          GRAPH <https://nuj.org.uk/monitor/changes> {{
            {change_uri.n3()} a nuj:PolicyChange ;
              nuj:affectsPlatform {platform_uri.n3()} ;
              nuj:affectsPolicy {policy_uri.n3()} ;
              nuj:severity "{severity}" ;
              nuj:confidenceScore {confidence_score} ;
              nuj:changeSummary "{change_summary.replace('"', '\\"')}" ;
              nuj:requiresNotification {str(requires_notification).lower()} ;
              nuj:detectedAt "{detected_at.isoformat()}"^^xsd:dateTime .
          }}
        }}
        """

        await self.execute_sparql(query)
        return True

    async def query_recent_changes(
        self,
        days: int = 30,
        severity: Optional[str] = None,
        platform: Optional[str] = None
    ) -> List[Dict[str, Any]]:
        """Query recent policy changes"""
        severity_filter = f'FILTER (?severity = "{severity}")' if severity else ""
        platform_filter = f'FILTER (?platformLabel = "{platform}")' if platform else ""

        query = f"""
        PREFIX nuj: <{NUJ}>
        PREFIX rdfs: <{RDFS}>

        SELECT ?change ?platform ?platformLabel ?severity ?confidence ?summary ?detectedAt
        WHERE {{
          GRAPH <https://nuj.org.uk/monitor/changes> {{
            ?change a nuj:PolicyChange ;
              nuj:affectsPlatform ?platform ;
              nuj:severity ?severity ;
              nuj:confidenceScore ?confidence ;
              nuj:changeSummary ?summary ;
              nuj:detectedAt ?detectedAt .

            ?platform rdfs:label ?platformLabel .

            FILTER (?detectedAt > "{(datetime.utcnow() - timedelta(days=days)).isoformat()}"^^xsd:dateTime)
            {severity_filter}
            {platform_filter}
          }}
        }}
        ORDER BY DESC(?detectedAt)
        LIMIT 100
        """

        result = await self.execute_sparql(query)
        return result.get("results", {}).get("bindings", [])

    async def query_platform_statistics(self, platform_id: str) -> Dict[str, Any]:
        """Get statistics for a platform"""
        platform_uri = PLATFORM[platform_id]

        query = f"""
        PREFIX nuj: <{NUJ}>

        SELECT
          (COUNT(DISTINCT ?change) as ?totalChanges)
          (COUNT(DISTINCT ?criticalChange) as ?criticalChanges)
          (AVG(?confidence) as ?avgConfidence)
        WHERE {{
          {{
            SELECT ?change ?confidence WHERE {{
              GRAPH <https://nuj.org.uk/monitor/changes> {{
                ?change a nuj:PolicyChange ;
                  nuj:affectsPlatform {platform_uri.n3()} ;
                  nuj:confidenceScore ?confidence .
              }}
            }}
          }}
          OPTIONAL {{
            ?criticalChange nuj:severity "critical" ;
              nuj:affectsPlatform {platform_uri.n3()} .
          }}
        }}
        """

        result = await self.execute_sparql(query)
        bindings = result.get("results", {}).get("bindings", [])
        if bindings:
            return bindings[0]
        return {}

    async def semantic_search(self, keywords: List[str]) -> List[Dict[str, Any]]:
        """Semantic search across all policy changes"""
        keyword_filter = " OR ".join([f'CONTAINS(?summary, "{kw}")' for kw in keywords])

        query = f"""
        PREFIX nuj: <{NUJ}>
        PREFIX rdfs: <{RDFS}>

        SELECT ?change ?platform ?severity ?summary ?detectedAt
        WHERE {{
          GRAPH <https://nuj.org.uk/monitor/changes> {{
            ?change a nuj:PolicyChange ;
              nuj:affectsPlatform ?platform ;
              nuj:severity ?severity ;
              nuj:changeSummary ?summary ;
              nuj:detectedAt ?detectedAt .

            FILTER ({keyword_filter})
          }}

          ?platform rdfs:label ?platformLabel .
        }}
        ORDER BY DESC(?detectedAt)
        LIMIT 50
        """

        result = await self.execute_sparql(query)
        return result.get("results", {}).get("bindings", [])

    async def export_graph(self, graph_uri: str, format: str = "turtle") -> str:
        """Export entire named graph"""
        query = f"""
        CONSTRUCT {{ ?s ?p ?o }}
        WHERE {{
          GRAPH <{graph_uri}> {{ ?s ?p ?o }}
        }}
        """

        response = await self.client.post(
            self.sparql_endpoint,
            data={"query": query, "format": f"application/{format}"},
            auth=(self.config.username, self.config.password)
        )
        return response.text

    async def close(self):
        """Close HTTP client"""
        await self.client.aclose()

# Example usage
async def main():
    from datetime import timedelta

    config = VirtuosoConfig()
    adapter = VirtuosoAdapter(config)

    # Query recent changes
    changes = await adapter.query_recent_changes(days=7, severity="critical")
    print(f"Found {len(changes)} critical changes in last 7 days")

    # Semantic search
    results = await adapter.semantic_search(["journalist", "harassment", "content removal"])
    print(f"Found {len(results)} changes matching keywords")

    # Platform statistics
    stats = await adapter.query_platform_statistics("twitter")
    print(f"Twitter stats: {stats}")

    await adapter.close()

if __name__ == "__main__":
    import asyncio
    asyncio.run(main())
