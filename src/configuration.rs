#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // 구성 읽기 초기화
    let settings = config::Config::builder()
    // 'config.yaml'로부터 구성값 추가(.json 파일도 가능)
    .add_source(
        config::File::new("configuration.yaml", config::FileFormat::Yaml)
    )
    .build()?;

    // 읽어들인 구성값을 Settings 타입으로 변환
    settings.try_deserialize::<Settings>()
}

// postgresql 과 연결하기 위한 설정값 반환
impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password,
            self.host,
            self.port,
            self.database_name
        )
    }
}