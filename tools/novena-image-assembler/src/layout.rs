use crate::{manifest::Manifest, require, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    pub offset: u64,
    pub size: u64,
}
impl Range {
    pub fn end(self) -> Result<u64> {
        self.offset
            .checked_add(self.size)
            .ok_or_else(|| "range end overflow".into())
    }
    pub fn overlaps(self, other: Self) -> Result<bool> {
        let (end, other_end) = (self.end()?, other.end()?);
        Ok(self.size != 0 && other.size != 0 && self.offset < other_end && other.offset < end)
    }
}
#[derive(Debug, Clone)]
pub struct Plan {
    pub disk_id: u32,
    pub fat_type: u8,
    pub spl: Range,
    pub partitions: [Range; 3],
    pub total_size: u64,
}
pub fn sector_bytes(sectors: u64) -> Result<u64> {
    sectors
        .checked_mul(512)
        .ok_or_else(|| "sector multiplication overflow".into())
}
pub fn plan(m: &Manifest) -> Result<Plan> {
    m.validate()?;
    let l = &m.layout;
    require(
        l.disk_id.len() == 10
            && l.disk_id.starts_with("0x")
            && l.disk_id.as_bytes()[2..]
                .iter()
                .all(|b| b.is_ascii_hexdigit()),
        "diskId must be 0x plus eight hex digits",
    )?;
    let disk_id = u32::from_str_radix(&l.disk_id[2..], 16)?;
    require(disk_id != 0, "zero diskId forbidden")?;
    let fat_type = match l.firmware_type.as_str() {
        "06" => 0x06,
        "0b" => 0x0b,
        "0c" => 0x0c,
        _ => return Err("firmwareType must be 06, 0b or 0c".into()),
    };
    let mut partitions = [Range { offset: 0, size: 0 }; 3];
    for (i, (start, a)) in [
        (l.firmware_start, &m.inputs.firmware),
        (l.swap_start, &m.inputs.swap),
        (l.root_start, &m.inputs.rootfs),
    ]
    .into_iter()
    .enumerate()
    {
        require(
            start > 0 && start <= u32::MAX as u64,
            "partition LBA out of range",
        )?;
        require(
            a.size > 0 && a.size % 512 == 0,
            "partition size must be positive and sector aligned",
        )?;
        require(
            a.size / 512 <= u32::MAX as u64,
            "partition length exceeds MBR",
        )?;
        partitions[i] = Range {
            offset: sector_bytes(start)?,
            size: a.size,
        };
    }
    let spl = Range {
        offset: 1024,
        size: m.inputs.spl.size,
    };
    require(
        spl.size > 0 && spl.end()? <= partitions[0].offset,
        "SPL overlaps FAT or is empty",
    )?;
    for pair in partitions.windows(2) {
        require(
            pair[0].end()? <= pair[1].offset,
            "partitions overlap or are out of order",
        )?;
    }
    let total_size = partitions[2].end()?;
    require(
        total_size / 512 <= u32::MAX as u64,
        "DOS total-sector limit exceeded",
    )?;
    Ok(Plan {
        disk_id,
        fat_type,
        spl,
        partitions,
        total_size,
    })
}
