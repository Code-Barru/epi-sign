FROM node:24-alpine3.21 AS build

WORKDIR /app

COPY ./package*.json ./
RUN npm ci

COPY . .

RUN npm run build


FROM nginx:alpine

RUN rm -rf /usr/share/nginx/html/*

COPY ./nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=build /app/build /usr/share/nginx/html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
