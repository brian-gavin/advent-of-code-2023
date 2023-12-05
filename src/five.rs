use std::ops::Range;

struct Mapping {
    dst_range: Range<u64>,
    src_range: Range<u64>,
}

impl Mapping {
    fn map(&self, src: u64) -> Option<u64> {
        self.src_range
            .contains(&src)
            .then(|| self.dst_range.start + (src - self.src_range.start))
    }

    fn rmap(&self, dst: u64) -> Option<u64> {
        self.dst_range
            .contains(&dst)
            .then(|| self.src_range.start + (dst - self.dst_range.start))
    }
}

struct Mappings(Vec<Mapping>);

impl Mappings {
    fn map(&self, src: u64) -> u64 {
        self.0.iter().find_map(|m| m.map(src)).unwrap_or(src)
    }

    fn rmap(&self, dst: u64) -> u64 {
        self.0.iter().find_map(|m| m.rmap(dst)).unwrap_or(dst)
    }
}

struct Maps {
    seed_to_soil: Mappings,
    soil_to_fertilizer: Mappings,
    fertilizer_to_water: Mappings,
    water_to_light: Mappings,
    light_to_temperature: Mappings,
    temperature_to_humidity: Mappings,
    humidity_to_location: Mappings,
}

impl Maps {
    const fn mappers(&self) -> [&Mappings; 7] {
        [
            &self.seed_to_soil,
            &self.soil_to_fertilizer,
            &self.fertilizer_to_water,
            &self.water_to_light,
            &self.light_to_temperature,
            &self.temperature_to_humidity,
            &self.humidity_to_location,
        ]
    }

    fn seed_to_location(&self, seed: u64) -> u64 {
        self.mappers().into_iter().fold(seed, |src, m| m.map(src))
    }

    fn location_to_seed(&self, location: u64) -> u64 {
        self.mappers()
            .into_iter()
            .rev()
            .fold(location, |dst, m| m.rmap(dst))
    }
}

pub fn solve1(input: crate::Input) -> u64 {
    let (seeds, maps) = parse_input(input);
    seeds
        .into_iter()
        .map(|seed| maps.seed_to_location(seed))
        .min()
        .unwrap()
}

pub fn solve2(input: crate::Input) -> u64 {
    let (seeds, maps) = parse_input(input);
    let seeds = seeds
        .chunks_exact(2)
        .map(|r| r[0]..r[0] + r[1])
        .collect::<Vec<_>>();
    let find_seed = |seed| seeds.iter().find(|r| r.contains(&seed)).is_some();
    use rayon::prelude::*;
    (0..u64::MAX)
        .into_par_iter()
        .find_first(|location| find_seed(maps.location_to_seed(*location)))
        .unwrap()
}

fn parse_input(mut input: crate::Input) -> (Vec<u64>, Maps) {
    fn parse_mapping(line: &str) -> Mapping {
        let mut line = line
            .split_ascii_whitespace()
            .map(|n| n.parse::<u64>().unwrap());
        let (dst_start, src_start, len) = (
            line.next().unwrap(),
            line.next().unwrap(),
            line.next().unwrap(),
        );
        Mapping {
            dst_range: dst_start..dst_start + len,
            src_range: src_start..src_start + len,
        }
    }
    fn parse_mappings(it: &mut crate::Input) -> Mappings {
        Mappings(
            it.by_ref()
                .skip(1)
                .map_while(|line| (!line.is_empty()).then(|| parse_mapping(&line)))
                .collect(),
        )
    }
    let seeds = input
        .next()
        .map(|line| {
            line.strip_prefix("seeds: ")
                .unwrap()
                .split_ascii_whitespace()
                .map(|s| s.parse::<u64>().ok().unwrap())
                .collect::<Vec<_>>()
        })
        .unwrap();
    let _ = input.next();
    let it = &mut input;
    let maps = Maps {
        seed_to_soil: parse_mappings(it),
        soil_to_fertilizer: parse_mappings(it),
        fertilizer_to_water: parse_mappings(it),
        water_to_light: parse_mappings(it),
        light_to_temperature: parse_mappings(it),
        temperature_to_humidity: parse_mappings(it),
        humidity_to_location: parse_mappings(it),
    };
    (seeds, maps)
}

mod tests {
    #[test]
    fn test_mapping() {
        use super::{Mapping, Mappings};
        let m1 = Mapping {
            dst_range: 50..50 + 2,
            src_range: 98..98 + 2,
        };
        let m2 = Mapping {
            dst_range: 52..52 + 48,
            src_range: 50..50 + 48,
        };
        assert_eq!(m1.map(98), Some(50));
        assert_eq!(m2.map(53), Some(55));
        let ms = Mappings(vec![m1, m2]);
        assert_eq!(ms.map(98), 50);
        assert_eq!(ms.map(53), 55);
        assert_eq!(ms.map(10), 10);
    }
}
