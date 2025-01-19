import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

// https://vite.dev/config/
export default defineConfig({
  plugins: [svelte()],
  server: {
    port: process.env.PORT? parseInt(process.env.PORT) : 5173,
    proxy: {
      '/api': {
        target: 'http://localhost:3000/', // 백엔드 서버 주소
        changeOrigin: true, // 대상 서버의 호스트 헤더를 변경
        // rewrite: (path) => path.replace(/^\/api/, ''), // "/api" 제거
      },
    },
  },
  build: {
    rollupOptions: {
      input: {
        index: 'index.html',
        movie: 'movie.html',
        mypage: 'mypage.html',
      }
    },
    outDir: '../backend/static', // Axum의 정적 파일 경로
    emptyOutDir: true,
  },
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    },
  },
})
