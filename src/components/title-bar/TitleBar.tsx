import { appWindow } from '@tauri-apps/api/window'
import './TitleBar.css'
import TitleBarButton from './title-bar-button/TitleBarButton'

function TitleBar() {
  const minimize = () => {
    appWindow.minimize()
  }

  const maximize = () => {
    appWindow.toggleMaximize()
  }

  const close = () => {
    appWindow.close()
  }

  return (
    <div data-tauri-drag-region className="titlebar">
      <div className="menu">
        <div>Library</div>
        <div>Settings</div>
      </div>
      <div className="window-commands">
        <TitleBarButton event={minimize} alt="minimize" icon="https://api.iconify.design/mdi:window-minimize.svg" />
        <TitleBarButton event={maximize} alt="maximize" icon="https://api.iconify.design/mdi:window-maximize.svg" />
        <TitleBarButton event={close} alt="close" icon="https://api.iconify.design/mdi:close.svg" />
      </div>
    </div>
  )
}

export default TitleBar
