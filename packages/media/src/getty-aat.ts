export const GETTY_AAT_SPARQL_ENDPOINT = "https://vocab.getty.edu/sparql";
export const GETTY_AAT_SOURCE_ID = "source-getty-aat";

export interface GettyAATRecord {
  externalUri: string;
  sourcePageUrl: string;
  originalLabel: string;
  language: string;
  sourceId: typeof GETTY_AAT_SOURCE_ID;
  retrievedAt: string;
}

interface SparqlBinding { concept?: { value?: string }; label?: { value?: string; "xml:lang"?: string }; }
interface SparqlResponse { results?: { bindings?: SparqlBinding[] } }

export interface GettyAATAdapterOptions {
  endpoint?: string;
  fetcher?: typeof fetch;
  now?: () => string;
}

/** Uses Getty's documented SPARQL/LOD endpoint; it never scrapes the visual site. */
export class GettyAATAdapter {
  private readonly endpoint: string;
  private readonly fetcher: typeof fetch;
  private readonly now: () => string;

  public constructor(options: GettyAATAdapterOptions = {}) {
    this.endpoint = options.endpoint ?? GETTY_AAT_SPARQL_ENDPOINT;
    this.fetcher = options.fetcher ?? fetch;
    this.now = options.now ?? (() => new Date().toISOString());
  }

  public async searchByLabel(label: string, limit = 10): Promise<GettyAATRecord[]> {
    const safeLabel = label.replaceAll('"', '\\"');
    const query = [
      "PREFIX skos: <http://www.w3.org/2004/02/skos/core#>",
      "PREFIX aat: <http://vocab.getty.edu/aat/>",
      "SELECT ?concept ?label WHERE {",
      "  ?concept skos:inScheme aat: ; skos:prefLabel ?label .",
      `  FILTER(LCASE(STR(?label)) = LCASE(\"${safeLabel}\"))`,
      "} LIMIT " + Math.max(1, Math.min(50, Math.trunc(limit)))
    ].join(" ");
    const url = `${this.endpoint}?query=${encodeURIComponent(query)}&format=json`;
    const response = await this.fetcher(url, { headers: { accept: "application/sparql-results+json" } });
    if (!response.ok) throw new Error(`Getty AAT request failed with HTTP ${response.status}`);
    const body = await response.json() as SparqlResponse;
    const retrievedAt = this.now();
    return (body.results?.bindings ?? []).flatMap((binding) => {
      const uri = binding.concept?.value;
      const originalLabel = binding.label?.value;
      if (!uri || !originalLabel || !uri.startsWith("http://vocab.getty.edu/aat/")) return [];
      return [{ externalUri: uri, sourcePageUrl: uri.replace("http://vocab.getty.edu/aat/", "https://vocab.getty.edu/page/aat/"), originalLabel, language: binding.label?.["xml:lang"] ?? "", sourceId: GETTY_AAT_SOURCE_ID, retrievedAt }];
    });
  }
}
