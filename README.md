# N3考级1000题

### 项目目录
|目录|描述|
|-|-|
|f7|前端项目|
|web|前端发布目录,Rust项目会嵌入该目录|


### Rust后端开发

#### 1.安装sea-orm-cli
```
cargo install sea-orm-cli
```
#### 2.初始化
```
sea-orm-cli migrate init
```
#### 3.编辑migration/Cargo.toml
```
features = [
  "runtime-tokio-rustls",  # `ASYNC_RUNTIME` feature
  "sqlx-sqlite",         # `DATABASE_DRIVER` feature
  #"sqlx-postgres","sqlx-sqlite",根据库更改
]
```

#### 3.创建实体模板
```
sea-orm-cli migrate generate <entity>
```

#### 4.生成实体类
```
sea-orm-cli migrate up
# sea-orm-cli migrate down
# sea-orm-cli migrate fresh
# sea-orm-cli migrate status
sea-orm-cli generate entity -o src/entities # 生成实体
```

#### 发布可执行文件
```
cd f7 && npm run build && cd .. && cargo build --release
```

#### rust.yaml本地测试
```
# 下载act工具 https://github.com/nektos/act/releases
# 软链接 ln -sf <path>/act /usr/bin/act
act -j build 
```