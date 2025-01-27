import { invoke } from "@tauri-apps/api/core"
import { describe, expect, Mock, test, vi } from "vitest"
import { GameService } from "./GameService"
import { gamesMock } from "./GameService.data.mock"

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}))

describe('fetchGames', () => {
  test("should fetch games and add loading equals to false", async () => {
    // Given
    const expectedGames = gamesMock.map(game => ({ ...game, loading: false }));
    (invoke as Mock).mockImplementation(() => gamesMock)

    // When
    const games = await GameService.fetchGames()

    // Then
    expect(invoke).toHaveBeenCalledWith('get_games')
    expect(games).toEqual(expectedGames)
  })
})