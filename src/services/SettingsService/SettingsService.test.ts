import { open } from "@tauri-apps/plugin-dialog"
import { invoke } from "@tauri-apps/api/core"
import { describe, expect, Mock, test, vi } from "vitest"
import { SettingsService } from "./SettingsService"
import { texts } from "../../utils/config"

// Mock the @tauri-apps/api module
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}))

// Mock the @tauri-apps/plugin-dialog module
vi.mock("@tauri-apps/plugin-dialog", () => ({
  open: vi.fn(),
}))

vi.stubGlobal('confirm', () => Promise.resolve(true))

describe("handleSetFolder", () => {
  test("should set folder when called with no value", async () => {
    // Given
    (open as Mock).mockResolvedValue('C:\\')
    const confirmSpy = vi.spyOn(globalThis, 'confirm')
    confirmSpy.mockResolvedValue(true);
    (invoke as Mock).mockImplementation((cmd: string) => {
      if (cmd === 'check_is_folder_already_used') {
        return Promise.resolve(false)
      }
      return Promise.resolve(undefined)
    })

    // When
    await SettingsService.handleSetFolder('')

    // Then
    expect(confirmSpy).not.toHaveBeenCalled()
    expect(open).toHaveBeenCalledWith({ directory: true, multiple: false })
    expect(invoke).toHaveBeenCalledWith('set_cloud_location', { path: 'C:\\', overrideFolder: true })
  })

  test("should set folder and confirm when called with some value", async () => {
    // Given
    (open as Mock).mockResolvedValue('C:\\')
    const confirmSpy = vi.spyOn(globalThis, 'confirm')
    confirmSpy.mockResolvedValue(true)
    vi.mocked(invoke).mockImplementation((cmd) => {
      if (cmd === 'check_is_folder_already_used') {
        return Promise.resolve(false)
      }
      return Promise.resolve(undefined)
    })

    // When
    await SettingsService.handleSetFolder('C:\\someFolder')

    // Then
    expect(confirmSpy).toHaveBeenCalledWith(texts.confirmMoveFolder())
    expect(open).toHaveBeenCalledWith({ directory: true, multiple: false })
    expect(invoke).toHaveBeenCalledWith('set_cloud_location', { path: 'C:\\', overrideFolder: true })
  })
})
