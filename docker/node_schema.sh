#!/bin/bash

export NODE_ENV="development"
export BABEL_ENV="development"

export NODE_TLS_REJECT_UNAUTHORIZED="0"

export REACT_APP_GRAPHQL_ENDPOINT="http://localhost:3333/graphql"
export REACT_APP_SERVER_GRAPHQL_ENDPOINT="http://gateway:8000/graphql"
export REACT_APP_GRAPHQL_ENDPOINT_NODEJS="http://gateway:8000/graphql"

export REACT_APP_HOST="https://stable.stq.cloud"

cd front
test -d .git \
    || { git init && git remote add origin https://stqcommon:Tz-q6qDL%23d3Mz8hm@github.com/StoriqaTeam/front.git ; }
git fetch \
    && git checkout master \
    && git pull origin master
rm -f yarn.lock
yarn -s --no-progress --non-interactive --production=false \
    && yarn updateSchema \
    && yarn relay

