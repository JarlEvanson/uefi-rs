// SPDX-License-Identifier: MIT OR Apache-2.0

use anyhow::Result;
use fatfs::{Date, DateTime, FileSystem, FormatVolumeOptions, FsOptions, Time};
use mbrman::{BOOT_INACTIVE, CHS, MBR, MBRPartitionEntry};
use std::io::{Cursor, Read, Write};
use std::ops::Range;
use std::path::Path;

const SECTOR_SIZE: usize = 512;

fn get_partition_byte_range(mbr: &MBR) -> Range<usize> {
    let partition_start_byte = mbr[1].starting_lba as usize * SECTOR_SIZE;
    let partition_num_bytes = mbr[1].sectors as usize * SECTOR_SIZE;
    partition_start_byte..partition_start_byte + partition_num_bytes
}

pub fn create_mbr_test_disk(path: &Path) -> Result<()> {
    // 10 MiB.
    let size_in_bytes = 10 * 1024 * 1024;

    let partition_byte_range;
    let mut disk = vec![0; size_in_bytes];
    {
        let mut cur = std::io::Cursor::new(&mut disk);

        let mut mbr = MBR::new_from(&mut cur, SECTOR_SIZE as u32, [0xff; 4])?;
        mbr[1] = MBRPartitionEntry {
            boot: BOOT_INACTIVE,
            first_chs: CHS::empty(),
            sys: 0x06,
            last_chs: CHS::empty(),
            starting_lba: 1,
            sectors: mbr.disk_size - 1,
        };

        partition_byte_range = get_partition_byte_range(&mbr);

        mbr.write_into(&mut cur)?;
    }

    init_fat_test_partition(&mut disk, partition_byte_range)?;

    fs_err::write(path, &disk)?;

    Ok(())
}

fn init_fat_test_partition(disk: &mut [u8], partition_byte_range: Range<usize>) -> Result<()> {
    {
        let cursor = Cursor::new(&mut disk[partition_byte_range.clone()]);
        fatfs::format_volume(
            cursor,
            FormatVolumeOptions::new().volume_label(*b"MbrTestDisk"),
        )?;
    }

    let cursor = Cursor::new(&mut disk[partition_byte_range]);
    let fs = FileSystem::new(cursor, FsOptions::new().update_accessed_date(false))?;

    assert_eq!(
        fs.read_volume_label_from_root_dir().unwrap(),
        Some("MbrTestDisk".to_string())
    );

    let root_dir = fs.root_dir();

    let dir = root_dir.create_dir("test_dir")?;

    let mut file = dir.create_file("test_input.txt")?;
    file.write_all(b"test input data")?;

    // The datetime-setting functions have been deprecated, but are
    // useful here to force an exact date that can be checked in the
    // test.
    #[allow(deprecated)]
    {
        let time = Time {
            hour: 0,
            min: 0,
            sec: 0,
            millis: 0,
        };
        file.set_created(DateTime {
            date: Date {
                year: 2000,
                month: 1,
                day: 24,
            },
            time,
        });
        file.set_accessed(Date {
            year: 2001,
            month: 2,
            day: 25,
        });
        file.set_modified(DateTime {
            date: Date {
                year: 2002,
                month: 3,
                day: 26,
            },
            time,
        });
    }

    let stats = fs.stats()?;
    // Assert these specific numbers here since they are checked by the
    // test-runner too.
    assert_eq!(stats.total_clusters(), 10183);
    assert_eq!(stats.free_clusters(), 10181);

    Ok(())
}

pub fn check_mbr_test_disk(path: &Path) -> Result<()> {
    println!("Verifying test disk has been correctly modified");
    let mut disk = fs_err::read(path)?;

    let partition_byte_range;
    {
        let mut cursor = Cursor::new(&disk);
        let mbr = MBR::read_from(&mut cursor, SECTOR_SIZE as u32)?;
        partition_byte_range = get_partition_byte_range(&mbr);
    }

    let cursor = Cursor::new(&mut disk[partition_byte_range]);
    let fs = FileSystem::new(cursor, FsOptions::new().update_accessed_date(false))?;
    let root_dir = fs.root_dir();

    // Check that the new file was created.
    let mut file = root_dir.open_file("new_test_file.txt")?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    assert_eq!(bytes, b"test output data");

    // Check that the original input file was deleted.
    let dir = root_dir.open_dir("test_dir")?;
    let children: Vec<_> = dir.iter().map(|e| e.unwrap().file_name()).collect();
    assert_eq!(children, [".", ".."]);

    Ok(())
}
