use sea_orm_migration::{ prelude::*, sea_orm::Statement };

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // 1. 插入基础分类数据
        let category_stmt = Statement::from_sql_and_values(
            manager.get_database_backend(),
            r#"
            INSERT INTO product_category 
                (id, name, parent_id, level, status, create_time, is_deleted)
            VALUES
                (1, '交通工具', 0, 1, 0, CURRENT_TIMESTAMP, 0),
                (2, '新能源车', 1, 2, 0, CURRENT_TIMESTAMP, 0),
                (3, '纯电动车', 2, 3, 0, CURRENT_TIMESTAMP, 0)
            ON CONFLICT (id) DO NOTHING;"#,
            []
        );
        db.execute(category_stmt).await?;

        // 2. 插入品牌数据
        let brand_stmt = Statement::from_sql_and_values(
            manager.get_database_backend(),
            r#"
            INSERT INTO product_brand 
  (id, name, "desc", pic, sort, create_time, is_deleted)
 VALUES
(1, '小米汽车', '智慧出行新选择', 'https://example.com/xiaomi-car-logo.png',1,CURRENT_TIMESTAMP, 0)
ON CONFLICT (id) DO NOTHING;
"#,
            []
        );
        db.execute(brand_stmt).await?;

        // 3. 插入属性分类
        let attr_category_stmt = Statement::from_sql_and_values(
            manager.get_database_backend(),
            r#"
            INSERT INTO product_attribute_category 
                (id, name, create_time, is_deleted)
            VALUES
                (1, '外观配置', CURRENT_TIMESTAMP, 0),
                (2, '内饰配置', CURRENT_TIMESTAMP, 0),
                (3, '智能选装', CURRENT_TIMESTAMP, 0)
            ON CONFLICT (id) DO NOTHING;"#,
            []
        );
        db.execute(attr_category_stmt).await?;

        // 4. 插入商品属性
        let attributes_stmt = Statement::from_sql_and_values(
            manager.get_database_backend(),
            r#"
            INSERT INTO product_attribute 
                (product_attribute_category_id, name, option_status, option_list, type, sort, create_time, is_deleted)
            VALUES
                -- 外观配置
                (1, '车身颜色', 1, '["宝石蓝","珍珠白","朱砂红","矿石银"]', 0, 1, CURRENT_TIMESTAMP, 0),
                (1, '轮毂样式', 1, '["运动轮毂","豪华轮毂"]', 0, 2, CURRENT_TIMESTAMP, 0),
                -- 内饰配置  
                (2, '座椅材质', 1, '["真皮座椅","人造皮革","织物座椅","仿皮材质"]', 0, 2, CURRENT_TIMESTAMP, 0),
                -- 智能选装
                (3, '自动驾驶', 1, '["基础L2","全自动驾驶"]', 0, 1, CURRENT_TIMESTAMP, 0),
                (3, '音响系统', 1, '["基础音响","高级音响"]', 0, 2, CURRENT_TIMESTAMP, 0)
            ON CONFLICT (id) DO NOTHING;"#,
            []
        );
        db.execute(attributes_stmt).await?;

        // 5. 插入SPU数据（原用户提供的代码）
        let spu_stmt = Statement::from_string(
            manager.get_database_backend(),
            r#"
            INSERT INTO product_spu (
    category_id, 
    brand_id, 
    name, 
    product_sn, 
    pic, 
    photo_album, 
    price, 
    promotion_price, 
    promotion_start_time, 
    promotion_end_time, 
    sub_title, 
    sales, 
    unit, 
    detail, 
    publish_status, 
    new_status, 
    recommand_status, 
    create_time, 
    is_deleted
) VALUES 
-- 小米 SU7 系列
(3, 1, '小米 SU7', 'SU7-001', 'https://example.com/xiaomi-su7.jpg', 'https://example.com/su7-album1.jpg,https://example.com/su7-album2.jpg', 29999, 27999, '2024-10-01 00:00:00', '2024-10-31 23:59:59', '高性能智能汽车', 0, '辆', '{"电机类型":"后驱单电机","电池容量":"73.6kWh","CLTC续航":"668km","0-100km/h加速":"5.28秒"}', 1, 1, 1, CURRENT_TIMESTAMP, 0),
(3, 1, '小米 SU7 Pro', 'SU7-Pro-002', 'https://example.com/xiaomi-su7-pro.jpg', 'https://example.com/su7-pro-album1.jpg,https://example.com/su7-pro-album2.jpg', 32999, 30999, '2024-10-01 00:00:00', '2024-10-31 23:59:59', '长续航后驱单电机版', 0, '辆', '{"电机类型":"后驱单电机","电池容量":"89.5kWh","CLTC续航":"800km","0-100km/h加速":"5.7秒"}', 1, 1, 1, CURRENT_TIMESTAMP, 0),
(3, 1, '小米 SU7 Max', 'SU7-Max-003', 'https://example.com/xiaomi-su7-max.jpg', 'https://example.com/su7-max-album1.jpg,https://example.com/su7-max-album2.jpg', 35999, 33999, '2024-10-01 00:00:00', '2024-10-31 23:59:59', '双电机全轮驱动性能版', 0, '辆', '{"电机类型":"双电机全轮驱动","电池容量":"101kWh","CLTC续航":"1000km","0-100km/h加速":"3.9秒"}', 1, 1, 1, CURRENT_TIMESTAMP, 0),
(3, 1, '小米 SU7 Ultra', 'SU7-Ultra-004', 'https://example.com/xiaomi-su7-ultra.jpg', 'https://example.com/su7-ultra-album1.jpg,https://example.com/su7-ultra-album2.jpg', 39999, 37999, '2024-10-01 00:00:00', '2024-10-31 23:59:59', '极致性能全轮驱动版', 0, '辆', '{"电机类型":"双电机全轮驱动","电池容量":"101kWh","CLTC续航":"1000km","0-100km/h加速":"3.2秒"}', 1, 1, 1, CURRENT_TIMESTAMP, 0),
-- 小米 YU7 系列
(3, 1, '小米 YU7', 'YU7-005', 'https://example.com/xiaomi-yu7.jpg', 'https://example.com/yu7-album1.jpg,https://example.com/yu7-album2.jpg', 25999, 23999, '2024-10-01 00:00:00', '2024-10-31 23:59:59', '入门级智能电动轿车', 0, '辆', '{"电机类型":"后驱单电机","电池容量":"62kWh","CLTC续航":"500km","0-100km/h加速":"6.5秒"}', 1, 1, 1, CURRENT_TIMESTAMP, 0),
(3, 1, '小米 YU7 Pro', 'YU7-Pro-006', 'https://example.com/xiaomi-yu7-pro.jpg', 'https://example.com/yu7-pro-album1.jpg,https://example.com/yu7-pro-album2.jpg', 28999, 26999, '2024-10-01 00:00:00', '2024-10-31 23:59:59', '长续航智能电动轿车', 0, '辆', '{"电机类型":"后驱单电机","电池容量":"75kWh","CLTC续航":"650km","0-100km/h加速":"6.0秒"}', 1, 1, 1, CURRENT_TIMESTAMP, 0),
(3, 1, '小米 YU7 Max', 'YU7-Max-007', 'https://example.com/xiaomi-yu7-max.jpg', 'https://example.com/yu7-max-album1.jpg,https://example.com/yu7-max-album2.jpg', 31999, 29999, '2024-10-01 00:00:00', '2024-10-31 23:59:59', '双电机长续航版', 0, '辆', '{"电机类型":"双电机全轮驱动","电池容量":"85kWh","CLTC续航":"800km","0-100km/h加速":"4.5秒"}', 1, 1, 1, CURRENT_TIMESTAMP, 0)"#.to_string()
        );
        db.execute(spu_stmt).await?;

        // 6. 插入SKU示例数据
        let sku_stmt = Statement::from_sql_and_values(
            manager.get_database_backend(),
            r#"

            "#,
            []
        );
        db.execute(sku_stmt).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 清理数据（可选）
        let _ = manager
            .get_connection()
            .execute(
                Statement::from_string(
                    manager.get_database_backend(),
                    "TRUNCATE TABLE product_sku, product_spu, product_attribute_value, product_attribute, product_attribute_category, product_brand, product_category RESTART IDENTITY CASCADE;"
                )
            ).await;

        Ok(())
    }
}
