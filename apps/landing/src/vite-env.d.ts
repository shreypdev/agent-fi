/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_CONNECT_URL: string;
  readonly VITE_PUBLIC_AGENT_URL: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
