use std::fs::File;

// Make the macro available in this file!
use crate::malloc_recur_protection;
use crate::malloc_no_conflict;
use std::io::Write;
// ###################

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

pub fn write_head() {
    let head: String = String::from("timestamp;kind;size;pid;pointer;\n");
    /*let mut cfg_guard: std::sync::MutexGuard<Option<LogConfig>> = crate::LOG_CONFIG.lock().unwrap();
    let cfg: &mut LogConfig = cfg_guard.as_mut().unwrap();
    let mut log_file = cfg.borrow_mut().log_file;
    log_file.write_all(head.as_bytes());*/

    // didn't make it work it to use the global stored file :/
    let mut file = File::create("malloc-log-lib.txt").unwrap();
    let _ = file.write_all(head.as_bytes());
}

pub fn write_record(record: Record) {
    let mut row: String = String::from("");

    row.push_str(format!("{}", record.timestamp).as_str());
    row.push(';');

    row.push_str(format!("{:?}", record.kind).as_str());
    row.push(';');

    row.push_str(format!("{}", record.size).as_str());
    row.push(';');

    row.push_str(format!("{}", record.pid).as_str());
    row.push(';');

    row.push_str(record.pointer.as_str());
    row.push(';');
    row.push('\n');

    // didn't make it work it to use the global stored file :/
    let mut file = std::fs::OpenOptions::new().append(true)
        .open("malloc-log-lib.txt").unwrap();

    let _ = file.write_all(row.as_bytes());
}
