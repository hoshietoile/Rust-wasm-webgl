import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import path from 'path';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      "~/": path.join(__dirname, "src/"),
      "@/": path.join(__dirname, "src/"),
      "@components/": path.join(__dirname, "src/components/"),
      "@atoms/": path.join(__dirname, "src/components/atoms/"),
      "@molecules/": path.join(__dirname, "src/components/molecules/"),
      "@organisms/": path.join(__dirname, "src/components/organisms/"),
      "@pages/": path.join(__dirname, "src/components/pages/"),
      "@hooks/": path.join(__dirname, "src/hooks/"),
      "@types/": path.join(__dirname, "src/@types/"),
    }
  }
})
