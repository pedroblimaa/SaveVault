import { invoke } from '@tauri-apps/api'
import { open } from '@tauri-apps/api/dialog'
import { SyntheticEvent, useState } from 'react'
import { FaFolder } from 'react-icons/fa'
import './Settings.css'

function Settings() {
  const [folder, setFolder] = useState('')

  const selectFolder = async () => {
    const selected = await open({ directory: true, multiple: false })
    setFolder(selected as string)
    invoke('set_cloud_folder', { path: selected })
  }

  const changeFolderValue = (event: SyntheticEvent<HTMLInputElement>) => {
    console.log(event)
    setFolder(event.currentTarget.value)
  }

  return (
    <>
      <div className="setting-container">
        <div className="setting-name">Cloud Folder</div>
        <button className="setting-button" onClick={selectFolder}>
          <FaFolder className="setting-icon" />
        </button>
        <input name="Cloud Folder" id="cloud_folder" value={folder} onChange={changeFolderValue} />
        {/* TODO - Add message to user know it will lose saved data if change folder */}
      </div>
    </>
  )
}

export default Settings
