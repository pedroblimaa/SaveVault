import { invoke } from '@tauri-apps/api/core'
import { texts } from '../../utils/config'
import { open } from '@tauri-apps/plugin-dialog'

export class SettingsService {

  static async handleSetFolder(folder: string): Promise<string> {
    if (folder?.length) {
      const confirmMove = await window.confirm(texts.confirmMoveFolder())
      if (!confirmMove) return folder
    }

    const selected = await open({ directory: true, multiple: false })
    const isFolderAlreadyUsed = await invoke('check_is_folder_already_used', { path: selected })

    const override = isFolderAlreadyUsed ? await confirm(texts.confirmOverrideFolder()) : true

    invoke('set_cloud_location', { path: selected, overrideFolder: override })

    return selected as string
  }
}
