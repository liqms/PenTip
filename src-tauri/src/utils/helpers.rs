// 工具函数
#[allow(dead_code)]
pub fn generate_id() -> String {
    uuid::Uuid::new_v4().to_string()
}
