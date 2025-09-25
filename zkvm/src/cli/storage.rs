//! Storage module.

use bincode::{Serializer as BincodeSer, deserialize, options, serialize_into};
use erased_serde::Serialize as ErasedSerialize;
use serde::{Deserialize, Serialize};
use std::{fs, io, path::Path};
use tempfile::NamedTempFile;
use zkvm_interface::{Input, InputItem};

/// Item helper enum for input serialization
#[derive(Serialize, Deserialize)]
enum StoredInputItem {
    ObjectBincode(Vec<u8>),
    SerializedObject(Vec<u8>),
    Bytes(Vec<u8>),
}

/// Data storage trait
pub trait Storage<T> {
    /// Saves the data to a file
    fn save<P: AsRef<Path>>(&self, path: P) -> io::Result<()>;

    /// Loads the data from a file
    fn load<P: AsRef<Path>>(path: P) -> io::Result<T>;
}

impl Storage<Input> for Input {
    fn save<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let path = path.as_ref();
        let cfg = options();

        let disk_items: Vec<StoredInputItem> = self
            .iter()
            .map(|item| -> io::Result<_> {
                match item {
                    InputItem::Object(obj) => {
                        let mut buf = Vec::new();
                        {
                            let mut ser = BincodeSer::new(&mut buf, cfg);
                            let mut erased = <dyn erased_serde::Serializer>::erase(&mut ser);

                            obj.erased_serialize(&mut erased)
                                .map_err(io::Error::other)?;
                        }
                        Ok(StoredInputItem::ObjectBincode(buf))
                    }
                    InputItem::SerializedObject(b) => {
                        Ok(StoredInputItem::SerializedObject(b.clone()))
                    }
                    InputItem::Bytes(b) => Ok(StoredInputItem::Bytes(b.clone())),
                }
            })
            .collect::<io::Result<Vec<StoredInputItem>>>()?;

        let dir = path.parent().unwrap_or_else(|| Path::new("."));
        let mut tmp = NamedTempFile::new_in(dir)?;

        serialize_into(&mut tmp, &disk_items).map_err(io::Error::other)?;

        tmp.as_file_mut().sync_all()?;
        tmp.persist(path).map(|_| ()).map_err(|e| e.error)
    }

    fn load<P: AsRef<Path>>(path: P) -> io::Result<Input> {
        let bytes = fs::read(path)?;
        let disk_items: Vec<StoredInputItem> =
            deserialize(&bytes).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let items: Vec<InputItem> = disk_items
            .into_iter()
            .map(|d| match d {
                StoredInputItem::SerializedObject(b) => InputItem::SerializedObject(b),
                StoredInputItem::ObjectBincode(b) | StoredInputItem::Bytes(b) => {
                    InputItem::Bytes(b)
                }
            })
            .collect();
        Ok(Input::from(items))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bincode::Options;
    use std::{fs, io};
    use tempfile::tempdir;

    #[derive(Serialize)]
    struct TestObject {
        a: u32,
        b: String,
    }

    #[test]
    fn input_storage_roundtrip_all_in_one() {
        // Test mixed roundtrip
        let mut input = Input::new();
        let test_string = "hello world".to_string();
        let test_bytes = vec![1u8, 2, 3, 4, 5];
        let test_obj = TestObject {
            a: 42,
            b: "life".into(),
        };
        input.write(test_string.clone());
        input.write_bytes(test_bytes.clone());
        input.write(test_obj);

        let cfg = bincode::options();
        let expected_string_bincode = cfg.serialize(&test_string).unwrap();
        let expected_obj_bincode = cfg
            .serialize(&TestObject {
                a: 42,
                b: "life".into(),
            })
            .unwrap();

        let dir = tempdir().unwrap();
        let path1 = dir.path().join("first_save.bin");
        let path2 = dir.path().join("second_save.bin");
        let path_empty = dir.path().join("empty.bin");
        let path_bad = dir.path().join("bad.bin");

        input.save(&path1).unwrap();
        let loaded1 = Input::load(&path1).unwrap();

        let items1: Vec<_> = loaded1.iter().collect();
        assert_eq!(items1.len(), 3);

        let &InputItem::Bytes(ref bytes_0) = items1[0] else {
            panic!()
        };
        assert_eq!(bytes_0, &expected_string_bincode);

        let &InputItem::Bytes(ref bytes_1) = items1[1] else {
            panic!()
        };
        assert_eq!(bytes_1, &test_bytes);

        let &InputItem::Bytes(ref bytes_2) = items1[2] else {
            panic!()
        };
        assert_eq!(bytes_2, &expected_obj_bincode);

        // Test resave
        loaded1.save(&path2).unwrap();
        let loaded2 = Input::load(&path2).unwrap();
        let items2: Vec<_> = loaded2.iter().collect();
        assert_eq!(items2.len(), items1.len());
        for (i, (a, b)) in items1.iter().zip(items2.iter()).enumerate() {
            match (*a, *b) {
                (InputItem::Bytes(b1), InputItem::Bytes(b2)) => assert_eq!(b1, b2),
                _ => panic!("non-bytes at index {i}"),
            }
        }

        // Test empty input
        let empty = Input::new();
        empty.save(&path_empty).unwrap();
        let loaded_empty = Input::load(&path_empty).unwrap();
        assert_eq!(loaded_empty.iter().count(), 0);

        // Test bad data
        fs::write(&path_bad, b"definitely not bincode").unwrap();
        let err = Input::load(&path_bad).err().unwrap();
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
    }
}
