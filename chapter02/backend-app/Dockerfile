FROM node:lts-alpine
WORKDIR /usr/src/app
COPY ./dist/backend-app .
EXPOSE 3000
RUN chown -R node /usr/src/app
USER node
CMD ["node", "index"]
