## 快速开始

安装sea-orm-cli

```cmd
cargo add seaorm-cli
```

运行迁移

```cmd
sea-orm-cli migrate up
```

或删除所有表重新迁移

```cmd
sea-orm-cli migrate fresh
```

启动应用

```cmd
cargo run
```

| CRUD     | 命名约定 |
| -------- | ----- |
| 添加     | Save|
| 删除     | Remove|
| 查询单个 | Find|
| 查询多个 | List|
| 分页查询 | Page|
| 统计     | Count |
