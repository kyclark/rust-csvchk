use csv::ReaderBuilder;
use std::{error::Error, fs::File};

pub type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    pub files: Vec<String>,
    pub delimiter: u8,
    pub has_headers: bool,
}

// --------------------------------------------------
pub fn process(
    filename: &String,
    delimiter: &u8,
    has_headers: bool,
) -> MyResult<()> {
    match File::open(filename) {
        Ok(file) => {
            println!("Processing {}", filename);
            let mut reader = ReaderBuilder::new()
                .delimiter(*delimiter)
                .has_headers(has_headers)
                .from_reader(file);

            let headers: Option<Vec<String>> = match has_headers {
                true => {
                    let hdrs = reader.headers().unwrap();
                    let longest = hdrs.iter().map(|h| h.len()).max().unwrap();
                    Some(
                        hdrs.into_iter()
                            .map(|h| format!("{:width$}", h, width = longest))
                            .collect(),
                    )
                }
                false => None,
            };

            for (i, record) in reader.records().enumerate() {
                let record = record?;
                let fields: Vec<&str> = record.iter().collect();
                let num_flds = fields.len();
                let hdrs = headers.clone().unwrap_or(
                    (1..=num_flds)
                        .into_iter()
                        .map(|n| format!("Field{:02}", n))
                        .collect::<Vec<String>>(),
                );

                println!(
                    "// ****** Record {} ****** //\n{}\n",
                    i + 1,
                    hdrs.iter()
                        .zip(fields.iter())
                        .map(|(hdr, val)| format!("{:>} : {}", hdr, val))
                        .collect::<Vec<String>>()
                        .join("\n")
                );
            }
        }
        Err(e) => eprintln!("{}: {}", filename, e),
    }

    Ok(())
}

// --------------------------------------------------
//fn format_record(
//    record: StringRecord,
//    headers: &Option<Vec<String>>,
//) -> String {
//    let fields: Vec<&str> = record.iter().collect();
//    let num_flds = fields.len();
//    let hdrs = headers.as_ref().unwrap_or(
//        (1..=num_flds)
//            .into_iter()
//            .map(|n| format!("Field{}", n))
//            .collect().cloned(),
//    );

//    hdrs.iter()
//        .zip(fields.iter())
//        .map(|(hdr, val)| format!("{:>20}: {}", hdr, val))
//        .collect::<Vec<&String>>()
//        .join("\n")
//}

// --------------------------------------------------
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
