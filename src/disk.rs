use std::{
    fs::{File, OpenOptions},
    io::{self, Read, Result, Seek, SeekFrom, Write},
    path::Path,
};

use zerocopy::{AsBytes, FromBytes};

pub const PAGE_SIZE: usize = 4096;

pub struct DiskManager {
    heap_file: File,
    next_page_id: u64,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, FromBytes, AsBytes)]
#[repr(C)]
pub struct PageId(pub u64);

impl PageId {
    pub const INVALID_PAGE_ID: PageId = PageId(u64::MAX);

    pub fn valid(self) -> Option<PageId> {
        if self == Self::INVALID_PAGE_ID {
            None
        } else {
            Some(self)
        }
    }

    pub fn to_u64(&self) -> u64 {
        self.0 as u64
    }
}

impl Default for PageId {
    fn default() -> Self {
        Self::INVALID_PAGE_ID
    }
}

impl From<Option<PageId>> for PageId {
    fn from(page_id: Option<PageId>) -> Self {
        page_id.unwrap_or_default()
    }
}

impl From<&[u8]> for PageId {
    fn from(bytes: &[u8]) -> Self {
        let arr = bytes.try_into().unwrap();
        PageId(u64::from_ne_bytes(arr))
    }
}

impl DiskManager {
    pub fn new(heap_file: File) -> Result<Self> {
        // ファイルサイズを取得
        let heap_file_size = heap_file.metadata()?.len();
        let next_page_id = heap_file_size / PAGE_SIZE as u64;
        Ok(Self {
            heap_file,
            next_page_id,
        })
    }

    pub fn open(heap_file_path: impl AsRef<Path>) -> Result<Self> {
        let heap_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(heap_file_path)?;
        Self::new(heap_file)
    }

    pub fn allocate_page(&mut self) -> PageId {
        let page_id = self.next_page_id;
        self.next_page_id += 1;
        PageId(page_id)
    }

    pub fn write_page_data(&mut self, page_id: PageId, data: &[u8]) -> Result<()> {
        // オフセットを計算
        let offset = PAGE_SIZE as u64 * page_id.to_u64();
        // ページ先頭へシーク
        self.heap_file.seek(SeekFrom::Start(offset))?;
        // データを書き込む
        self.heap_file.write_all(data)
    }

    pub fn read_page_data(&mut self, page_id: PageId, data: &mut [u8]) -> Result<()> {
        // オフセットを計算
        let offset = PAGE_SIZE as u64 * page_id.to_u64();
        // ページ先頭へシーク
        self.heap_file.seek(SeekFrom::Start(offset))?;
        // データを読み出す
        self.heap_file.read_exact(data)
    }

    pub fn sync(&mut self) -> io::Result<()> {
        self.heap_file.flush()?;
        self.heap_file.sync_all()
    }
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use crate::disk::DiskManager;

    #[test]
    fn open_ファイルが開けること() {
        let test_file_path = "src/tests/sample.txt";
        let dm_reslt = DiskManager::open(test_file_path);

        assert_eq!(dm_reslt.is_ok(), true);
        let mut dm = dm_reslt.unwrap();

        let mut buf = String::new();
        let _ = dm.heap_file.read_to_string(&mut buf);

        assert_eq!(buf, "123");
    }
}
