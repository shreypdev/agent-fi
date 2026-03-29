const KEY_STORAGE = "agentfi_console_api_key";

export function apiBase(): string {
  const raw = import.meta.env.VITE_CONSOLE_API_BASE as string | undefined;
  if (raw == null || String(raw).trim() === "") {
    return "";
  }
  return String(raw).replace(/\/+$/, "");
}

export function getApiKey(): string {
  return sessionStorage.getItem(KEY_STORAGE) ?? "";
}

export function setApiKey(key: string): void {
  sessionStorage.setItem(KEY_STORAGE, key.trim());
}

export async function consoleFetch(
  path: string,
  init: RequestInit = {},
): Promise<Response> {
  const base = apiBase();
  if (!base) throw new Error("Set VITE_CONSOLE_API_BASE at build time");
  const key = getApiKey();
  const headers = new Headers(init.headers);
  if (key) headers.set("X-Console-Key", key);
  return fetch(`${base}${path}`, { ...init, headers });
}
