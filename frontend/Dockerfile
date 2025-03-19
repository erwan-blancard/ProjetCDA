FROM node:latest

WORKDIR /app

COPY package*.json ./
RUN npm i
RUN npm install vite

COPY . .

EXPOSE 5173

CMD ["npx", "vite", "--host"]
