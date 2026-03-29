/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_CONNECT_URL?: string;
  readonly VITE_PUBLIC_AGENT_URL?: string;
  /** Public searchd base URL (no trailing slash), e.g. https://search.example.com */
  readonly VITE_SEARCH_API_BASE_URL?: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
