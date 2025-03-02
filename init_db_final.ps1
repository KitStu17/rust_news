# 디버깅 활성화
$DebugPreference = "Continue"

# 오류 발생 시 스크립트 중단
$ErrorActionPreference = "Stop"

# sqlx 설치 확인
if (-not (Get-Command sqlx -ErrorAction SilentlyContinue)) {
    Write-Error "Error: sqlx is not installed."
    Write-Error "Use:"
    Write-Error "    cargo install --version='~0.8' sqlx-cli --no-default-features --features rustls,postgres"
    Write-Error "to install it."
    exit 1
}

# 환경 변수 설정 (기본값 포함)
$env:DB_PORT = if ($env:DB_PORT) { $env:DB_PORT } else { "5432" }
$env:SUPERUSER = if ($env:SUPERUSER) { $env:SUPERUSER } else { "postgres" }
$env:SUPERUSER_PWD = if ($env:SUPERUSER_PWD) { $env:SUPERUSER_PWD } else { "password" }
$env:APP_USER = if ($env:APP_USER) { $env:APP_USER } else { "app" }
$env:APP_USER_PWD = if ($env:APP_USER_PWD) { $env:APP_USER_PWD } else { "secret" }
$env:APP_DB_NAME = if ($env:APP_DB_NAME) { $env:APP_DB_NAME } else { "newsletter" }

# Docker를 건너뛰지 않는 경우
if (-not $env:SKIP_DOCKER) {
    # 실행 중인 Postgres 컨테이너 확인
    $RUNNING_POSTGRES_CONTAINER = docker ps --filter 'name=postgres' --format '{{.ID}}'
    if ($RUNNING_POSTGRES_CONTAINER) {
        Write-Error "There is a postgres container already running, kill it with"
        Write-Error "    docker kill ${RUNNING_POSTGRES_CONTAINER}"
        exit 1
    }

    # 컨테이너 이름 설정 (고유 이름 생성)
    $CONTAINER_NAME = "postgres_$(Get-Date -UFormat '%s')"

    # Docker로 Postgres 실행
    docker run `
        --env POSTGRES_USER=$env:SUPERUSER `
        --env POSTGRES_PASSWORD=$env:SUPERUSER_PWD `
        --health-cmd="pg_isready -U $env:SUPERUSER || exit 1" `
        --health-interval=1s `
        --health-timeout=5s `
        --health-retries=5 `
        --publish "$($env:DB_PORT):5432" `
        --detach `
        --name $CONTAINER_NAME `
        postgres -N 1000

    # 컨테이너가 건강한 상태가 될 때까지 대기
    do {
        $HEALTH_STATUS = docker inspect -f "{{.State.Health.Status}}" $CONTAINER_NAME
        if ($HEALTH_STATUS -ne "healthy") {
            Write-Host "Postgres is still unavailable - sleeping"
            Start-Sleep -Seconds 1
        }
    } until ($HEALTH_STATUS -eq "healthy")

    # 애플리케이션 사용자 생성
    $CREATE_QUERY = "CREATE USER $env:APP_USER WITH PASSWORD '$env:APP_USER_PWD';"
    docker exec -it $CONTAINER_NAME psql -U $env:SUPERUSER -c $CREATE_QUERY

    # 애플리케이션 사용자에게 DB 생성 권한 부여
    $GRANT_QUERY = "ALTER USER $env:APP_USER CREATEDB;"
    docker exec -it $CONTAINER_NAME psql -U $env:SUPERUSER -c $GRANT_QUERY
}

Write-Host "Postgres is up and running on port $env:DB_PORT - running migrations now!"

# 애플리케이션 데이터베이스 생성
$env:DATABASE_URL = "postgres://$env:APP_USER:$env:APP_USER_PWD@localhost:$env:DB_PORT/$env:APP_DB_NAME"
sqlx database create
sqlx migrate run

Write-Host "Postgres has been migrated, ready to go!"