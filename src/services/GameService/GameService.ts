import { invoke } from "@tauri-apps/api/core"
import { Game } from "../../models/Game"

export class GameService {

  static async fetchGames(): Promise<Game[]> {
    const response = (await invoke('get_games')) as Game[]
    return response.map((game: Game) => ({ ...game, loading: false }))
  }
}