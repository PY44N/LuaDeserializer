pub mod deserializer;
pub mod enums;
pub mod serializer;
pub mod structs;
pub mod util;

#[cfg(test)]
mod tests {
    use std::{
        fs::{self, File},
        io::{BufReader, Read},
        panic,
        process::Command,
    };

    use deserializer::Deserializer;
    use serializer::Serializer;

    use super::*;

    fn setup(test: &str) {
        std::fs::create_dir(test).expect("Failed to create test folder");

        std::fs::write(test.to_owned() + "/Test.lua", "print('Hello world')")
            .expect("Failed to create test file");
    }

    fn cleanup(test: &str) {
        fs::remove_dir_all(test).expect("Failed to remove test directory");
    }

    fn run_test<T>(test_name: &str, test: T) -> ()
    where
        T: FnOnce() -> () + panic::UnwindSafe,
    {
        setup(test_name);
        let result = panic::catch_unwind(|| test());
        cleanup(test_name);
        assert!(result.is_ok())
    }

    fn compile(test: &str) {
        Command::new("luac")
            .arg("Test.lua")
            .current_dir(test)
            .output()
            .expect("Failed to compile lua file");
    }

    #[test]
    fn deserialize() {
        run_test("dstest", || {
            compile("dstest");

            let mut reader = BufReader::new(File::open("dstest/luac.out").unwrap());
            let mut buffer = Vec::new();

            reader
                .read_to_end(&mut buffer)
                .expect("Failed to read luac.out");

            let mut deserializer = Deserializer::new(buffer);
            deserializer.deserialize();
        });
    }

    #[test]
    fn reserialize() {
        run_test("rstest", || {
            compile("rstest");

            let mut reader = BufReader::new(File::open("rstest/luac.out").unwrap());
            let mut buffer = Vec::new();

            reader
                .read_to_end(&mut buffer)
                .expect("Failed to read luac.out");

            let mut deserializer = Deserializer::new(buffer);
            let main_chunk = deserializer.deserialize();

            let mut serializer = Serializer::new();
            serializer.serialize(main_chunk);
        });
    }

    #[test]
    fn serializer_equivalency() {
        run_test("seqtest", || {
            compile("seqtest");

            let mut reader = BufReader::new(File::open("seqtest/luac.out").unwrap());
            let mut buffer = Vec::new();

            reader
                .read_to_end(&mut buffer)
                .expect("Failed to read luac.out");

            let mut deserializer = Deserializer::new(buffer.clone());
            let main_chunk = deserializer.deserialize();

            let mut serializer = Serializer::new();
            let output = serializer.serialize(main_chunk);

            for (i, v) in buffer.iter().enumerate() {
                assert_eq!(v, output.get(i).expect(&format!("Failed to get output byte at index {}, outputs are not the same size. Read: {}, serialized: {}", i, buffer.len(), output.len())));
            }
        });
    }
}
