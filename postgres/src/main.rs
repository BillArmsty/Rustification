use postgres::{ Client, NoTls, Error };
use std::collections::HashMap;

struct Author {
    _id: i32,
    name: String,
    country: String,
}

fn main() -> Result<(), Error> {
    let mut client = Client::connect("postgresql://postgres:postgres@localhost/library", NoTls)?;

    // client.batch_execute("
    //     CREATE TABLE IF NOT EXISTS author (
    //         id              SERIAL PRIMARY KEY,
    //         name            VARCHAR NOT NULL,
    //         country         VARCHAR NOT NULL
    //         )
    // ")?;

    // client.batch_execute("
    //     CREATE TABLE IF NOT EXISTS book  (
    //         id              SERIAL PRIMARY KEY,
    //         title           VARCHAR NOT NULL,
    //         author_id       INTEGER NOT NULL REFERENCES author
    //         )
    // ")?;

    // Ok(());
    println!("Connected to database!");

    let mut authors = HashMap::new();
    authors.insert(String::from("Chinua Achebe"), "Nigeria");
    authors.insert(String::from("Rabindranath Tagore"), "India");
    authors.insert(String::from("Anita Nair"), "India");

    //Insert data
    for (key, value) in &authors {
        let author = Author {
            _id: 0,
            name: key.to_string(),
            country: value.to_string(),
        };

        client.execute(
            "INSERT INTO author (name, country) VALUES ($1, $2)",
            &[&author.name, &author.country]
        )?;
    }
    //Query data
    for row in client.query("SELECT id, name, country FROM author", &[ &authors; 1 ]?)? {
        let author = Author {
            _id: row.get(0),
            name: row.get(1),
            country: row.get(2),
        };
        println!("Author {} is from {}", author.name, author.country);
    }
    Ok(())
}
