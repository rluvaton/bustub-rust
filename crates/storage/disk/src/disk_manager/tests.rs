#[cfg(test)]
mod tests {
    use pages::PAGE_SIZE;
    use std::path::PathBuf;
    use tempdir::TempDir;
    use crate::*;

    fn setup() -> TempDir {
        TempDir::new("disk_manager_tests").expect("Should create tmp directory")
    }

    #[test]
    fn read_write_page() {
        let mut buf = [0u8; PAGE_SIZE];
        let mut data = [0u8; PAGE_SIZE];
        let tmp_dir = setup();

        let db_file = tmp_dir.path().join("test.db");
        let mut dm = DefaultDiskManager::new(db_file).expect("Should create disk manager");
        let val = "A test string.";
        data[0..val.len()].copy_from_slice(val.as_bytes());

        dm.read_page(0, &mut buf);  // tolerate empty read

        dm.write_page(0, &data);
        dm.read_page(0, &mut buf);

        // EXPECT_EQ(std::memcmp(buf, data, sizeof(buf)), 0);
        assert_eq!(buf, data);

        // std::memset(buf, 0, sizeof(buf));
        buf.fill(0);

        dm.write_page(5, &data);
        dm.read_page(5, &mut buf);

        // EXPECT_EQ(std::memcmp(buf, data, sizeof(buf)), 0);
        assert_eq!(buf, data);

        dm.shut_down();
    }


    #[test]
    fn read_write_log() {
        const BUF_SIZE: usize = 16;
        let mut buf = [0u8; BUF_SIZE];
        let mut data = [0u8; BUF_SIZE];
        let tmp_dir = setup();

        let db_file = tmp_dir.path().join("test.db");
        let mut dm = DefaultDiskManager::new(db_file).expect("Should create disk manager");

        let val = "A test string.";
        data[0..val.len()].copy_from_slice(val.as_bytes());

        dm.read_log(&mut buf, BUF_SIZE as i32, 0);  // tolerate empty read

        dm.write_log(&data, BUF_SIZE as i32);
        dm.read_log(&mut buf, BUF_SIZE as i32, 0);

        // EXPECT_EQ(std::memcmp(buf, data, sizeof(buf)), 0);
        assert_eq!(buf, data);

        dm.shut_down();
    }

    #[test]
    fn bad_file() {
        let p = PathBuf::from("dev/null\\/foo/bar/baz/test.db");
        let creation = DefaultDiskManager::new(p);

        assert_eq!(creation.is_err(), true);
    }


    #[test]
    fn read_write_page_unlimited_memory() {
        let mut buf = [0u8; PAGE_SIZE];
        let mut data = [0u8; PAGE_SIZE];

        let mut dm = DiskManagerUnlimitedMemory::new();
        let val = "A test string.";
        data[0..val.len()].copy_from_slice(val.as_bytes());

        dm.write_page(0, &data);
        dm.read_page(0, &mut buf);

        // EXPECT_EQ(std::memcmp(buf, data, sizeof(buf)), 0);
        assert_eq!(buf, data);

        // std::memset(buf, 0, sizeof(buf));
        buf.fill(0);

        dm.write_page(5, &data);
        dm.read_page(5, &mut buf);

        // EXPECT_EQ(std::memcmp(buf, data, sizeof(buf)), 0);
        assert_eq!(buf, data);

        dm.shut_down();
    }
}
