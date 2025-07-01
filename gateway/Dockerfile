# -------- Build Stage --------
FROM node:24-alpine3.21 AS build

WORKDIR /app

COPY package.json yarn.lock ./
RUN yarn install --frozen-lockfile

COPY . .

RUN yarn build

# -------- Production Stage --------
FROM node:24-alpine3.21 AS prod

WORKDIR /app

COPY package.json yarn.lock ./
RUN yarn install --frozen-lockfile --production --prefer-offline

COPY --from=build /app/dist ./dist
COPY --from=build /app/migrations ./dist/migrations

RUN rm -rf node_modules/.cache

RUN yarn cache clean

ENV NODE_ENV=production
EXPOSE 8000
CMD ["node", "dist/src/main.js"]
