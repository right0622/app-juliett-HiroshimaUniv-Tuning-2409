#!/bin/bash

# ==================================
# 負荷試験スクリプト。
# ==================================


if [[ $HOSTNAME == benchmarker ]];
then
    JOB_ID=$1
    TEAM_ID=$2
    FILE_NAME=$3
    COMMIT_ID=$4
    TIMESTAMP=$5
	CLIENT_ORIGIN_URL="https://$TEAM_ID.ftt2407.dabaas.net"
else
	CLIENT_ORIGIN_URL="http://nginx"
    FILE_NAME=`date "+%Y%m%d_%H%M%S"`
fi

LOG_FILE_PATH="./logs/${FILE_NAME}.json"
RAW_DATA_FILE_PATH="./scores/raw-data-${FILE_NAME}.json"
SCORE_FILE_PATH="./scores/score-${FILE_NAME}.json"

# 負荷試験開始
echo "負荷試験を開始します。"

if [[ $HOSTNAME == benchmarker ]];
then
    k6 run --out json=${LOG_FILE_PATH} main.js -e CLIENT_ORIGIN_URL=${CLIENT_ORIGIN_URL} -e RAW_DATA_FILE_PATH=${RAW_DATA_FILE_PATH} && \
    bash ./calculate_score.sh $LOG_FILE_PATH $SCORE_FILE_PATH $RAW_DATA_FILE_PATH $JOB_ID $TEAM_ID $COMMIT_ID $TIMESTAMP
else
    docker run --name k6 --rm --network webapp-network \
      -v $(pwd):/usr/src/benchmarker \
      -v /usr/src/benchmarker/node_modules \
      -it hirouniv2409.azurecr.io/benchmarker:latest \
      /bin/bash -c "k6 run --log-output=file=${LOG_FILE_PATH},level=warning --log-format json main.js -e CLIENT_ORIGIN_URL=${CLIENT_ORIGIN_URL} -e RAW_DATA_FILE_PATH=${RAW_DATA_FILE_PATH} && bash ./calculate_score.sh $LOG_FILE_PATH $SCORE_FILE_PATH $RAW_DATA_FILE_PATH"
fi
