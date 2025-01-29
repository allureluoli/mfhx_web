use crate::data_structure::{MemberBaseInfo,MemberInfo};
use rusqlite::{Connection, Result};
// 函数式查询


// 返回全部数据
pub fn query_data(conn: &Connection) -> Result<Vec<MemberBaseInfo>> {
    let mut stmt = conn.prepare("SELECT id, name, position, avatar FROM members")?;
    
    let members = stmt.query_map([], |row| {
        Ok(MemberBaseInfo {
            id: row.get(0)?,
            name: row.get(1)?,
            position: row.get(2)?,
            avatar: row.get(3)?,
        })
    })?;

    let mut result = Vec::new();
    for member in members {
        result.push(member?);
    }

    Ok(result)
}



// 通过id 返回数据
pub fn query_id_data(conn: &Connection, id: u32) -> Result<MemberInfo> {
    let mut stmt = conn.prepare("SELECT id, name, position, avatar, introduce FROM members WHERE id = ?")?;
    
    stmt.query_row([id], |row| {
        Ok(MemberInfo {
            id: row.get(0)?,
            name: row.get(1)?,
            position: row.get(2)?,
            avatar: row.get(3)?,
            introduction: row.get(4)?
        })
    })
}

pub fn info_number(conn: &Connection) -> Result<i32> {
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM members")?;
    
    let count: i32 = stmt.query_row([], |row| row.get(0))?;
    Ok(count)
}