use crate::data_structure::MemberBaseInfo;
use rusqlite::{Connection, Result};
// 函数式查询


// 返回全部数据
pub fn query_data(conn: &Connection) -> Result<MemberBaseInfo> {
    let mut stmt = conn.prepare("SELECT id, name, position, avatar FROM member")?;
    
    stmt.query_row([], |row| {
        Ok(MemberBaseInfo {
            id: row.get(0)?,
            name: row.get(1)?,
            position: row.get(2)?,
            avatar: row.get(3)?,
        })
    })
}

// 通过id 返回数据
pub fn query_id_data(conn: &Connection, id: u32) -> Result<MemberBaseInfo> {
    let mut stmt = conn.prepare("SELECT id, name, position, avatar FROM member WHERE id = ?")?;
    
    stmt.query_row([id], |row| {
        Ok(MemberBaseInfo {
            id: row.get(0)?,
            name: row.get(1)?,
            position: row.get(2)?,
            avatar: row.get(3)? 
        })
    })
}