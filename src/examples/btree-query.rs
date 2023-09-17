fn main() -> Result<()> {
    let disk = DiskManager::open("test.btr")?;
    let pool = BufferPool::new(10);
    let mut bufmgr = BufferPoolManager::new(disk, pool);

    let btree = Btree::new(PageId(0));
    let mut iter = btree.search(&mut bufmgr, SearchMode::Key(b"Hyogo".to_vec()))?;
    
}