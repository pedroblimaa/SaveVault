import { invoke } from "@tauri-apps/api"
import { expect, Mock, test } from "vitest"
import { GameService } from "./gameService"
import { mockGames } from "./GameService.data.mock"


test("should fetch games and add loading equals to false", async () => {
  // Given
  (invoke as Mock).mockResolvedValue(mockGames)
  const expectedGames = [
    { ...mockGames[0], loading: false },
    { ...mockGames[1], loading: false },
  ]

  // When
  const games = await GameService.fetchGames()

  // Then
  expect(invoke).toHaveBeenCalledWith('get_games')
  expect(games).toEqual(expectedGames)
})