import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import wasmPack from 'vite-plugin-wasm-pack';

export default defineConfig({
  build: {
    minify: false
  },
  plugins: [react(), wasmPack(['./pdf-writer-wasm'])]
})