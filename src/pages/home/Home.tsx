import { useEffect, useState } from 'react'
import { open } from '@tauri-apps/api/dialog'
import GameItem from '../../components/game-item/GameItem'
import { Game } from '../../models/Game'
import './Home.css'
import { invoke } from '@tauri-apps/api'

function Home() {
  const [games, setGames] = useState<Game[]>([])

  useEffect(() => {
    const fetchGames = async () => {
      const response = await fetch('src/assets/TD/games.json')
      const games = await response.json()
      setGames(games as Game[])
    }
    fetchGames()
  }, [])

  const handleAddGame = async () => {
    const gameExe = await open({ multiple: false, })
    const addedGame = await invoke('add_game', { path: gameExe })
    setGames([...games, addedGame as Game])
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
