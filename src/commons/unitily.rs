use super::config::Config;
use migration::{Migrator, MigratorTrait};
use salvo::__private::tracing;
use sea_orm::{DatabaseConnection, DbErr};
use std::cell::OnceCell;
use std::sync::LazyLock;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

static mut DB_INIT: OnceCell<bool> = OnceCell::new();
pub static CONFIG: LazyLock<Config> = LazyLock::new(|| Config::load().unwrap());

pub fn init_logger() {
    if CONFIG.dbg {
        let subscriber = FmtSubscriber::builder()
            .with_env_filter(EnvFilter::new("sea_orm=debug,sqlx=debug")) // 确保日志级别正确
            .with_target(false) // 关闭目标前缀
            .with_ansi(true) // 允许 ANSI 颜色
            .finish();
        tracing::subscriber::set_global_default(subscriber).expect("Failed to set logger");
    }
}

pub async fn get_db() -> Result<DatabaseConnection, DbErr> {
    let db_init = unsafe { DB_INIT.get().or(Some(&false)).unwrap() };
    let db = sea_orm::Database::connect(&CONFIG.db).await?;
    if *db_init {
        return Ok(db);
    }
    Migrator::up(&db, None).await?;
    unsafe {
        DB_INIT.set(true).unwrap();
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
