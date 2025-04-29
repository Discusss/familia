FROM node:18.13-bullseye-slim

USER root

WORKDIR /cabras
COPY . .

RUN npm install --force
RUN npm run build

ENTRYPOINT ["npm", "run", "start"]