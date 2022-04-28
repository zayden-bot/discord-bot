FROM node:18

# Create app directory
WORKDIR /usr/src/zayden

# Install app dependencies
# A wildcard is used to ensure both package.json AND package-lock.json are copied
# where available (npm@5+)
COPY package*.json ./

RUN npm install

# Bundle app source
COPY .env.local .
COPY ./build .
COPY ./configs ./configs

ENV NODE_ENV=development

CMD [ "node", "index.js" ]