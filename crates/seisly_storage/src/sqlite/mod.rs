//! SQLite database handling

use anyhow::Result;
use rusqlite::{Connection, Transaction};
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

/// Execute a closure within a database transaction
/// This ensures atomicity - either all operations succeed or none do
pub fn with_transaction<F, T>(conn: &mut Connection, f: F) -> Result<T>
where
    F: FnOnce(&Transaction) -> Result<T>,
{
    let tx = conn.transaction()?;
    let result = f(&tx)?;
    tx.commit()?;
    Ok(result)
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

    #[test]
    fn test_transaction_wrapper() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test_transaction.sqlite");

        let mut conn = init_database(&db_path)?;

        // Test successful transaction
        let result = with_transaction(&mut conn, |tx| {
            tx.execute("INSERT INTO datasets (id, type, name, crs_def, created_at) VALUES (?, ?, ?, ?, ?)",
                       ["test1", "surface", "Test Surface", "EPSG:32648", "2026-04-01"])?;
            Ok(42i32)
        })?;

        assert_eq!(result, 42);

        // Verify the insert succeeded
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM datasets WHERE id = 'test1'",
            [],
            |row| row.get(0),
        )?;
        assert_eq!(count, 1);

        // Test transaction rollback on error
        let test_result: Result<()> = with_transaction(&mut conn, |tx| {
            tx.execute("INSERT INTO datasets (id, type, name, crs_def, created_at) VALUES (?, ?, ?, ?, ?)",
                       ["test2", "surface", "Test Surface 2", "EPSG:32648", "2026-04-01"])?;
            // Simulate an error
            anyhow::bail!("Simulated error")
        });

        assert!(test_result.is_err());

        // Verify test2 was NOT inserted (rollback)
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM datasets WHERE id = 'test2'",
            [],
            |row| row.get(0),
        )?;
        assert_eq!(count, 0);

        Ok(())
    }
}
