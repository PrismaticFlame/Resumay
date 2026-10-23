use rusqlite::{params, Connection, Result};

#[derive(Debug, serde::Serialize)]
pub struct Application {
    id: i32,
    company: String,
    position: String,
    website: Option<String>,
    job_description: Option<String>,
    username: Option<String>,
    status: String,
    resume: Option<i32>,
    notes: Option<String>,
    refer: Option<String>,
    apply_date: String,
}

pub fn init(conn: &Connection) -> Result<()> {
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;
    conn.execute_batch(include_str!("schema.sql"))?;
    Ok(())
}

pub fn add_application(conn: &Connection, 
                        company: &str, 
                        position: &str,
                        website: Option<&str>,
                        job_description: Option<&str>,
                        username: Option<&str>,
                        status: &str,
                        resume: Option<i32>,
                        notes: Option<&str>,
                        refer: Option<&str>,
                        apply_date: &str) -> Result<i32> {
    conn.execute(
        "INSERT INTO   applications (company, position, website, job_description, username, status, resume, notes, refer, apply_date)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![company, position, website, job_description, username, status, resume, notes, refer, apply_date]
    )?;
    
    return Ok(conn.last_insert_rowid().try_into().unwrap())
}


pub fn list_applications(conn: &Connection) -> Result<Vec<Application>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, company, position, website, job_description, username, status, resume, notes, refer, apply_date FROM applications"
    )?;
    let appli_iter = stmt.query_map([], |row| {
        Ok(Application {
            id: row.get::<_, i32>(0)?,
            company: row.get::<_, String>(1)?,
            position: row.get::<_, String>(2)?,
            website: row.get::<_, Option<String>>(3)?,
            job_description: row.get::<_, Option<String>>(4)?,
            username: row.get::<_, Option<String>>(5)?,
            status: row.get::<_, String>(6)?,
            resume: row.get::<_, Option<i32>>(7)?,
            notes: row.get::<_, Option<String>>(8)?,
            refer: row.get::<_, Option<String>>(9)?,
            apply_date: row.get::<_, String>(10)?,
        })
    })?;

    let application_list: Vec<Application> = appli_iter.collect::<Result<Vec<_>,_>>()?;

    Ok(application_list)
}

pub fn update_application(conn: &Connection, 
                        id: i32,
                        company: &str, 
                        position: &str,
                        website: Option<&str>,
                        job_description: Option<&str>,
                        username: Option<&str>,
                        status: &str,
                        resume: Option<i32>,
                        notes: Option<&str>,
                        refer: Option<&str>,
                        apply_date: &str) -> Result<usize> {
    let changed = conn.execute(
        "UPDATE applications
        SET company = ?1, position = ?2, website = ?3, job_description = ?4, username = ?5, status = ?6, resume = ?7, notes = ?8, refer = ?9, apply_date = ?10
        WHERE id = ?11",
        params![company, position, website, job_description, username, status, resume, notes, refer, apply_date, id]
    )?;

    Ok(changed)
}


pub fn delete_application(conn: &Connection, id: i32) -> Result<(), String> {
    conn.execute("DELETE FROM applications WHERE id = ?1", [id]).map_err(|e| e.to_string())?;
    Ok(())
}