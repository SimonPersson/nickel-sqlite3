extern crate serialize;
extern crate nickel;
extern crate http;
extern crate nickel_sqlite;

use nickel_sqlite::{Sqlite3Middleware, Sqlite3RequestExtensions};
use nickel::{ Nickel, Request, Response }; 
use std::io::net::ip::Ipv4Addr;
use std::collections::HashMap;
use serialize::Encodable;

fn main() {

    let mut server = Nickel::new();

    fn root_handler (req: &Request, response: &mut Response) {

        let ref mut db = req.db_conn().lock();

        let mut stmt = match db.prepare("select * from numbers;")
        {
            Ok(s) => s,
            Err(e) => fail!("failed..: {}", e),
        };
        
        let mut res =  stmt.execute();
     
        #[deriving(Encodable, Decodable)]
        struct Num
        {
            number: i32
        }
        let mut data = HashMap::<&str, Vec<Num>>::new();

        let mut numbers = Vec::new();
        loop
        {
            match res.step()
            {
                Some(Ok(ref mut row)) =>
                {
                     numbers.push(Num{number:row.column_int(0)});
                },
                _ => break,
            };
        }

        data.insert("numbers", numbers);
        response.render("example/assets/template.tpl", &data);
    }
    let sqlite_mw = Sqlite3Middleware::new("example/assets/test.db".to_string());
    server.utilize(sqlite_mw);

    server.get("/", root_handler);

    server.listen(Ipv4Addr(127, 0, 0, 1), 6767);
}
