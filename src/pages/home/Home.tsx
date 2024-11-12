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
    const gameExe = await open({ multiple: false })
    await invoke('add_game', { path: gameExe })
  }

  const fetchGames = async () => {
    const response = await fetch('src/assets/TD/games.json')
    const games = await response.json()
    setGames(games as Game[])
  }

  return (
    <div className="home">
      <div onClick={handleAddGame}>
        <GameItem key={'add'} name={'Add Game'} imgUrl={''} />
      </div>
      {games.map((game: Game) => (
        <GameItem key={game.id} name={game.name} imgUrl={game.imgUrl} />
      ))}
    </div>
  )
}

export default Home
