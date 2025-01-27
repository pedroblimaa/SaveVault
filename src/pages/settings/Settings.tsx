import { invoke } from '@tauri-apps/api/core'
import { SyntheticEvent, useEffect, useState } from 'react'
import { FaFolder } from 'react-icons/fa'
import './Settings.css'
import { SettingsService } from '../../services/SettingsService/SettingsService'

function Settings() {
  const [folder, setFolder] = useState('')

  useEffect(() => {
    invoke('get_cloud_location').then((folder) => setFolder(folder as string))
  }, [])

  const selectFolder = async () => {
    const selectedFolder = await SettingsService.handleSetFolder(folder)

    setFolder(selectedFolder)
  }

  const changeFolderValue = (event: SyntheticEvent<HTMLInputElement>) => {
    console.log(event)
    setFolder(event.currentTarget.value)
  }

  return (
    <>
      {folder.length === 0 && (
        <div className="empty-folder-message">Please, select a cloud folder to start using the app</div>
      )}
      <div className="setting-container">
        <div className="setting-name">Cloud Folder</div>
        <button className="setting-button" onClick={selectFolder}>
          <FaFolder className="setting-icon" />
        </button>
        <input name="Cloud Folder" id="cloud_folder" value={folder} onChange={changeFolderValue} />
      </div>
    </>
  )
}

export default Settings
