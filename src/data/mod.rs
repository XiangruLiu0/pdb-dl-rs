#[derive(Clone, Copy)]
pub struct PdbGuid {
    u0: u32,
    u1: u16,
    u2: u16,
    u3: u32,
    u4: u32,
}

impl From<[u8; 16]> for PdbGuid {
    fn from(bytes: [u8; 16]) -> Self {
        let u0 = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
        let u1 = u16::from_le_bytes(bytes[4..6].try_into().unwrap());
        let u2 = u16::from_le_bytes(bytes[6..8].try_into().unwrap());
        let u3 = u32::from_be_bytes(bytes[8..12].try_into().unwrap());
        let u4 = u32::from_be_bytes(bytes[12..16].try_into().unwrap());
        PdbGuid { u0, u1, u2, u3, u4 }
    }
}

impl From<PdbGuid> for String {
    fn from(guid: PdbGuid) -> Self {
        format!(
            "{:08X}{:04X}{:04X}{:08X}{:08X}",
            guid.u0, guid.u1, guid.u2, guid.u3, guid.u4
        )
    }
}

impl std::fmt::Display for PdbGuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(*self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pdb_guid() {
        let guid = PdbGuid::from([
            185, 30, 59, 126, 63, 252, 238, 44, 51, 235, 12, 242, 102, 61, 151, 3,
        ]);
        assert_eq!(guid.to_string(), "7E3B1EB9FC3F2CEE33EB0CF2663D9703");
    }
}
