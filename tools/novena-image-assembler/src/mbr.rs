use crate::{layout::Plan, require, Result};

/// Deterministic MBR: no BIOS code, no active flag; saturated CHS fields.
pub fn encode(plan: &Plan) -> Result<[u8; 512]> {
    let mut bytes = [0u8; 512];
    bytes[440..444].copy_from_slice(&plan.disk_id.to_le_bytes());
    for (i, (range, kind)) in plan
        .partitions
        .iter()
        .zip([plan.fat_type, 0x82, 0x83])
        .enumerate()
    {
        require(
            range.offset % 512 == 0 && range.size % 512 == 0,
            "MBR ranges must be sector aligned",
        )?;
        let entry = &mut bytes[446 + i * 16..446 + (i + 1) * 16];
        entry[1..4].copy_from_slice(&[0xfe, 0xff, 0xff]);
        entry[4] = kind;
        entry[5..8].copy_from_slice(&[0xfe, 0xff, 0xff]);
        entry[8..12].copy_from_slice(&u32::try_from(range.offset / 512)?.to_le_bytes());
        entry[12..16].copy_from_slice(&u32::try_from(range.size / 512)?.to_le_bytes());
    }
    bytes[510..512].copy_from_slice(&[0x55, 0xaa]);
    Ok(bytes)
}
