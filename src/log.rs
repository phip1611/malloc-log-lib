use std::fs::File;

// Make the macro available in this file!
use crate::endless_recur_protection;
use crate::malloc_no_conflict;
// ###################

// this file is all about logging to the file, creating records, holding the buffer, ...

#[derive(Debug)]
pub struct LogConfig {
    log_file: Option<File> // I couldn't manage it to make this non-optional...
}

impl LogConfig {
    pub fn new() -> LogConfig {
        malloc_no_conflict!({
            eprintln!("foobar");
        });

        /* for some reason either the ok nor the err branch are executed
        // i don't get it.. todo investigate
        let mut file: Option<File> = None;
        match File::open("malloc-log-lib.txt") {
            Ok(f) => {
                malloc_no_conflict!(
                    println!(
                        "file: {:#?}", f
                    )
                );
                file = Some(f)
            },
            _ => {
                #[allow(unreachable_code)] // because panic breaks the program
                {
                    malloc_no_conflict!({
                        eprintln!("Can't open out-file 'malloc-log-lib.txt'! Exiting program.");
                        panic!("hello");
                    });
                }
            }
        };*/

        LogConfig {
            log_file: file
        }
    }
}
