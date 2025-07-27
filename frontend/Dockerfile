FROM node:latest

WORKDIR /app/frontend

COPY package*.json ./
RUN npm i
RUN npm install vite

COPY . .

EXPOSE 5173

# CMD ["npx", "vite", "src", "--host", "0.0.0.0"]

# use vite.config.js
CMD ["npx", "vite"]
