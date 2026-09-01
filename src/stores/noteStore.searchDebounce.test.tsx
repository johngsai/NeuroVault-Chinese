import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import { useNoteStore } from "./noteStore";

/**
 * The sidebar search box drives semantic ranking through
 * `noteStore.setSearchQuery` → `GET /api/recall?mode=titles`. Every human
 * surface (command palette, search view) debounces its recall; the sidebar
 * used to fire one request PER KEYSTROKE with no debounce. A 17-char query
 * therefore burned 17 calls against the shared per-brain 60s recall throttle
 * and surfaced the agent-facing "Recall hard-throttled: 17 calls in 60s" hint
 * in the command palette during normal human search.
 */
describe("noteStore sidebar search recall", () => {
  beforeEach(() => {
    vi.useFakeTimers();
    useNoteStore.setState({ searchQuery: "", searchResults: [] });
  });

  afterEach(() => {
    vi.unstubAllGlobals();
    vi.runOnlyPendingTimers();
    vi.useRealTimers();
  });

  it("fires one recall per typing burst, not one per keystroke", async () => {
    const fetchMock = vi.fn().mockResolvedValue({ ok: true, json: async () => [] });
    vi.stubGlobal("fetch", fetchMock);

    // A 17-character query typed normally = 17 keystrokes in a burst.
    const query = "recall rate limit hint";
    for (let i = 1; i <= query.length; i++) {
      useNoteStore.getState().setSearchQuery(query.slice(0, i));
    }

    // Before the debounce elapses, nothing has hit the network.
    expect(fetchMock).not.toHaveBeenCalled();

    // After the pause, exactly one recall fires — for the final query.
    await vi.advanceTimersByTimeAsync(300);
    expect(fetchMock).toHaveBeenCalledTimes(1);
    const url = String(fetchMock.mock.calls[0]![0]);
    expect(url).toContain(`q=${encodeURIComponent(query)}`);
  });

  it("does not fetch for queries shorter than two characters", () => {
    const fetchMock = vi.fn();
    vi.stubGlobal("fetch", fetchMock);
    useNoteStore.getState().setSearchQuery("a");
    vi.runOnlyPendingTimers();
    expect(fetchMock).not.toHaveBeenCalled();
  });
});
