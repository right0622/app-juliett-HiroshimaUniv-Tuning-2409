#!/bin/bash

# ==================================
# E2Eテストスクリプト。
# ==================================

if [[ $HOSTNAME == app-* ]];
then
	BASE_URL="https://${HOSTNAME}.ftt2407.dabaas.net"
elif [[ $GITHUB_ACTIONS ]];
then
	BASE_URL="http://host.docker.internal"
else
	BASE_URL="http://nginx"
fi

# E2Eテスト開始
echo "E2Eテストを開始します。"

if [[ $GITHUB_ACTIONS ]];
then
    docker run --rm --add-host=host.docker.internal:host-gateway \
        -e BASE_URL=${BASE_URL} \
        -i hirouniv2409.azurecr.io/e2e:development \
        yarn test
else
    docker run --name e2e --rm --network webapp-network \
        -e BASE_URL=${BASE_URL} \
        -it hirouniv2409.azurecr.io/e2e:development \
        yarn test
fi


if [ $? -ne 0 ]; then
    echo "E2Eのテストに失敗しました。"
    exit 1
fi
