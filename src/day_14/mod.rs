use std::fmt::{self};

#[derive(Clone, Copy, Debug)]
struct CaveCoord {
    x: usize,
    y: usize,
    is_sand: bool,
}
struct Cave {
    left_bound: usize,
    right_bound: usize,
    bottom_bound: usize,
    upper_bound: usize,
    floor_y: usize,
    sand_start: CaveCoord,
    rocks: Vec<CaveCoord>,
}

impl Cave {
    fn new() -> Self {
        Self {
            left_bound: usize::MAX.clone(),
            right_bound: usize::MIN.clone(),
            bottom_bound: usize::MIN.clone(),
            upper_bound: 0,
            floor_y: 0,
            sand_start: CaveCoord {
                x: 500,
                y: 0,
                is_sand: true,
            },
            rocks: vec![],
        }
    }

    fn in_bounds(&self, coord: &CaveCoord) -> bool {
        coord.x <= self.right_bound && coord.x >= self.left_bound && coord.y <= self.bottom_bound
    }

    fn push_rock(&mut self, rock: CaveCoord, is_new_formation: bool, move_bound: bool) {
        if rock.x < self.left_bound {
            self.left_bound = rock.x;
        }

        if rock.x > self.right_bound {
            self.right_bound = rock.x;
        }

        if rock.y > self.bottom_bound && move_bound {
            self.bottom_bound = rock.y;
            self.floor_y = self.bottom_bound + 2;
        }

        if !is_new_formation {
            self.fill_out_rocks(&rock);
        }

        self.rocks.push(rock);
    }

    fn fill_out_rocks(&mut self, new_rock: &CaveCoord) {
        let last = self.rocks.last().cloned();

        if let Some(last_rock) = last {
            if last_rock.x > new_rock.x {
                for i in (last_rock.x - (last_rock.x - new_rock.x) + 1)..last_rock.x {
                    self.rocks.push(CaveCoord {
                        x: i,
                        y: last_rock.y,
                        is_sand: false,
                    });
                }
            } else if last_rock.x < new_rock.x {
                for i in (new_rock.x - (new_rock.x - last_rock.x) + 1)..new_rock.x {
                    self.rocks.push(CaveCoord {
                        x: i,
                        y: last_rock.y,
                        is_sand: false,
                    });
                }
            }

            if last_rock.y > new_rock.y {
                for i in (last_rock.y - (last_rock.y - new_rock.y) + 1)..last_rock.y {
                    self.rocks.push(CaveCoord {
                        x: last_rock.x,
                        y: i,
                        is_sand: false,
                    });
                }
            } else if last_rock.y < new_rock.y {
                for i in (new_rock.y - (new_rock.y - last_rock.y) + 1)..new_rock.y {
                    self.rocks.push(CaveCoord {
                        x: last_rock.x,
                        y: i,
                        is_sand: false,
                    });
                }
            }
        } else {
            ()
        }
    }

    fn has_rock(&self, rock: &CaveCoord) -> Option<CaveCoord> {
        if rock.y == self.floor_y {
            return Some(CaveCoord {
                x: rock.x,
                y: rock.y,
                is_sand: false,
            });
        }

        let r = self
            .rocks
            .iter()
            .find(|&cave_rock| cave_rock.x == rock.x && cave_rock.y == rock.y)
            .cloned();

        r
    }

    /*

    } */
}

impl fmt::Debug for Cave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        println!("CAVE: ");
        for j in self.upper_bound..=self.floor_y {
            for i in self.left_bound..=self.right_bound {
                if j == self.floor_y {
                    write!(f, "#");
                } else {
                    let rock = self.has_rock(&CaveCoord {
                        x: i,
                        y: j,
                        is_sand: false,
                    });
                    if let Some(r) = rock {
                        if r.is_sand {
                            write!(f, "o");
                        } else {
                            write!(f, "#");
                        }
                    } else {
                        write!(f, ".");
                    }
                }
            }
            writeln!(f, "");
        }

        Ok(())
    }
}

pub fn solution(input: String) -> usize {
    let mut cave = Cave::new();

    for rock_path in input.lines() {
        let rocks = rock_path.split(" -> ").collect::<Vec<&str>>();

        let mut is_new_formation = true;
        for &rock in rocks.iter() {
            let coords = rock.split(",").collect::<Vec<&str>>();

            let x = coords[0].parse::<usize>().unwrap();
            let y = coords[1].parse::<usize>().unwrap();

            cave.push_rock(
                CaveCoord {
                    x,
                    y,
                    is_sand: false,
                },
                is_new_formation,
                true,
            );
            is_new_formation = false;
        }
    }

    // println!(
    //     "len {} bounds l{} r{} b{}{:?}",
    //     cave.rocks.len(),
    //     cave.left_bound,
    //     cave.right_bound,
    //     cave.bottom_bound,
    //     cave.rocks
    // );

    // println!("{}", cave.floor_y);
    // return 42;

    let mut is_sand_falling_forever = false;
    let mut sand_count = 0usize;

    while !is_sand_falling_forever {
        sand_count += 1;
        let sand_start = cave.sand_start;

        let mut current_sand = sand_start;
        let rock = cave.has_rock(&CaveCoord {
            x: current_sand.x,
            y: current_sand.y,
            is_sand: true,
        });
        if rock.is_some() && rock.unwrap().is_sand {
            println!("CAVE HAS ROCK IN {current_sand:?}");
            break;
        }

        loop {
            let next_move = CaveCoord {
                x: current_sand.x,
                y: current_sand.y + 1,
                is_sand: true,
            };

            if current_sand.y == cave.floor_y {
                cave.push_rock(current_sand, true, false);
                break;
            }

            // if !cave.in_bounds(&next_move) {
            //     println!("SAND IS NOT IN BOUNDS {next_move:?}");
            //     is_sand_falling_forever = true;
            //     break;
            // }

            if cave.has_rock(&next_move).is_some() {
                let move_left = CaveCoord {
                    x: next_move.x - 1,
                    y: next_move.y,
                    is_sand: true,
                };

                // if !cave.in_bounds(&move_left) {
                //     println!("SAND IS NOT IN BOUNDS {move_left:?}");

                //     is_sand_falling_forever = true;
                //     break;
                // }

                if cave.has_rock(&move_left).is_some() {
                    let move_right = CaveCoord {
                        x: next_move.x + 1,
                        y: next_move.y,
                        is_sand: true,
                    };

                    // if !cave.in_bounds(&move_right) {
                    //     println!("SAND IS NOT IN BOUNDS {move_right:?}");

                    //     is_sand_falling_forever = true;
                    //     break;
                    // }

                    if cave.has_rock(&move_right).is_some() {
                        cave.push_rock(current_sand, true, false);
                        // println!("{cave:?}");
                        break;
                    } else {
                        current_sand = move_right;
                    }
                } else {
                    current_sand = move_left;
                }
            } else {
                current_sand = next_move;
            }
        }
    }

    println!("{cave:?}");
    sand_count - 1
}
