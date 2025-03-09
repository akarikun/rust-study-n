use dotenv::dotenv;
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, DbErr, EntityOrSelect, EntityTrait, PaginatorTrait,
    QueryFilter,
};
use std::io::Write;
use std::{env, fs};

pub fn init_env() {
    let env_file_path = ".env";
    if !std::path::Path::new(env_file_path).exists() {
        println!("未找到配置,正在生成默认配置...");
        let mut file = fs::File::create(env_file_path).expect("Failed to create .env file");
        writeln!(file, "HOST=127.0.0.1:3000").expect("写入配置文件异常");
        writeln!(file, "DATABASE_URL=sqlite://data.db?mode=rwc").expect("写入配置文件异常");
        println!("生成成功,配置更改后请重启程序");
    }
    dotenv().ok();
}

pub fn get_cfg(key: &str) -> Result<String, env::VarError> {
    std::env::var(key)
}

pub async fn get_db() -> Result<DatabaseConnection, DbErr> {
    let config_path = env::var("DATABASE_URL").expect("配置<DATABASE_URL>不存在");
    let db = sea_orm::Database::connect(&config_path).await?;
    Ok(db)
}
