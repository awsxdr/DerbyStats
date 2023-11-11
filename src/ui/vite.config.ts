import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import path from 'path';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      "@atoms": path.resolve(__dirname, "src/components/atoms/"),
      "@components": path.resolve(__dirname, "src/components/"),
      "@contexts": path.resolve(__dirname, "src/contexts/"),
      "@controls": path.resolve(__dirname, "src/components/controls/"),
      "@hooks": path.resolve(__dirname, "src/hooks/"),
      "@pages": path.resolve(__dirname, "src/components/pages/")
    }
  }
})
