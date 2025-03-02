# 환경 변수 설정
$env:DB_USER = if ($env:POSTGRES_USER) { $env:POSTGRES_USER } else { "admin" }
$env:DB_PASSWORD = if ($env:POSTGRES_PASSWORD) { $env:POSTGRES_PASSWORD } else { "cyan6077" }
$env:DB_NAME = if ($env:POSTGRES_DB) { $env:POSTGRES_DB } else { "rust_news" }
$env:DB_PORT = if ($env:POSTGRES_PORT) { $env:POSTGRES_PORT } else { "5433" }

# 컨테이너명 설정
$CONTAINER_NAME = "rust-postgres"

# psql 설치 확인
if (-not (Get-Command psql -ErrorAction SilentlyContinue)) {
    Write-Error "Error: psql is not installed."
    exit 1
}

# sqlx 설치 확인
if (-not (Get-Command sqlx -ErrorAction SilentlyContinue)) {
    Write-Error "Error: sqlx is not installed."
    Write-Error "install..."
    Write-Error "cargo install --version='~0.6' sqlx-cli --no-default-features --features rustls,postgres"
    exit 1
}

# 컨테이너가 이미 존재하는지 확인
$existingContainer = docker ps -a --filter "name=$CONTAINER_NAME" --format "{{.Names}}"

if ($existingContainer) {
    # 컨테이너가 존재하면 실행 상태 확인
    $isRunning = docker ps --filter "name=$CONTAINER_NAME" --format "{{.Names}}"

    if ($isRunning) {
        Write-Host "Container '$CONTAINER_NAME' is already running..."
    } else {
        Write-Host "Container '$CONTAINER_NAME' is stopped. Starting it now..."
        docker start $CONTAINER_NAME
    }
} else {
    # 컨테이너가 없으면 새로 생성하고 실행
    Write-Host "Creating Container '$CONTAINER_NAME'..."
    # Docker 컨테이너 실행
    docker run `
        --name $CONTAINER_NAME `
        -e POSTGRES_USER=$env:DB_USER `
        -e POSTGRES_PASSWORD=$env:DB_PASSWORD `
        -e POSTGRES_DB=$env:DB_NAME `
        -p "$($env:DB_PORT):5432" `
        -v $(pwd)/migrations:/migrations `
        -d postgres `
        postgres -N 1000
}

# 환경 변수 설정
$env:PGPASSWORD = $env:DB_PASSWORD

# Postgres가 명령어를 받아들일 준비가 될 때까지 대기
do {
    try {
        psql -h localhost -U $env:DB_USER -p $env:DB_PORT -d postgres -c "\q"
        $POSTGRES_READY = $true
    } catch {
        Write-Host "Postgres is still unavailable - sleeping"
        Start-Sleep -Seconds 1
        $POSTGRES_READY = $false
    }
} until ($POSTGRES_READY)

Write-Host "Postgres is up and running on port $env:DB_PORT! - running migrations now!"

# 데이터베이스 URL 설정
set DATABASE_URL="postgres://$env:DB_USER:$env:DB_PASSWORD@localhost:$env:DB_PORT/$env:DB_NAME"
Write-Host "DATABASE_URL: $env:DATABASE_URL"

# DB 마이그레이션 생성
# slqx migration add create_subscriptions_table

# DB 마이그레이션 실행
# sqlx migrate run --database-url postgres://$env:DB_USER:$env:DB_PASSWORD@localhost:$env:DB_PORT/$env:DB_NAME

# sqlx db query
sqlx query "SELECT * FROM subscriptions"