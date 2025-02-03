use serde::Serialize;
use serde::Deserialize;

// 定义成员信息结构

#[derive(Serialize, Deserialize, Debug)]
pub struct MemberInfo {
    pub id: u32,
    pub name: String,
    pub position: String,
    pub avatar: String,
    pub introduction: String
}

// 就是不含介绍的
#[derive(Serialize, Deserialize, Debug)]
pub struct MemberBaseInfo {
    pub id: u32,
    pub name: String,
    pub position: String,
    pub avatar: String,
}
