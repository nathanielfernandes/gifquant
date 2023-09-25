use crate::gif::GlobalPalette;

pub fn parse_gpl(src: &str) -> Vec<u8> {
    let mut palette = Vec::new();

    'outer: for line in src.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let mut parts = line.split_whitespace();

        let mut color = [0u8, 0u8, 0u8, 255u8];

        for (i, part) in parts.by_ref().take(3).enumerate() {
            let Ok(part) = part.parse::<u8>() else {
                continue 'outer;
            };

            color[i] = part;
        }

        palette.extend_from_slice(&color);
    }

    palette
}

impl Into<GlobalPalette> for Vec<u8> {
    fn into(self) -> GlobalPalette {
        let len = (self.len() / 4).min(256);
        GlobalPalette::new(1, len, &self)
    }
}
