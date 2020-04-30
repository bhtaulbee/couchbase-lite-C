#![allow(unused_imports)]

extern crate couchbase_lite;
extern crate tempdir;

use couchbase_lite::*;
use tempdir::TempDir;

const DB_NAME : &str = "test_db";

const LEVEL_PREFIX : [&str;5] = ["((", "_", "", "WARNING: ", "***ERROR: "];
const LEVEL_SUFFIX : [&str;5] = ["))", "_", "", "",          " ***"];


fn logger(domain: logging::Domain, level: logging::Level, message: &str) {
    // Log to stdout, not stderr, so that `cargo test` will buffer the output.
    let i = level as usize;
    println!("CBL {:?}: {}{}{}",
             domain, LEVEL_PREFIX[i], message, LEVEL_SUFFIX[i])

}

fn init_logging() {
    logging::set_callback(Some(logger));
}

// Test wrapper function -- takes care of creating and deleting the database.
fn with_db<F>(f: F)
    where F: Fn(&mut Database)
{
    init_logging();
    
    let start_inst_count = instance_count() as isize;
    let tmp_dir = TempDir::new("cbl_rust").expect("create temp dir");
    let cfg = DatabaseConfiguration{directory: tmp_dir.path(), flags: CREATE};
    let mut db = Database::open(DB_NAME, Some(cfg)).expect("open db");
    assert!(Database::exists(DB_NAME, tmp_dir.path()));
    
    f(&mut db);
    
    drop(db);
    println!("DROPPED DB");
    if instance_count() as isize > start_inst_count {
        dump_instances();
        panic!("Native object leak: {} objects, was {}", 
            instance_count(), start_inst_count);
    }
    println!("BYE");
}


#[test]
fn db_properties() {
    with_db(|db| {
        assert_eq!(db.name(), DB_NAME);
        assert_eq!(db.count(), 0);
    });
}

#[test]
fn create_document() {
    with_db(|_db| {
        let doc = Document::new("foo");
        assert_eq!(doc.id(), "foo");
        assert_eq!(doc.sequence(), 0);
        assert!(doc.properties());
        assert_eq!(doc.properties().count(), 0);
    });
}

#[test]
fn save_document() {
    with_db(|db| {
        {
            logging::set_level(logging::Level::Info, logging::Domain::All);
            let mut doc = Document::new("foo");
            let mut props = doc.mutable_properties();
            props.at("i").put_i64(1234);
            props.at("s").put_string("Hello World!");

            db.save_document(&mut doc, ConcurrencyControl::FailOnConflict).expect("save");
        }
        {
            let doc = db.get_document("foo").expect("reload document");
            let props = doc.properties();
            verbose!("Blah blah blah");
            info!("Interesting: {} = {}", 2+2, 4);
            warn!("Some warning");
            error!("Oh no, props = {}", props);
            assert_eq!(props.as_value().to_json(), r#"{"i":1234,"s":"Hello World!"}"#);
        }
    });
}



/*
// This test doesn't and shouldn't compile -- it tests that the borrow-checker will correctly 
// prevent Fleece data from being used after its document has been freed. 
#[test]
fn document_borrow_check() {
    let mut db = Database::open(DB_NAME, None).expect("open db");
    let v : Value;
    {
        let doc = db.get_document("foo").expect("get doc");
        v = doc.properties().get("a");
    }
    println!("v = {:?}", v);
}
*/