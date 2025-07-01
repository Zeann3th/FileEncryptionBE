use sea_orm::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "files")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub size: String,
    pub encryption_method: EncryptionMethod,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Text", enum_name = "encryption_method")]
pub enum EncryptionMethod {
    #[sea_orm(string_value = "aes256gcm")]
    Aes256Gcm,
    #[sea_orm(string_value = "aes128gcm")]
    Aes128Gcm,
    #[sea_orm(string_value = "chacha20poly1305")]
    ChaCha20Poly1305,
}

impl std::fmt::Display for EncryptionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            EncryptionMethod::Aes256Gcm => "aes256gcm",
            EncryptionMethod::Aes128Gcm => "aes128gcm",
            EncryptionMethod::ChaCha20Poly1305 => "chacha20poly1305",
        };
        write!(f, "{}", s)
    }
}
