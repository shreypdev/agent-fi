import { describe, expect, it } from "vitest";
import { ensureAbsoluteHttpUrl } from "./absoluteHttpUrl";

describe("ensureAbsoluteHttpUrl", () => {
  it("returns empty for blank", () => {
    expect(ensureAbsoluteHttpUrl("")).toBe("");
    expect(ensureAbsoluteHttpUrl("  ")).toBe("");
  });

  it("prepends https for host-only (Railway-style) values", () => {
    expect(ensureAbsoluteHttpUrl("agent-fi.up.railway.app")).toBe(
      "https://agent-fi.up.railway.app",
    );
  });

  it("preserves explicit schemes and strips trailing slashes", () => {
    expect(ensureAbsoluteHttpUrl("http://127.0.0.1:8090/")).toBe(
      "http://127.0.0.1:8090",
    );
    expect(ensureAbsoluteHttpUrl("https://api.example.com/")).toBe(
      "https://api.example.com",
    );
  });

  it("leaves relative path bases unchanged", () => {
    expect(ensureAbsoluteHttpUrl("/api")).toBe("/api");
    expect(ensureAbsoluteHttpUrl("/api/")).toBe("/api");
  });
});
