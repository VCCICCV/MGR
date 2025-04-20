# 注意

迁移后sea_orm_active_enums中的Status会默认驼峰命名，请修改为大写

```rust
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "Status")]
pub enum Status {
    #[sea_orm(string_value = "BANNED")]
    BANNED,
    #[sea_orm(string_value = "DISABLED")]
    DISABLED,
    #[sea_orm(string_value = "ENABLED")]
    ENABLED,
}
```
