extern crate sync;
extern crate nickel;
extern crate sqlite3;

use nickel::{Request, Middleware, Response, Action, NickelError, Continue};
use sqlite3::core::DatabaseConnection;
use sqlite3::access;
use self::sync::{Mutex, Arc};

#[deriving(Clone)]
pub struct Sqlite3Middleware
{
    conn: Arc<Mutex<DatabaseConnection>>
}

impl Sqlite3Middleware
{
    pub fn new(file: String) -> Sqlite3Middleware
    {
        Sqlite3Middleware
        {
            conn: Arc::new(Mutex::new(access::open(file).unwrap()))
        }
    }
}

impl Middleware for Sqlite3Middleware
{
    fn invoke(&self, req: &mut Request, _: &mut Response) -> Result<Action, NickelError>
    {
        req.map.insert(self.conn.clone());
        Ok(Continue)
    }
}

pub trait Sqlite3RequestExtensions
{
    fn db_conn(&self) -> &Arc<Mutex<DatabaseConnection>>;
}

impl<'a> Sqlite3RequestExtensions for Request<'a>
{
    fn db_conn(&self) -> &Arc<Mutex<DatabaseConnection>>
    {
        return self.map.find::<Arc<Mutex<DatabaseConnection>>>().unwrap()
    }
}
