/**
 * Normalizes env values for fetch() and <a href>. Host-only strings (e.g. Railway vars
 * without a scheme) become https URLs; explicit http/https is preserved. Leading-slash
 * paths are left as relative bases.
 */
export function ensureAbsoluteHttpUrl(raw: string): string {
  const trimmed = raw.trim();
  if (trimmed === "") return "";
  if (/^https?:\/\//i.test(trimmed)) {
    return trimmed.replace(/\/+$/, "");
  }
  if (trimmed.startsWith("/")) {
    const base = trimmed.replace(/\/+$/, "");
    return base === "" ? "/" : base;
  }
  return `https://${trimmed.replace(/\/+$/, "")}`;
}
