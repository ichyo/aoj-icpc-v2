FROM node:12 as dev
WORKDIR /app
COPY package.json .
COPY package-lock.json .
RUN npm install

FROM dev as build
COPY public public/
COPY src src/
COPY tsconfig.json .
RUN npm run-script build

FROM nginx:alpine
COPY --from=build /app/build /usr/share/nginx/html
