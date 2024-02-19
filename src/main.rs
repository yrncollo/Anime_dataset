use animes::date_conversion::converted_date;
use csv::Reader;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Rawdata{
    //uid,title,synopsis,genre,aired,episodes,members,popularity,ranked,score,img_url,link

    uid: i32,
    title: String,
    synopsis: String,
    genre: String,
    aired: String,
    episodes: String,
    members: String,
    popularity: String,
    ranked: String,
    score: String,
    img_url: String,
    link: String,
}
fn main() {
    let mut csv_file = Reader::from_path("./data/animes.csv").expect("Failed to read the file");
    let rows = csv_file
        .deserialize()
        .map(|record| record.unwrap())
        .collect::<Vec<Rawdata>>();

   // rows.iter().take(12).for_each(|row| println!("{:?}", row.aired));
    // taking dates from the csv file
    rows.iter().for_each(
        |row|{
            println!("{:?}", row.aired);
            println!("{:?}", converted_date(&row.aired).unwrap());

        }
        );


}
