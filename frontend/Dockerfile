FROM node:latest

WORKDIR /app/frontend

COPY package*.json ./
RUN npm i
RUN npm install vite

COPY . .

# grab assets folder from build context "assets"
COPY --from=assets ./assets/cards.json ../assets/cards.json

EXPOSE 5173

# CMD ["npx", "vite", "src", "--host", "0.0.0.0"]

# use vite.config.ts
CMD ["npx", "vite"]
