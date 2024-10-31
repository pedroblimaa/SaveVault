import { useEffect, useState } from 'react'
import { open } from '@tauri-apps/api/dialog'
import GameItem from '../../components/game-item/GameItem'
import { Game } from '../../models/Game'
import './Home.css'
import { invoke } from '@tauri-apps/api'
import { useNavigate } from 'react-router-dom'

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
    // TODO - Fix game being added duplicated when try to add it again
    const addedGame = await invoke('add_game', { path: gameExe })
    setGames([...games, addedGame as Game])
  }

  const fetchGames = async () => {
    const response = await fetch('src/assets/TD/games.json')
    const games = await response.json()
    setGames(games as Game[])
  }

  return (
    <div className="home">
      {games.map((game: Game) => (
        <GameItem key={game.id} name={game.name} imgUrl={game.imgUrl} />
      ))}
      <div onClick={handleAddGame}>
        <GameItem key={'add'} name={'Add Game'} imgUrl={''} />
      </div>
    </div>
  )
}

export default Home
