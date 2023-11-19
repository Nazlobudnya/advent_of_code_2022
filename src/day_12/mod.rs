#![allow(dead_code)]

use core::fmt;
use std::collections::{HashMap, HashSet};

static mut START: (usize, usize) = (0, 0);
static mut DEST: (usize, usize) = (0, 0);
static mut STATES_REACHED: usize = 0;

#[derive(Clone, Copy)]
enum Cell {
    Start,
    End,
    Square(u8),
}

impl Cell {
    fn elevation(self) -> u8 {
        match self {
            Cell::Start => 0,
            Cell::End => 25,
            Cell::Square(e) => e,
        }
    }
}

struct CellRecord {
    prev: Option<GridCoord>,
}

pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
    visited: HashMap<GridCoord, CellRecord>,
    current: HashSet<GridCoord>,
    num_steps: usize,
    start: GridCoord,
    end: GridCoord,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let first_line = input.lines().next().unwrap();
        let width = first_line.len();
        let height = input.lines().count();
        let mut cells = vec![];
        let mut start = GridCoord { x: 0, y: 0 };
        let mut end = GridCoord { x: 0, y: 0 };

        input.lines().enumerate().for_each(|(line_idx, line)| {
            line.chars().enumerate().for_each(|(ch_idx, ch)| {
                let cell = match ch {
                    'S' => {
                        start.x = ch_idx;
                        start.y = line_idx;
                        Cell::Start
                    }
                    'E' => {
                        end.x = ch_idx;
                        end.y = line_idx;
                        Cell::End
                    }
                    'a'..='z' => Cell::Square(ch as u8 - b'a'),
                    _ => panic!("invalid character: {ch}"),
                };

                cells.push(cell);
            })
        });

        Self {
            width,
            height,
            cells,
            current: Default::default(),
            visited: Default::default(),
            num_steps: 0,
            start,
            end,
        }
    }

    fn in_bounds(&self, coord: GridCoord) -> bool {
        coord.x < self.width && coord.y < self.height
    }

    fn cell(&self, coord: GridCoord) -> Option<&Cell> {
        if !self.in_bounds(coord) {
            return None;
        }
        Some(&self.cells[coord.y * self.width + coord.x])
    }

    fn cell_mut(&mut self, coord: GridCoord) -> Option<&mut Cell> {
        if !self.in_bounds(coord) {
            return None;
        }
        Some(&mut self.cells[coord.y * self.width + coord.x])
    }

    fn walkable_neighbors(&self, coord: GridCoord) -> impl Iterator<Item = GridCoord> + '_ {
        let curr_elev = self.cell(coord).unwrap().elevation();
        let deltas: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        deltas.into_iter().filter_map(move |(dx, dy)| {
            Some(GridCoord {
                x: coord.x.checked_add_signed(dx)?,
                y: coord.y.checked_add_signed(dy)?,
            })
            .filter(|&coord| {
                self.in_bounds(coord) && self.cell(coord).unwrap().elevation() <= curr_elev + 1
            })
        })
    }

    fn backwards_walkable_neighbors(
        &self,
        coord: GridCoord,
    ) -> impl Iterator<Item = GridCoord> + '_ {
        let curr_elev = self.cell(coord).unwrap().elevation();
        let deltas: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        deltas.into_iter().filter_map(move |(dx, dy)| {
            Some(GridCoord {
                x: coord.x.checked_add_signed(dx)?,
                y: coord.y.checked_add_signed(dy)?,
            })
            .filter(|&coord| {
                self.in_bounds(coord) && self.cell(coord).unwrap().elevation() + 1 >= curr_elev
            })
        })
    }

    pub fn solve_1(&mut self) -> usize {
        if self.current.is_empty() {
            // find start coordinate
            let mut start_coord: Option<GridCoord> = None;
            for y in 0..self.height {
                for x in 0..self.width {
                    let coord: GridCoord = (x, y).into();
                    if let Cell::Start = self.cell(coord).unwrap() {
                        start_coord = Some(coord);
                        break;
                    }
                }
            }
            let start_coord = start_coord.unwrap();
            self.current.insert(start_coord);
            self.visited.insert(start_coord, CellRecord { prev: None });
        }

        while !self.current.is_empty() {
            let current = std::mem::take(&mut self.current);
            let mut next = HashSet::new();
            let mut visited = std::mem::take(&mut self.visited);

            for curr in current {
                for ncoord in self.walkable_neighbors(curr) {
                    if visited.contains_key(&ncoord) {
                        // don't visit it again!
                        continue;
                    }

                    if self.end == ncoord {
                        println!("{:?} {:?}", self.end, ncoord);
                        return self.num_steps + 1;
                    };

                    visited.insert(ncoord, CellRecord { prev: Some(curr) });
                    next.insert(ncoord);
                }
            }
            self.current = next;
            self.visited = visited;
            self.num_steps += 1;
        }

        self.num_steps
    }

    pub fn solve_2(&mut self) -> usize {
        if self.current.is_empty() {
            // find start coordinate
            let mut start_coord: Option<GridCoord> = None;
            for y in 0..self.height {
                for x in 0..self.width {
                    let coord: GridCoord = (x, y).into();
                    if let Cell::End = self.cell(coord).unwrap() {
                        start_coord = Some(coord);
                        break;
                    }
                }
            }
            let start_coord = start_coord.unwrap();
            self.current.insert(start_coord);
            self.visited.insert(start_coord, CellRecord { prev: None });
        }

        while !self.current.is_empty() {
            let current = std::mem::take(&mut self.current);
            let mut next = HashSet::new();
            let mut visited = std::mem::take(&mut self.visited);

            for curr in current {
                for ncoord in self.backwards_walkable_neighbors(curr) {
                    if visited.contains_key(&ncoord) {
                        // don't visit it again!
                        continue;
                    }

                    if let Some(c) = self.cell(ncoord) {
                        match c {
                            Cell::Square(x) => {
                                if *x == 0 {
                                    return self.num_steps + 1;
                                }
                            }
                            _ => {}
                        }
                    }

                    visited.insert(ncoord, CellRecord { prev: Some(curr) });
                    next.insert(ncoord);
                }
            }
            self.current = next;
            self.visited = visited;
            self.num_steps += 1;
        }

        self.num_steps
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} by {}:", self.width, self.height)?;
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.cell((x, y).into()).unwrap();
                let c = match cell {
                    Cell::Start => 'S',
                    Cell::End => 'E',
                    Cell::Square(elevation) => (b'a' + elevation) as char,
                };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct GridCoord {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for GridCoord {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

pub fn solution(input: String) -> usize {
    let mut grid = Grid::parse(&input);
    println!("{grid:?}");

    let r = grid.solve_2();

    r
    // const LINES: usize = 41;
    // const COLS: usize = 113;

    // // let mut arr = [['.'; 113]; 41];
    // let mut arr = [[0; COLS]; LINES];

    // input.lines().enumerate().for_each(|(line_idx, line)| {
    //     line.chars().enumerate().for_each(|(ch_idx, mut ch)| {
    //         if ch == 'S' {
    //             unsafe { START = (line_idx, ch_idx) };
    //             ch = 'a';
    //         }

    //         if ch == 'E' {
    //             unsafe {
    //                 DEST = (line_idx, ch_idx);
    //             }

    //             ch = 'z';
    //         }

    //         arr[line_idx][ch_idx] = ch as u8 - 'a' as u8;
    //     })
    // });

    // for i in 0..LINES {
    //     for j in 0..COLS {
    //         print!(
    //             "{}",
    //             char::from_u32((arr[i][j] + 'a' as u8) as u32).unwrap()
    //         );
    //     }
    //     println!("");
    // }

    // let mut move_map: HashMap<(usize, usize), usize> = HashMap::new();
    // let r = solve_maze(
    //     &arr,
    //     unsafe { START },
    //     unsafe { START },
    //     0,
    //     &mut usize::MAX.clone(),
    //     &mut move_map,
    // );

    // println!("{}", unsafe { STATES_REACHED });
    // println!("{r:?}");
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum STATE {
    STUCK,
    FINISHED(usize),
}

const MAX_MOVES: usize = 15;

fn solve_maze(
    arr: &[[u8; 113]; 41],
    current: (usize, usize),
    previous: (usize, usize),
    move_number: usize,
    current_lowest: &mut usize,
    move_map: &mut HashMap<(usize, usize), usize>,
) -> STATE {
    let mut res: Vec<STATE> = Vec::with_capacity(3);
    let current_char = arr[current.0][current.1];
    let v = move_map.get(&current);

    if v.is_some() {
        if *v.unwrap() <= move_number {
            return STATE::STUCK;
        }
    } else {
        move_map.insert(current, move_number);
    }

    unsafe {
        STATES_REACHED += 1;
    }
    if move_number >= *current_lowest {
        return STATE::STUCK;
    }

    if current == unsafe { DEST } {
        // println!("REACHED DEST {current_char} {previous:?}, {move_number}");
        return STATE::FINISHED(move_number);
    }

    if move_number == MAX_MOVES {
        return STATE::STUCK;
    }

    if current.1 > 0 {
        let to_left = (current.0, current.1 - 1);
        let elem = arr.get(to_left.0).unwrap().get(to_left.1);
        if elem.is_some() && to_left != previous {
            if *elem.unwrap() > current_char && (*elem.unwrap() - current_char) > 1 {
                // println!(
                //     "[X] Cannot move move = {move_number} elem = {} current {}",
                //     *elem.unwrap(),
                //     current_char
                // );
                res.push(STATE::STUCK);
            } else {
                let st = solve_maze(
                    arr,
                    to_left,
                    current,
                    move_number + 1,
                    current_lowest,
                    move_map,
                );
                // println!(
                //     "[O] Can move = {move_number} elem = {} current {}, moves = {st}",
                //     *elem.unwrap(),
                //     current_char
                // );
                res.push(st);
            }
        }
    }

    let to_right = (current.0, current.1 + 1);
    let elem = arr.get(to_right.0).unwrap().get(to_right.1);
    if elem.is_some() && to_right != previous {
        if *elem.unwrap() > current_char && (*elem.unwrap() - current_char) > 1 {
            // println!(
            //     "[X] Cannot move move = {move_number} elem = {} current {}",
            //     *elem.unwrap(),
            //     current_char
            // );
            res.push(STATE::STUCK);
        } else {
            let st = solve_maze(
                arr,
                to_right,
                current,
                move_number + 1,
                current_lowest,
                move_map,
            );

            match st {
                STATE::FINISHED(x) => {
                    if x < *current_lowest {
                        *current_lowest = x;
                    }
                }
                _ => {}
            }
            // println!(
            //     "[O] Can move = {move_number} elem = {} current {}, moves = {st}",
            //     *elem.unwrap(),
            //     current_char
            // );
            res.push(st);
        }
    }

    if current.0 > 0 {
        let to_up = (current.0 - 1, current.1);
        let elem = arr.get(to_up.0);
        if elem.is_some() && to_up != previous {
            let elem = elem.unwrap().get(to_up.1);
            if *elem.unwrap() > current_char && (*elem.unwrap() - current_char) > 1 {
                // println!(
                //     "[X] Cannot move move = {move_number} elem = {} current {}",
                //     *elem.unwrap(),
                //     current_char
                // );
                res.push(STATE::STUCK);
            } else {
                let st = solve_maze(
                    arr,
                    to_up,
                    current,
                    move_number + 1,
                    current_lowest,
                    move_map,
                );

                match st {
                    STATE::FINISHED(x) => {
                        if x < *current_lowest {
                            *current_lowest = x;
                        }
                    }
                    _ => {}
                }

                // println!(
                //     "[O] Can move = {move_number} elem = {} current {}, moves = {st}",
                //     *elem.unwrap(),
                //     current_char
                // );
                res.push(st);
            }
        }
    }

    let to_down = (current.0 + 1, current.1);
    let elem = arr.get(to_down.0);
    if elem.is_some() && to_down != previous {
        let elem = elem.unwrap().get(to_down.1);
        if *elem.unwrap() > current_char && (*elem.unwrap() - current_char) > 1 {
            // println!(
            //     "[X] Cannot move move = {move_number} elem = {} current {}",
            //     *elem.unwrap(),
            //     current_char
            // );
            res.push(STATE::STUCK);
        } else {
            let st = solve_maze(
                arr,
                to_down,
                current,
                move_number + 1,
                current_lowest,
                move_map,
            );

            match st {
                STATE::FINISHED(x) => {
                    if x < *current_lowest {
                        *current_lowest = x;
                    }
                }
                _ => {}
            }
            // println!(
            //     "[O] Can move = {move_number} elem = {} current {}, moves = {st}",
            //     *elem.unwrap(),
            //     current_char
            // );
            res.push(st);
        }
    }

    res.into_iter()
        .filter(|st| match st {
            STATE::FINISHED(_) => true,
            _ => false,
        })
        .min()
        .unwrap_or(STATE::STUCK)
}
