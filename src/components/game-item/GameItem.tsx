import { FaPlus } from 'react-icons/fa'

import { CSSProperties } from 'react'
import { MoonLoader } from 'react-spinners'
import { Game } from '../../models/Game'
import './GameItem.css'

const override: CSSProperties = {
  display: 'block',
  margin: '0 auto',
  borderColor: 'red',
}

function GameItem({ game }: { game: Partial<Game> }) {
  const renderGameItem = () => {
    const renderOptions = {
      img_path: <img src={game.img_path} alt={game.name} className="game-item-img" />,
      loading: <MoonLoader color={'#ffffff'} loading={true} cssOverride={override} size={50} />,
      default: <FaPlus className="game-item-icon" />,
    }

    if (game.img_path) {
      return renderOptions.img_path
    }

    return game.loading ? renderOptions.loading : renderOptions.default
  }

  return <div className="game-item">{renderGameItem()}</div>
}

export default GameItem
