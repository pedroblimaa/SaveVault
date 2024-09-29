import { open } from '@tauri-apps/api/dialog'
import { SyntheticEvent, useState } from 'react'
import { FaFolder } from 'react-icons/fa'
import './Settings.css'

function Settings() {
  const [folder, setFolder] = useState('')

  const selectFolder = async () => {
    const selected = await open({ directory: true, multiple: false })
    setFolder(selected as string)
  }

  const changeFolderValue = (event: SyntheticEvent<HTMLInputElement>) => {
    console.log(event);
    setFolder(event.currentTarget.value);
  };

  return (
    <>
      <div className="setting-container">
        <div className="setting-name">Cloud Folder:</div>
        <button className="setting-button" onClick={selectFolder}>
          <FaFolder className='setting-icon'/>
        </button>
        <input name="Cloud Folder" id="cloud_folder" value={folder} onChange={changeFolderValue} />
      </div>
    </>
  )
}

export default Settings
