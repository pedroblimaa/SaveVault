import { FaPlus } from 'react-icons/fa'

import { Game } from '../../models/Game'
import './GameItem.css'

function GameItem({ game }: { game: Partial<Game> }) {
  return (
    <div className="game-item">
      {/* TODO - Refactor this logic */}
      {game.img_path ? (
        <img src={game.img_path} alt={game.name} className="game-item-img" />
      ) : game.loading ? (
        <div className="game-item-loading">Loading</div>
      ) : (
        <FaPlus className="game-item-icon" />
      )}
    </div>
  )
}

export default GameItem
