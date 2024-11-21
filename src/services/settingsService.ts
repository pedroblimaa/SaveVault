import { invoke } from '@tauri-apps/api'
import { open } from '@tauri-apps/api/dialog'

export class SettingsService {
  private static readonly CONFIRM_MOVE_FOLDER =
    'Changing the cloud folder will move all existing files to the new location.'
  private static readonly CONFIRM_OVERRIDE_FOLDER =
    'This folder is already set as a cloud folder, do you want to override?'

  static async handleSetFolder(folder: string): Promise<string> {
    if (folder?.length) {
      const confirmMove = await confirm(this.CONFIRM_MOVE_FOLDER)
      if (!confirmMove) return folder
    }

    const selected = await open({ directory: true, multiple: false })
    const folderAlreadyUsed = await invoke('folder_already_used', { path: selected })

    const override = folderAlreadyUsed ? await confirm(this.CONFIRM_OVERRIDE_FOLDER) : true

    invoke('set_cloud_location', { path: selected, overrideFolder: override })

    return selected as string
  }
}
