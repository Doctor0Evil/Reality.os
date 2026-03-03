import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import { VitePWA } from 'vite-plugin-pwa';
import path from 'path';

export default defineConfig({
  plugins: [
    react(),
    VitePWA({
      registerType: 'autoUpdate',
      includeAssets: ['favicon.ico', 'robots.txt', 'apple-touch-icon.png'],
      manifest: {
        name: 'Reality.OS - Augmented Citizen Portal',
        short_name: 'Reality.OS',
        description: 'Sovereign biophysical dApp for augmented citizens',
        theme_color: '#2c3e50',
        background_color: '#f8f9fa',
        display: 'standalone',
        orientation: 'portrait',
        icons: [
          {
            src: '/icon-192x192.png',
            sizes: '192x192',
            type: 'image/png',
          },
          {
            src: '/icon-512x512.png',
            sizes: '512x512',
            type: 'image/png',
          },
          {
            src: '/icon-512x512.png',
            sizes: '512x512',
            type: 'image/png',
            purpose: 'any maskable',
          },
        ],
      },
      workbox: {
        globPatterns: ['**/*.{js,css,html,ico,png,svg,woff2}'],
        runtimeCaching: [
          {
            urlPattern: /^https:\/\/rpc\.bostrom\.cybernode\.ai\/.*/i,
            handler: 'NetworkFirst',
            options: {
              cacheName: 'aln-rpc-cache',
              expiration: {
                maxEntries: 50,
                maxAgeSeconds: 60 * 60 * 24, // 24 hours
              },
              cacheableResponse: {
                statuses: [0, 200],
              },
            },
          },
          {
            urlPattern: /^https:\/\/lcd\.bostrom\.cybernode\.ai\/.*/i,
            handler: 'NetworkFirst',
            options: {
              cacheName: 'aln-lcd-cache',
              expiration: {
                maxEntries: 50,
                maxAgeSeconds: 60 * 60 * 24,
              },
              cacheableResponse: {
                statuses: [0, 200],
              },
            },
          },
        ],
      },
    }),
  ],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
      '@components': path.resolve(__dirname, './src/components'),
      '@contexts': path.resolve(__dirname, './src/contexts'),
      '@hooks': path.resolve(__dirname, './src/hooks'),
      '@pages': path.resolve(__dirname, './src/pages'),
      '@types': path.resolve(__dirname, './src/types'),
      '@services': path.resolve(__dirname, './src/services'),
      '@cyberspectre': path.resolve(__dirname, './src/cyberspectre'),
    },
  },
  server: {
    port: 3000,
    host: true,
    proxy: {
      '/api': {
        target: 'http://localhost:9001', // CozoDB
        changeOrigin: true,
      },
      '/sovereignty': {
        target: 'http://localhost:8080', // SovereigntyCore
        changeOrigin: true,
      },
    },
  },
  build: {
    outDir: 'dist',
    sourcemap: true,
    rollupOptions: {
      output: {
        manualChunks: {
          vendor: ['react', 'react-dom', 'react-router-dom'],
          aln: ['@cosmjs/stargate', '@cosmjs/proto-signing'],
          cozodb: ['cozo-client'],
        },
      },
    },
  },
  define: {
    'process.env.NODE_ENV': JSON.stringify(process.env.NODE_ENV),
    'process.env.VITE_ALN_RPC_URL': JSON.stringify(process.env.VITE_ALN_RPC_URL || 'https://rpc.bostrom.cybernode.ai'),
    'process.env.VITE_ALN_LCD_URL': JSON.stringify(process.env.VITE_ALN_LCD_URL || 'https://lcd.bostrom.cybernode.ai'),
    'process.env.VITE_COZODB_URL': JSON.stringify(process.env.VITE_COZODB_URL || 'http://localhost:9001'),
  },
});
