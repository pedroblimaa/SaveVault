import { invoke } from '@tauri-apps/api'
import { open } from '@tauri-apps/api/dialog'
import { SyntheticEvent, useEffect, useState } from 'react'
import { FaFolder } from 'react-icons/fa'
import './Settings.css'

function Settings() {
  const [folder, setFolder] = useState('')

  useEffect(() => {
    invoke('get_cloud_location').then((folder) => setFolder(folder as string))
  }, [])

  const selectFolder = async () => {
    if (folder?.length) {
      const userConfirmed = await confirm('Changing the cloud folder will move all existing files to the new location.')
      if (!userConfirmed) return
    }

    const selected = await open({ directory: true, multiple: false })
    setFolder(selected as string)
    invoke('set_cloud_location', { path: selected })
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
      </div>
    </>
  )
}

export default Settings
