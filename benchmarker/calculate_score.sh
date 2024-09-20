#!/bin/bash

# ==================================
# 採点スクリプト。
# ==================================

LOG_FILE_PATH=$1
SCORE_FILE_PATH=$2
RAW_DATA_FILE_PATH=$3

if [[ $HOSTNAME == benchmarker ]]; then
    JOB_ID=$4
    TEAM_ID=$5
    COMMIT_ID=$6
    TIMESTAMP=$7
fi

node score.mjs $SCORE_FILE_PATH $RAW_DATA_FILE_PATH

# スコアデータを送信
if [[ $HOSTNAME == benchmarker ]];
then
    if [ -f "$SCORE_FILE_PATH" ]; then
        finalScore=$(jq -r '.finalScore' "$SCORE_FILE_PATH")

        raw_score_data="{\"teamId\":\"$TEAM_ID\",\"finalScore\":$finalScore,\"jobId\":\"$JOB_ID\",\"commitId\":\"$COMMIT_ID\",\"timestamp\":\"$TIMESTAMP\"}"
        
        # スコアデータにデジタル署名を施す
        echo -n "$raw_score_data" | openssl dgst -sha256 -sign /usr/src/benchmarker/secrets/benchmarker-priv.pem -out ./scores/signed_score_data-${TEAM_ID}_${TIMESTAMP}.bin

        # 署名結果をJSONデータで渡すためにBase64でエンコード
        encoded_score_data=$(openssl base64 -in ./scores/signed_score_data-${TEAM_ID}_${TIMESTAMP}.bin | tr -d '\n')

        data=$(jq -n \
            --arg rawScoreData "$raw_score_data" \
            --arg encodedScoreData "$encoded_score_data" \
            '{
                rawScoreData: $rawScoreData,
                encodedScoreData: $encodedScoreData,
            }')

        response=$(curl -s -w "%{http_code}" -o /dev/stdout -X POST https://ranking.ftt2407.dabaas.net/api/scores \
              -H "Content-Type: application/json" \
              -d "$data")

        http_status="${response: -3}"
        response_body="${response%${http_status}}"

        if [ "$http_status" -eq 201 ]; then
            echo "Successfully sent score data"
        else
            echo "Failed to send score data: HTTP $http_status"
            echo "Response body: $response_body"
            exit 1
        fi
    fi
else
    SCORE=$(cat ${SCORE_FILE_PATH} | jq -r ".finalScore")

    # パスをrun.shからの相対パスに変換
    LOG_FILE_PATH=$(echo $LOG_FILE_PATH | sed 's|./logs/|./benchmarker/logs/|')
    RAW_DATA_FILE_PATH=$(echo $RAW_DATA_FILE_PATH | sed 's|./scores/|./benchmarker/scores/|')
    SCORE_FILE_PATH=$(echo $SCORE_FILE_PATH | sed 's|./scores/|./benchmarker/scores/|')

    echo -e "\n\n===================================================\n\n"
    echo -e "負荷試験が完了しました！！！"
    echo -e "あなたのスコア: $SCORE\n"
    echo -e "より詳細な情報は下記ファイルをご覧ください"
    echo -e "ログファイル: $LOG_FILE_PATH"
    echo -e "負荷試験詳細ファイル: $RAW_DATA_FILE_PATH"
    echo -e "スコアファイル: $SCORE_FILE_PATH"
    echo -e "\n\n===================================================\n\n"
fi

if [ $? -ne 0 ]; then
    echo "スコアの計算に失敗しました。"
    exit 1
fi
