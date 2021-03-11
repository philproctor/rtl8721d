FROM node:lts-alpine

RUN npm install -g @vue/cli && \
    yarn global add @vue/cli-service-global
