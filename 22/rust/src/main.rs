#![feature(assert_matches)]
use itertools::Itertools;
use serde_scan::scan;
use std::str::FromStr;
use Switch::{Off, On};

#[derive(Debug, Clone, Copy)]
struct Instruction {
    switch: Switch,
    cuboid: Cuboid,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Switch {
    On,
    Off,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cuboid {
    x: Range,
    y: Range,
    z: Range,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum CuboidDiff {
    NoOverlap,
    Diff(Vec<Cuboid>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Range {
    lo: i32,
    hi: i32,
}

fn main() {
    let input = include_str!("../../input.txt");
    let instructions: Vec<Instruction> = input
        .lines()
        .map(Instruction::from_str)
        .try_collect()
        .expect("invalid input");
    let on_cuboids = reboot(&instructions);
    println!(
        "part 2: {}",
        on_cuboids
            .iter()
            .map(Cuboid::volume_in_start_area)
            .sum::<usize>()
    );
    println!(
        "part 2: {}",
        on_cuboids.iter().map(Cuboid::volume).sum::<usize>()
    );
}

fn reboot(instructions: &[Instruction]) -> Vec<Cuboid> {
    let mut cuboids: Vec<Cuboid> = Vec::new();
    for Instruction { switch, cuboid } in instructions {
        let mut to_add = Vec::new();
        cuboids.retain(|prev_cuboid| match prev_cuboid.diff(*cuboid) {
            CuboidDiff::NoOverlap => true,
            CuboidDiff::Diff(mut diff) => {
                to_add.append(&mut diff);
                false
            }
        });
        cuboids.append(&mut to_add);
        if *switch == On {
            cuboids.push(*cuboid);
        }
    }
    cuboids
}

impl Range {
    fn of(lo: i32, hi: i32) -> Range {
        Range { lo, hi }
    }

    fn intersect(&self, other: Range) -> Option<Range> {
        let lo = i32::max(self.lo, other.lo);
        let hi = i32::min(self.hi, other.hi);
        (lo <= hi).then(|| Range { lo, hi })
    }

    fn len(&self) -> usize {
        (self.hi - self.lo) as usize + 1
    }

    fn len_in_start_area(&self) -> usize {
        let hi = (self.hi + 1).clamp(-50, 50);
        let lo = self.lo.clamp(-50, 50);
        (hi - lo) as usize
    }
}

impl Cuboid {
    fn intersect(&self, other: Cuboid) -> Option<Cuboid> {
        self.x.intersect(other.x).and_then(|xrange| {
            self.y.intersect(other.y).and_then(|yrange| {
                self.z.intersect(other.z).map(|zrange| Cuboid {
                    x: xrange,
                    y: yrange,
                    z: zrange,
                })
            })
        })
    }

    fn diff(&self, other: Cuboid) -> CuboidDiff {
        match self.intersect(other) {
            None => CuboidDiff::NoOverlap,
            Some(intersection) => {
                let mut result = Vec::new();
                if self.x.lo < intersection.x.lo {
                    result.push(Cuboid {
                        x: Range::of(self.x.lo, intersection.x.lo - 1),
                        y: self.y,
                        z: self.z,
                    });
                }
                if self.x.hi > other.x.hi {
                    result.push(Cuboid {
                        x: Range::of(intersection.x.hi + 1, self.x.hi),
                        y: self.y,
                        z: self.z,
                    });
                }
                let x_range_intersection = self.x.intersect(intersection.x).unwrap();
                if self.y.lo < intersection.y.lo {
                    result.push(Cuboid {
                        x: x_range_intersection,
                        y: Range::of(self.y.lo, intersection.y.lo - 1),
                        z: self.z,
                    });
                }
                if self.y.hi > intersection.y.hi {
                    result.push(Cuboid {
                        x: x_range_intersection,
                        y: Range::of(intersection.y.hi + 1, self.y.hi),
                        z: self.z,
                    });
                }
                let y_range_intersection = self.y.intersect(intersection.y).unwrap();
                if self.z.lo < intersection.z.lo {
                    result.push(Cuboid {
                        x: x_range_intersection,
                        y: y_range_intersection,
                        z: Range::of(self.z.lo, intersection.z.lo - 1),
                    });
                }
                if self.z.hi > intersection.z.hi {
                    result.push(Cuboid {
                        x: x_range_intersection,
                        y: y_range_intersection,
                        z: Range::of(intersection.z.hi + 1, self.z.hi),
                    })
                }
                CuboidDiff::Diff(result)
            }
        }
    }

    fn volume(&self) -> usize {
        self.x.len() * self.y.len() * self.z.len()
    }

    fn volume_in_start_area(&self) -> usize {
        self.x.len_in_start_area() * self.y.len_in_start_area() * self.z.len_in_start_area()
    }
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (switch, xmin, xmax, ymin, ymax, zmin, zmax): (&str, i32, i32, i32, i32, i32, i32) =
            scan!("{} x={}..{},y={}..{},z={}..{}" <- s)?;
        Ok(Instruction {
            switch: if switch == "on" { On } else { Off },
            cuboid: Cuboid {
                x: Range { lo: xmin, hi: xmax },
                y: Range { lo: ymin, hi: ymax },
                z: Range { lo: zmin, hi: zmax },
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{Cuboid, CuboidDiff, Range};
    use std::assert_matches::assert_matches;

    const UNIT_SQ: Cuboid = Cuboid {
        x: Range { lo: 0, hi: 0 },
        y: Range { lo: 0, hi: 0 },
        z: Range { lo: 0, hi: 0 },
    };

    const UNIT_SQ_3: Cuboid = Cuboid {
        x: Range { lo: 0, hi: 2 },
        y: Range { lo: 0, hi: 2 },
        z: Range { lo: 0, hi: 2 },
    };

    #[test]
    fn test_intersect() {
        let b = Cuboid {
            x: { Range { lo: -1, hi: 3 } },
            y: { Range { lo: -1, hi: 3 } },
            z: { Range { lo: -1, hi: 3 } },
        };
        assert_eq!(Some(UNIT_SQ_3), UNIT_SQ_3.intersect(b));
        assert_eq!(Some(UNIT_SQ), UNIT_SQ_3.intersect(UNIT_SQ))
    }

    #[test]
    fn test_difference() {
        let diff = UNIT_SQ.diff(UNIT_SQ);
        assert_matches!(diff, CuboidDiff::Diff(_));

        let diff = UNIT_SQ_3.diff(UNIT_SQ);
        dbg!(diff);
    }
}
