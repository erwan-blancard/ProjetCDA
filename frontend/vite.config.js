import { defineConfig } from 'vite'
import fs from 'fs'
import path from 'path'


/** @type {import('vite').UserConfig} */
export default defineConfig({
    root: './src',
    server: {
        host: '0.0.0.0',
    },
    test: {
      environment: 'jsdom',
    },
    plugins: [
      // dev only: resolve cards.json from parent folder as if it was in public dir
      {
        name: "cards-assets-middleware",
        configureServer(server) {
          server.middlewares.use((req, res, next) => {
            if (req.url === "/assets/cards.json") {
              const filePath = path.resolve(__dirname, '../assets/cards.json')
              fs.createReadStream(filePath).pipe(res)
              return
            }
            next()
          })
        }
      }
    ]
})
