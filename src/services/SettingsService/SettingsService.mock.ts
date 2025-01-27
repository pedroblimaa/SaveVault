import { invoke } from "@tauri-apps/api/core"
import { open } from "@tauri-apps/plugin-dialog"

import { Mock } from "vitest"

export const checkIsFolderAlreadyUsedMock = () => {
  (invoke as Mock).mockImplementation((cmd: string) => {
    if (cmd === 'check_is_folder_already_used') {
      return Promise.resolve(false)
    }
    return Promise.resolve(undefined)
  })
}

export const mockOpen = () => { 
  (open as Mock).mockResolvedValue('C:\\')
}