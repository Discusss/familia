FROM node:18.13-bullseye-slim

USER root

WORKDIR /cabras
COPY package*.json ./

RUN npm install --force
COPY . .
RUN npm run build

RUN rm -rf src

ENTRYPOINT ["npm", "run", "start"]