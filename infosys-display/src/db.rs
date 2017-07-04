extern crate config;
extern crate postgres;

use self::postgres::Connection;
use self::postgres::TlsMode;

static SQL_CREATE_SCHEDULE: &'static str = "CREATE TABLE schedule (
    timeslot        TIME NOT NULL PRIMARY KEY,
    message_id      INTEGER NOT NULL,
    FOREIGN KEY     (message_id) REFERENCES messages(id)
);";

static SQL_CREATE_MESSAGE: &'static str = "CREATE TABLE messages (
    id SERIAL PRIMARY KEY
);";

static SQL_CREATE_STRING: &'static str = "CREATE TABLE strings (
    id              INTEGER NOT NULL,
    message_id      INTEGER NOT NULL,
    mode            VARCHAR(32) NOT NULL,
    data            TEXT NOT NULL,
    PRIMARY KEY     (id, message_id),
    FOREIGN KEY     (message_id) REFERENCES messages(id)
);";

static SQL_DEFAULT_VALUES: &'static str = "
INSERT INTO messages (id) VALUES (0);

INSERT INTO schedule (timeslot, message_id) VALUES ('00:00:00', 0);

INSERT INTO strings (id, message_id, mode, data) VALUES (1, 0, 'STANDARD_HOLD', 'Welcome to CSH!');";

pub fn db_init(settings: &config::Config) ->Connection{
    let con = Connection::connect(settings.get_str("dbstring").unwrap(), TlsMode::None).unwrap();

    let mut init = true;
    init = init && check_or_create_db(&con, "messages", SQL_CREATE_MESSAGE);
    init = init && check_or_create_db(&con, "schedule", SQL_CREATE_SCHEDULE);
    init = init && check_or_create_db(&con, "strings", SQL_CREATE_STRING);

    // If we're doing a clean init add some default values!
    if init {
        con.batch_execute(SQL_DEFAULT_VALUES).unwrap();
    }
    return con;
}

fn check_or_create_db(con: &Connection, name: &str, sql: &str) -> bool {

    let exists: bool = con.execute("SELECT 1::integer FROM pg_tables
            WHERE schemaname = 'public' AND tablename = $1::text;",
            &[&name]).unwrap() != 0;

    // Create the table!
    if !exists {
        con.execute(&sql, &[]).unwrap();
    }

    // Return true if we created the table!
    return !exists;
}

// TODO move this into a single request utilizing the schedule and foreign keys
pub fn retrieve_strings_for_message_id(con: &Connection, id: i32) -> Vec<(String, String)> {

    let mut str_list: Vec<(String, String)> = Vec::new();

    for string in &con.query(
        "SELECT id, message_id, data, mode FROM strings where message_id = $1",
        &[&id]).unwrap()
    {
        let data: String = string.get(2);
        let mode: String = string.get(3);
        str_list.push((mode, data));
    }
    return str_list;
}
