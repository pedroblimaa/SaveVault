import { FaPlus } from 'react-icons/fa'
import { Game } from '../../models/Game'
import './GameItem.css'

function GameItem({ img, name }: Partial<Game>) {
  return (
    <div className="game-item">
      {img ? <img src={img} alt={name} className="game-item-img" /> : <FaPlus className="game-item-icon" />}
    </div>
  )
}

export default GameItem
