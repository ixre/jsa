use crate::schema::u_user;
/// User flag
pub enum UserFlag {
    /// 是否启用
    Enabled = 1,
    /// 是否激活
    Activated = 2,
    /// 是否为超级用户
    SuperUser = 4,
}

#[derive(Debug, Queryable, Clone, Serialize, Deserialize)]
#[table_name = "u_user"]
pub struct User {
    pub id: i32,
    pub user: String,
    pub name: String,
    pub pwd: String,
    pub flag: i16,
    pub email: String,
    pub create_time: i32,
}

#[derive(Insertable)]
#[table_name = "u_user"]
pub struct NewUser {
    pub user: String,
    pub name: String,
    pub pwd: String,
    pub flag: i16,
    pub email: String,
    pub create_time: i32,
}
