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

#[derive( Queryable,Insertable,AsChangeset,Debug, Clone, Serialize, Deserialize)]
#[table_name = "u_user"]
#[primary_key("id")]
#[changeset_for(u_users)]
pub struct User {
    pub id: i32,
    pub user: String,
    pub name: String,
    pub pwd: String,
    pub flag: i16,
    pub email: String,
    pub create_time: i32,
}

#[derive(Insertable,AsChangeset)]
#[table_name = "u_user"]
pub struct NewUser {
    pub user: String,
    pub name: String,
    pub pwd: String,
    pub flag: i16,
    pub email: String,
    pub create_time: i32,
}
impl From<&User> for NewUser{
    fn from(src: &User) -> Self {
        Self{
            user: src.user.to_owned(),
            name: src.name.to_owned(),
            pwd: src.pwd.to_owned(),
            flag: src.flag,
            email: src.email.to_owned(),
            create_time: src.create_time
        }
    }
}
