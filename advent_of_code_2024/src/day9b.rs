use std::fs;
use std::fmt::Formatter;
use std::fmt::Display;

#[derive(Debug)]
enum MapError {
    MapDepleted,
    NoSpace,
}

impl Display for MapError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            MapError::MapDepleted => write!(f, "Map has been depleted"),
            MapError::NoSpace => write!(f, "No space found"),
        }
    }
}

impl std::error::Error for MapError {}

#[derive(Debug, Clone)]
struct DataBlock {
    id: u32,
    start_index: u32,
    length: u32,
}

impl DataBlock {
    fn contains_index(&self, index: u32) -> bool {
        index >= self.start_index && index < self.start_index + self.length
    }
}


#[derive(Debug, Clone)]
struct EmptyBlock {
    start_index: u32,
    length: u32,
}

#[derive(Debug, Clone)]
struct DiskMap {
    data_blocks: Vec<DataBlock>,
    empty_blocks: Vec<EmptyBlock>,
    first_index: u32,
    last_index: u32,
}

impl DiskMap {
    fn optimize(&mut self) {
        let mut data_blocks = self.data_blocks.clone();

        for data_block in data_blocks.iter_mut().rev() {
            let destination = self.make_space_for(data_block.length, data_block.start_index);

            if let Ok(destination_index) = destination {
                data_block.start_index = destination_index;
            }
        }

        data_blocks.sort_by_key(|b| b.start_index);

        self.data_blocks = data_blocks;
    }

    fn make_space_for(&mut self, length: u32, before_index: u32) -> Result<u32, MapError> {
        for (i, block) in self.empty_blocks.iter_mut().enumerate() {
            if block.length >= length && block.start_index < before_index {
                let destination_index = block.start_index;

                block.start_index += length;
                block.length -= length;

                if block.length == 0 {
                    self.empty_blocks.remove(i);
                }

                return Ok(destination_index);
            }
        }

        Err(MapError::NoSpace)
    }

    fn next(&mut self) -> Result<u32, MapError> {
        if self.first_index > self.last_index {
            return Err(MapError::MapDepleted);
        }

        let target_index = self.first_index;
        self.first_index += 1;

        if let Some(block) = self.data_blocks.first() {
            if block.contains_index(target_index) {
                let result = Ok(block.id);

                if (block.start_index + block.length) - 1 == target_index {
                    self.data_blocks.remove(0);
                }

                return result;
            }
        }

        Ok(0)
    }

    fn from_string(string: String) -> DiskMap {
        let mut data_blocks: Vec<DataBlock> = Vec::new();
        let mut empty_blocks: Vec<EmptyBlock> = Vec::new();
        let mut block_id = 0;
        let mut block_pos = 0;
        let mut empty_block = false;

        for line in string.lines() {
            for char in line.chars() {
                let block_len = char.to_digit(10).expect("File should only contain numeric chars");
                
                if empty_block {
                    empty_blocks.push(EmptyBlock {
                        start_index: block_pos,
                        length: block_len,
                    });

                    empty_block = false;
                    block_pos += block_len;

                    continue;
                }

                data_blocks.push(DataBlock {
                    id: block_id,
                    start_index: block_pos,
                    length: block_len,
                });

                block_id += 1;
                block_pos += block_len;
                empty_block = true;
            }
        }

        DiskMap {
            data_blocks,
            empty_blocks,
            first_index: 0,
            last_index: block_pos - 1,
        }
    }
}

pub fn run() {
    let mut disk_map = DiskMap::from_string(fs::read_to_string("inputs/day9.txt").unwrap());
    
    // println!("Disk Map built: {disk_map:#?}");

    disk_map.optimize();

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
