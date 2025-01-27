import { invoke } from "@tauri-apps/api/core"
import { open } from "@tauri-apps/plugin-dialog"
import { beforeEach, describe, expect, MockInstance, test, vi } from "vitest"
import { texts } from "../../utils/config"
import { SettingsService } from "./SettingsService"
import { checkIsFolderAlreadyUsedMock, mockOpen } from "./SettingsService.mock"

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}))

vi.mock("@tauri-apps/plugin-dialog", () => ({
  open: vi.fn(),
}))

vi.stubGlobal('confirm', () => Promise.resolve(true))

let confirmSpy: MockInstance

describe("handleSetFolder", () => {
  beforeEach(() => {
    mockOpen()
    checkIsFolderAlreadyUsedMock()

    confirmSpy = vi.spyOn(globalThis, 'confirm')
    confirmSpy.mockResolvedValue(true)
  })

  test("should set folder when called with no value", async () => {
    // When
    await SettingsService.handleSetFolder('')

    // Then
    expect(confirmSpy).not.toHaveBeenCalled()
    expect(open).toHaveBeenCalledWith({ directory: true, multiple: false })
    expect(invoke).toHaveBeenCalledWith('set_cloud_location', { path: 'C:\\', overrideFolder: true })
  })

  test("should set folder and confirm when called with some value", async () => {
    // When
    await SettingsService.handleSetFolder('C:\\someFolder')

    // Then
    expect(confirmSpy).toHaveBeenCalledWith(texts.confirmMoveFolder())
    expect(open).toHaveBeenCalledWith({ directory: true, multiple: false })
    expect(invoke).toHaveBeenCalledWith('set_cloud_location', { path: 'C:\\', overrideFolder: true })
  })
})
