use std::fs::File;

// Make the macro available in this file!
use crate::malloc_recur_protection;
use crate::malloc_no_conflict;
// ###################

// this file is all about logging to the file, creating records, holding the buffer, ...

#[derive(Debug)]
pub struct LogConfig {
    log_file: Option<File> // I couldn't manage it to make this non-optional...
}

impl LogConfig {
    pub fn new() -> LogConfig {

        let mut file: Option<File> = None;
        // creates it or opens and truncates it
        match File::create("malloc-log-lib.txt") {
            Ok(f) => {
                malloc_no_conflict!(
                    println!(
                        "file: {:#?}", f
                    )
                );
                file = Some(f)
            },
            Err(e) => {
                #[allow(unreachable_code)] // because panic breaks the program
                {
                    malloc_no_conflict!({
                        eprintln!("Can't open out-file 'malloc-log-lib.txt'! Exiting program. Error={:#?}", e);
                        panic!("hello");
                    });
                }
            }
        };

        LogConfig {
            log_file: file
        }
    }
}
