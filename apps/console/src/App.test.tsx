import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { MemoryRouter } from "react-router-dom";
import { beforeEach, describe, expect, it, vi } from "vitest";
import * as api from "./api";

vi.mock("./api", () => ({
  apiBase: vi.fn(),
  getApiKey: vi.fn(),
  setApiKey: vi.fn(),
  consoleFetch: vi.fn(),
}));

describe("App", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(api.apiBase).mockReturnValue("");
    vi.mocked(api.getApiKey).mockReturnValue("");
  });

  it("shows missing base message when apiBase empty", async () => {
    const { default: App } = await import("./App");
    render(
      <MemoryRouter>
        <App />
      </MemoryRouter>,
    );
    expect(screen.getByText(/VITE_CONSOLE_API_BASE/i)).toBeInTheDocument();
  });

  it("renders setup shell when API base is set", async () => {
    vi.mocked(api.apiBase).mockReturnValue("https://consoled.test");
    const { default: App } = await import("./App");
    render(
      <MemoryRouter initialEntries={["/"]}>
        <App />
      </MemoryRouter>,
    );
    expect(screen.getByText(/Agent Search Console/i)).toBeInTheDocument();
    expect(screen.getByText(/Console API key/i)).toBeInTheDocument();
  });

  it("claims page loads list after List claims succeeds", async () => {
    vi.mocked(api.apiBase).mockReturnValue("https://consoled.test");
    vi.mocked(api.consoleFetch).mockResolvedValue({
      ok: true,
      text: async () => "",
      json: async () => [{ domain: "claim.example", status: "pending" }],
    } as Response);

    const { default: App } = await import("./App");
    render(
      <MemoryRouter initialEntries={["/claims"]}>
        <App />
      </MemoryRouter>,
    );

    fireEvent.click(screen.getByRole("button", { name: /List claims/i }));

    await waitFor(() => {
      expect(screen.getByText(/claim\.example/)).toBeInTheDocument();
    });
    expect(api.consoleFetch).toHaveBeenCalledWith("/v1/console/domain-claims");
  });
});
