use crate::util::cli::MDX_FILES;
use log::info;
use rusqlite::{named_params, Connection};

pub fn query(word: String) -> String {
    let w = word;
    for file in MDX_FILES.lock().unwrap().iter() {
        let db_file = file.with_extension("db");
        let conn = Connection::open(&db_file).unwrap();
        let mut stmt = conn
            .prepare("select * from MDX_INDEX WHERE text= :word limit 1;")
            .unwrap();
        info!("query params={}", &w);

        let mut rows = stmt.query(named_params! { ":word": w }).unwrap();
        let row = rows.next().unwrap();
        if let Some(row) = row {
            let def = row.get::<usize, String>(1).unwrap();
            return def;
        }
    }
    "not found".to_string()
}
