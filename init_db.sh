#!/usr/bin/env bash
set -x
set -eo pipefail

# 커스텀 유저 설정
DB_USER=${POSTGRES_USER:=admin}
# 커스텀 비밀번호 설정
DB_PASSWORD="${POSTGRES_PASSWORD:=cyan6077}"
# 커스텀 데이터베이스 이름 설정
DB_NAME=${POSTGRES_DB:=rust_news}
# 커스텀 포트 설정
DB_PORT="${POSTGRES_PORT:=5433}"

# 도커로 postgres 구동
docker run \
  -e POSTGRES_USER=${DB_USER} \
  -e POSTGRES_PASSWORD=${DB_PASSWORD} \
  -e POSTGRES_DB=${DB_NAME} \
  -p ${DB_PORT}:5433 \
  -d postgres \
  postgres -N 1000
  # ^ 테스트 목적으로 커넥션 최대치로 설정
