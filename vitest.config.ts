import { defineConfig } from 'vitest/config'
import vue from '@vitejs/plugin-vue'
import { resolve } from 'path'

export default defineConfig({
  plugins: [vue()],
  test: {
    environment: 'jsdom',
    globals: true,
    setupFiles: ['./tests/setup.ts'],
    include: ['tests/**/*.test.ts'],
    exclude: ['node_modules/**', 'src-tauri/**'],
    coverage: {
      reporter: ['text', 'html', 'lcov'],
      include: ['src/**/*'],
      exclude: [
        'src/main.ts',
        'src/**/*.d.ts',
        'src/types/**',
      ]
    }
  },
  resolve: {
    alias: {
      '@': resolve(__dirname, 'src'),
    },
  },
})