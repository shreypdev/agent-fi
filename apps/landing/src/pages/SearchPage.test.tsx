import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { MemoryRouter } from "react-router-dom";
import SearchPage from "./SearchPage";

describe("SearchPage", () => {
  it("renders discover heading (mobile-friendly single column)", () => {
    render(
      <MemoryRouter>
        <SearchPage />
      </MemoryRouter>,
    );
    expect(screen.getByRole("heading", { name: /discover agents/i })).toBeInTheDocument();
    expect(screen.getByRole("searchbox")).toBeInTheDocument();
  });
});
