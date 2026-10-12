use rusqlite::{params, Connection, Result};

#[derive(Debug, serde::Serialize)]
pub struct Application {
    id: i32,
    company: String,
    position: String,
    website: Option<String>,
    status: String,
    resume: Option<i32>,
    notes: Option<String>,
    refer: Option<String>,
    apply_date: String,
}

pub fn init(conn: &Connection) -> Result<()> {
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;
    conn.execute_batch(include_str!("schema.sql"))?;

    //let _app_num = add_application(conn, "Lockheed Martin","SWE", Some("lockheedmartin.com"),
    //                                "applied", None, Some("First Application"), Some("Cindy"), "2026-10-09")?;
    //
    //let appli_list = list_applications(conn)?;

    //println!("{:#?}", appli_list);

    Ok(())
}

pub fn add_application(conn: &Connection, 
                        company: &str, 
                        position: &str,
                        website: Option<&str>,
                        status: &str,
                        resume: Option<i32>,
                        notes: Option<&str>,
                        refer: Option<&str>,
                        apply_date: &str) -> Result<i32> {
    conn.execute(
        "INSERT INTO   applications (company, position, website, status, resume, notes, refer, apply_date)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![company, position, website, status, resume, notes, refer, apply_date]
    )?;
    
    return Ok(conn.last_insert_rowid().try_into().unwrap())
}


pub fn list_applications(conn: &Connection) -> Result<Vec<Application>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, company, position, website, status, resume, notes, refer, apply_date FROM applications"
    )?;
    let appli_iter = stmt.query_map([], |row| {
        Ok(Application {
            id: row.get::<_, i32>(0)?,
            company: row.get::<_, String>(1)?,
            position: row.get::<_, String>(2)?,
            website: row.get::<_, Option<String>>(3)?,
            status: row.get::<_, String>(4)?,
            resume: row.get::<_, Option<i32>>(5)?,
            notes: row.get::<_, Option<String>>(6)?,
            refer: row.get::<_, Option<String>>(7)?,
            apply_date: row.get::<_, String>(8)?,
        })
    })?;

    let application_list: Vec<Application> = appli_iter.collect::<Result<Vec<_>,_>>()?;

    Ok(application_list)
}
