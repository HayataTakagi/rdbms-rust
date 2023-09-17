use anyhow::Result;

use relly::btree::Btree;
use relly::buffer::{BufferPool, BuggerPoolManager};
use relly::disk::DiskManager;

fn main() -> Result<()> {
    let disk = DiskManager::open("test.btr")?;
    let pool = BuggerPool::new(10);
    let mut bufmgr = BufferPoolManager::new(disk, pool);

    let btree = Btree::create(&mut bufmgr)?;

    btree.insert(&mut bufmgr, b"Kanagawa", b"Yokohama")?;
    btree.insert(&mut bufmgr, b"Osaka", b"Osaka")?;
    btree.insert(&mut bufmgr, b"Aichi", b"Nagoya")?;
    btree.insert(&mut bufmgr, b"Hokkaido", b"Sapporo")?;
    btree.insert(&mut bufmgr, b"Fukuoka", b"Fukuoka")?;
    btree.insert(&mut bufmgr, b"Hyogo", b"Kobe")?;

    bufmgr.flush()?;

    Ok(())
}