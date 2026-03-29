//! SQLite database handling

use anyhow::Result;
use rusqlite::Connection;
use std::path::Path;

/// Open database connection and run migrations
pub fn open_database(db_path: &Path) -> Result<Connection> {
    let conn = Connection::open(db_path)?;

    // Enable foreign keys
    conn.execute("PRAGMA foreign_keys = ON", [])?;

    // Run migrations
    run_migrations(&conn)?;

    Ok(conn)
}

/// Run database migrations
fn run_migrations(conn: &Connection) -> Result<()> {
    let schema = include_str!("../../../../schemas/sqlite/0001_init.sql");
    conn.execute_batch(schema)?;
    Ok(())
}

/// Initialize database for a project
pub fn init_database(db_path: &Path) -> Result<Connection> {
    // Remove existing database if present
    if db_path.exists() {
        std::fs::remove_file(db_path)?;
    }

    open_database(db_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_database_initialization() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.sqlite");

        let conn = init_database(&db_path)?;

        // Verify tables exist by checking we can query them
        let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table'")?;
        let table_iter = stmt.query_map([], |row| row.get::<_, String>(0))?;

        let mut tables = Vec::new();
        for table in table_iter {
            tables.push(table?);
        }

        assert!(tables.contains(&"datasets".to_string()));
        assert!(tables.contains(&"wells".to_string()));
        assert!(tables.contains(&"surfaces".to_string()));

        Ok(())
    }

    #[test]
    fn test_structural_schema() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test_structural.sqlite");

        let conn = init_database(&db_path)?;

        // Verify tables exist
        let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table'")?;
        let table_iter = stmt.query_map([], |row| row.get::<_, String>(0))?;

        let mut tables = Vec::new();
        for table in table_iter {
            tables.push(table?);
        }

        assert!(
            tables.contains(&"faults".to_string()),
            "faults table should exist"
        );
        assert!(
            tables.contains(&"fault_sticks".to_string()),
            "fault_sticks table should exist"
        );

        Ok(())
    }
}
