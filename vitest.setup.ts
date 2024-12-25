import { vi } from 'vitest'

vi.mock('@tauri-apps/api', () => ({
  invoke: vi.fn(),
  dialog: {
    open: vi.fn(),
  }
}))