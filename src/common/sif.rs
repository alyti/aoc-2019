//! Implements the Space Image Format from day 8.
use std::str;

pub type Row = Vec<u8>;
pub type Layer = Vec<Row>;

#[derive(Debug, Clone)]
pub struct SpaceImage {
    pub resolution: (u32, u32),
    pub layers: Vec<Layer>
}

impl SpaceImage {
    pub fn load((x, y): (u32, u32), raw: &str) -> Self {
        // TODO: This is hella hacky but I don't care anymore...
        let mut raw = raw.to_owned();
        let data = unsafe{ raw.as_bytes_mut() };
        Self{
            resolution: (x, y),
            layers: data
                .chunks_exact_mut((x * y) as usize)
                .map(|row| {
                    row
                        .chunks_exact_mut(x as usize)
                        .map(|pi| {
                            // Since these are bytes, our ones and zeroes are actually 48+
                            // Yet another hacky thing to make it usable...
                            for p in pi.iter_mut() {
                                *p -= 48;
                            }
                            pi.to_vec()
                        })
                        .collect::<Vec<Row>>()
                })
                .collect::<Vec<Layer>>(),
        }
    }

    pub fn pixels_in_layer(&self, layer: usize, pixel: u8) -> usize {
        self.layers.get(layer).unwrap().iter()
            .map(|row| row.iter().filter(|&&p| pixel == p).count())
            .sum()
    }

    pub fn flat_layer(&self) -> Layer {
        let mut img = vec![vec![2; self.resolution.0 as usize]; self.resolution.1 as usize];
        for layer in self.layers.iter() {
            for y in 0..layer.len() {
                for x in 0..layer[y].len() {
                    let a = img[y][x];
                    let b = layer[y][x];
                    img[y][x] = if a == 2 { b } else { a };
                }
            }
        }
        img
    }
}

pub fn draw_layer(layer: Layer) -> String {
    layer.iter().map(|row| {
        let mut s = row.iter()
            .map(|pixel| {
                match pixel {
                    0 => '■',
                    1 => '□',
                    _ => ' ',
                }
            }).collect::<String>();
        s.push('\n');
        s
    }).collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_load() {
        let img = SpaceImage::load((2, 2), "0222112222120000");
        
        assert_eq!(img.layers.len(), 4);
        assert_eq!(img.layers[0].len(), 2);
        assert_eq!(img.layers[0][0].len(), 2);
    }
}