use std::fs;

type Result<T> = std::result::Result<T, MapDepletedError>;

#[derive(Debug, Clone)]
struct MapDepletedError;

#[derive(Debug)]
struct DataBlock {
    id: u32,
    start_index: u32,
    end_index: u32,
}

impl DataBlock {
    fn contains_index(&self, index: u32) -> bool {
        index >= self.start_index && index <= self.end_index
    }
}

#[derive(Debug)]
struct DiskMap {
    data_blocks: Vec<DataBlock>,
    first_index: u32,
    last_index: u32,
}

impl DiskMap {
    pub fn next(&mut self) -> Result<u32> {
        if self.first_index > self.last_index {
            return Err(MapDepletedError);
        }

        let target_index = self.first_index;
        self.first_index += 1;

        for data_block in self.data_blocks.iter() {
            if data_block.contains_index(target_index) {
                return Ok(data_block.id);
            }
        }

        if self.first_index >= self.last_index {
            return Err(MapDepletedError);
        }

        return Ok(self.pop());
    }

    fn pop(&mut self) -> u32 {
        loop {
            if self.last_index < self.first_index {
                panic!("First index surpassed last index");
            }

            let target_index = self.last_index;

            self.last_index -= 1;
    
            for data_block in self.data_blocks.iter() {
                if data_block.contains_index(target_index) {
                    return data_block.id;
                }
            }
        }
    }

    fn from_string(string: String) -> DiskMap {
        let mut data_blocks: Vec<DataBlock> = Vec::new();
        let mut block_id = 0;
        let mut block_pos = 0;
        let mut skip_block = false;

        for line in string.lines() {
            for char in line.chars() {
                let block_len = char.to_digit(10).expect("File should only contain numeric chars");
                
                if skip_block {
                    skip_block = false;
                    block_pos += block_len;
                    continue;
                }

                data_blocks.push(DataBlock {
                    id: block_id,
                    start_index: block_pos,
                    end_index: block_pos + block_len - 1,
                });

                block_id += 1;
                block_pos += block_len;
                skip_block = true;
            }
        }

        DiskMap {
            data_blocks,
            first_index: 0,
            last_index: block_pos - 1,
        }
    }
}

pub fn run() {
    let mut disk_map = DiskMap::from_string(fs::read_to_string("inputs/day9.txt").unwrap());

    // println!("Disk Map built: {disk_map:#?}");

    let mut checksum: u64 = 0;
    let mut block_pos = 0;

    loop {
        if let Ok(block_id) = disk_map.next() {
            checksum += block_id as u64 * block_pos as u64;
            block_pos += 1;
        } else {
            break;
        }
    }

    println!("Checksum: {checksum}");
}
