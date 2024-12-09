import { invoke } from '@tauri-apps/api'
import { open } from '@tauri-apps/api/dialog'
import { useEffect, useState } from 'react'
import { useNavigate } from 'react-router-dom'
import GameItem from '../../components/game-item/GameItem'
import { Game } from '../../models/Game'
import './Home.css'

function Home() {
  const navigate = useNavigate()
  const [games, setGames] = useState<Game[]>([])

  useEffect(() => {
    initApp()
  }, [])

  const initApp = async () => {
    const folder = await invoke('get_cloud_location')
    folder ? fetchGames() : navigate('/settings')
  }

  const handleAddGame = async () => {
    // TODO - Refactor this logic
    const gameExe = await open({ multiple: false })
    let game: Game = await invoke('add_game', { path: gameExe })
    await fetchGames(game)

    await invoke('set_game_metadata', { id: game.id })
    await fetchGames()
  }

  const fetchGames = async (loadingGame?: Game) => {
    const response = (await invoke('get_games')) as Game[]

    // TODO  Move this to a Service layer
    const isLoading = (game: Game) => game.id === loadingGame?.id
    const games = response.map((game: Game) => ({ ...game, loading: isLoading(game) }))


    setGames(games)
    // TODO - Give an option for the user to change the game name
  }

  const getGameInfo = async (game: Game) => {
    console.log('Getting game info')
    await invoke('get_game_info', { id: game.id })
  }

  return (
    <div className="home">
      <div onClick={handleAddGame}>
        <GameItem key={'add'} game={{ id: 'add', name: 'Add Game', loading: false }} />
      </div>
      <div onClick={handleAddGame}>
        <GameItem key={'add'} game={{ id: 'add', name: 'Add Game', loading: true }} />
      </div>
      {games.map((game: Game) => (
        <div onClick={() => getGameInfo(game)}>
          <GameItem key={game.id} game={game} />
        </div>
      ))}
    </div>
  )
}

export default Home
