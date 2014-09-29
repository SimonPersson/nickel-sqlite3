# nickel-sqlite3
A simple sqlite middleware for the nickel.rs web framework.

## Usage
### Cargo.toml
```
[dependencies.nickel-sqlite]
git = "https://github.com/SimonPersson/nickel-sqlite3.git"
```

### Import
```
extern crate nickel_sqlite;
use nickel_sqlite::Sqlite3Middleware;
```

### Usage
```
server.utilize(Sqlite3Middleware::new("database.db".to_string()));

fn handle(req: &Request, res: &mut Response)
{
	let ref mut db = req.db_conn().lock();
	let mut stmt = match db.prepare("select * from numbers;")
	{
		Ok(s) => s,
		Err(e) => fail!("failed..: {}", e),
	};
	let mut res = stmt.execute();
	match res.step()
	{
		Some(Ok(ref mut row))
		{
			println!("{}", row.column_int(0));
		}
		_ =>
		{
			println!("no data in db :(");
		}
	}
}
```
