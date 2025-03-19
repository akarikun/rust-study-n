use dotenv::dotenv;
use migration::{Migrator, MigratorTrait};
use salvo::__private::tracing;
use sea_orm::{
    ColumnTrait, Condition, ConnectOptions, DatabaseConnection, DbErr, EntityOrSelect, EntityTrait,
    PaginatorTrait, QueryFilter,
};
use std::io::Write;
use std::{env, fs};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

static mut db_init: bool = false;
static env_file_path: &str = ".env";

pub fn init_env() {
    if !std::path::Path::new(env_file_path).exists() {
        println!("未找到配置,正在生成默认配置...");
        let mut file = fs::File::create(env_file_path).expect("Failed to create .env file");
        writeln!(file, "HOST=127.0.0.1:3000").expect("写入配置文件异常");
        writeln!(file, "DATABASE_URL=sqlite://data.db?mode=rwc").expect("写入配置文件异常");
        writeln!(file, "DEBUG=true").expect("写入配置文件异常");
        println!("生成成功,配置更改后请重启程序");
    }
    dotenv().ok();
}

pub fn get_cfg(key: &str) -> Result<String, env::VarError> {
    std::env::var(key)
}

pub fn init_logger() {
    let dbg = env::var("DEBUG").unwrap_or(format!("true"));
    let dbg = dbg.parse::<bool>().unwrap_or(true);
    if dbg {
        let subscriber = FmtSubscriber::builder()
            .with_env_filter(EnvFilter::new("sea_orm=debug,sqlx=debug")) // 确保日志级别正确
            .with_target(false) // 关闭目标前缀
            .with_ansi(true) // 允许 ANSI 颜色
            .finish();
        tracing::subscriber::set_global_default(subscriber).expect("Failed to set logger");
    }
}

pub async fn get_db() -> Result<DatabaseConnection, DbErr> {
    let config_path = env::var("DATABASE_URL").expect("配置<DATABASE_URL>不存在");
    let db = sea_orm::Database::connect(&config_path).await?;
    if unsafe { db_init } {
        return Ok(db);
    } else {
        Migrator::up(&db, None).await?;
        unsafe {
            db_init = true;
        }
    }
    Ok(db)
}

pub fn string_default_val<'a>(str: &'a str, def_val: &'a str) -> (&'a str, bool) {
    if str.is_empty() {
        return (def_val, true);
    }
    (str.trim(), false)
}

pub fn log_print(msg: String) {
    if cfg!(debug_assertions) {
        println!("log => {}", msg);
    }
}
