use std::fs::File;

// Make the macro available in this file!
use crate::malloc_recur_protection;
use crate::malloc_no_conflict;
// ###################

// this file is all about logging to the file, creating records, holding the buffer, ...
lazy_static!(
    pub static ref RECORDS: std::sync::Mutex<Vec<Record>> = std::sync::Mutex::new(Vec::new());
);


#[derive(Debug)]
//#[repr(packed)] // packed because we don't need much reading on it but a lot of items of it
pub struct Record {
    pid: u32,
    timestamp: u128,
    kind: RecordKind,
    pointer: String,
    size: u64
}

#[derive(Debug)]
enum RecordKind {
    MALLOC, FREE
}

impl Record {
    pub fn new_free(pointer: String) -> Record {
        Record::new(RecordKind::FREE, 0, pointer)
    }

    pub fn new_malloc(pointer: String, size: u64) -> Record {
        Record::new(RecordKind::MALLOC, size, pointer)
    }

    fn new(kind: RecordKind, size: u64, pointer: String) -> Record {
        let start = std::time::SystemTime::now();
        let since_the_epoch = start.duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards");

        let pid = std::process::id();
        let timestamp = since_the_epoch.as_micros();

        Record {
            kind,
            pid,
            timestamp,
            pointer,
            size
        }
    }
}

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
                        std::process::exit(1);
                    });
                }
            }
        };

        LogConfig {
            log_file: file
        }
    }
}
