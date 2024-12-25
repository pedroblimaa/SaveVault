import { invoke } from '@tauri-apps/api'
import { open } from '@tauri-apps/api/dialog'
import { useEffect, useState } from 'react'
import { useNavigate } from 'react-router-dom'
import GameItem from '../../components/game-item/GameItem'
import { Game } from '../../models/Game'
import './Home.css'
import { texts } from '../../utils/config'
import { GameService } from '../../services/gameService'

function Home() {
  const navigate = useNavigate()
  const [games, setGames] = useState<Game[]>([])

  useEffect(() => {
    initApp()
  }, [])

  const initApp = async () => {
    const folder = await invoke('get_cloud_location')
    folder ? updateGames() : navigate('/settings')
  }

  const handleAddGame = async () => {
    const gameExe = await open({ multiple: false })
    let game: Game = await invoke('add_game', { path: gameExe })
    if (games.find((g) => g.id === game.id)) return

    setNewGame(game)
  }

  const setNewGame = async (game: Game) => {
    const newGameName = window.prompt(texts.changeName(), game.name)
    if (newGameName) {
      game.name = newGameName
    }

    setGames([...games, { ...game, loading: true }])

    await invoke('set_game_metadata', { id: game.id, name: game.name })
    await updateGames()
  }

  const updateGames = async () => {
    const games = await GameService.fetchGames()
    setGames(games)
  }

  return (
    <div className="home">
      <div onClick={handleAddGame}>
        <GameItem key={'add'} game={{ id: 'add', name: 'Add Game', loading: false }} />
      </div>
      {games.map((game: Game) => (
        <div>
          <GameItem key={game.id} game={game} />
        </div>
      ))}
    </div>
  )
}

export default Home
