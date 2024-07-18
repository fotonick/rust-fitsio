use fitsio::tables::{ColumnDataType, ColumnDescription};
use fitsio::FitsFile;
use std::path::Path;
use tempfile::Builder;

#[test]
fn test_reading_bit_data_type_as_bool() {
    let source_file = Path::new("tests/fixtures/1065880128_01.mwaf");
    let mut f = FitsFile::open(source_file).unwrap();

    let table_hdu = f.hdu(1).unwrap();
    let flags: Vec<bool> = table_hdu.read_col(&mut f, "FLAGS").unwrap();
    assert_eq!(flags.len(), 1_849_344 * 32);
}

#[test]
fn test_reading_bit_data_type_as_u8() {
    let source_file = Path::new("tests/fixtures/1065880128_01.mwaf");
    let mut f = FitsFile::open(source_file).unwrap();

    let table_hdu = f.hdu(1).unwrap();
    let bitmasks: Vec<u8> = table_hdu.read_col(&mut f, "FLAGS").unwrap();
    assert_eq!(bitmasks.len(), 1_849_344 * 4);
}

#[test]
fn test_reading_bit_data_type_as_u32() {
    let source_file = Path::new("tests/fixtures/1065880128_01.mwaf");
    let mut f = FitsFile::open(source_file).unwrap();

    let table_hdu = f.hdu(1).unwrap();
    let bitmasks: Vec<u32> = table_hdu.read_col(&mut f, "FLAGS").unwrap();
    assert_eq!(bitmasks.len(), 1_849_344);
}

#[test]
fn test_writing_bit_data_type_as_bool() {
    /* Create a temporary directory to work from */
    let tmp_dir = Builder::new().prefix("fitsio-").tempdir().unwrap();
    let file_path = tmp_dir.path().join("example.fits");

    {
        let mut fitsfile = FitsFile::create(&file_path).open().unwrap();

        let data: Vec<bool> = vec![true, false].repeat(32);
        let col = ColumnDescription::new("BITMASK")
            .with_type(ColumnDataType::Bit)
            .that_repeats(8)
            .create()
            .unwrap();
        let columns = &[col];
        let table_hdu = fitsfile.create_table("DATA", columns).unwrap();
        table_hdu
            .write_col(&mut fitsfile, "BITMASK", &data)
            .unwrap();
    }

    let mut f = FitsFile::open(file_path).unwrap();

    let table_hdu = f.hdu("DATA").unwrap();
    let flags: Vec<u8> = table_hdu.read_col(&mut f, "BITMASK").unwrap();
    assert_eq!(flags.len(), 64);
}

#[test]
fn test_writing_bit_data_type_as_u8() {
    /* Create a temporary directory to work from */
    let tmp_dir = Builder::new().prefix("fitsio-").tempdir().unwrap();
    let file_path = tmp_dir.path().join("example.fits");

    {
        let mut fitsfile = FitsFile::create(&file_path).open().unwrap();

        let data: Vec<u8> = vec![0xa5].repeat(8);
        let col = ColumnDescription::new("BITMASK")
            .with_type(ColumnDataType::Bit)
            .that_repeats(8)
            .create()
            .unwrap();
        let columns = &[col];
        let table_hdu = fitsfile.create_table("DATA", columns).unwrap();
        table_hdu
            .write_col(&mut fitsfile, "BITMASK", &data)
            .unwrap();
    }

    let mut f = FitsFile::open(file_path).unwrap();

    let table_hdu = f.hdu("DATA").unwrap();
    let flags: Vec<bool> = table_hdu.read_col(&mut f, "BITMASK").unwrap();
    assert_eq!(flags.len(), 64);
}
