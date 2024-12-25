import { dialog, invoke } from "@tauri-apps/api"
import { expect, test, vi } from "vitest"
import { SettingsService } from "./settingsService"



test("should fetch games and add loading equals to false", async () => {
  // Given
  const openSpy = vi.spyOn(dialog, 'open').mockResolvedValue('C:\\')
  vi.mocked(invoke).mockImplementation((cmd) => {
    if (cmd === 'check_is_folder_already_used') {
      return Promise.resolve(false)
    }
    return Promise.resolve(undefined)
  })

  // When
  await SettingsService.handleSetFolder('')

  // Then
  expect(openSpy).toHaveBeenCalledWith({ directory: true, multiple: false })
  expect(invoke).toHaveBeenCalledWith('set_cloud_location', { path: 'C:\\', overrideFolder: true })
})
