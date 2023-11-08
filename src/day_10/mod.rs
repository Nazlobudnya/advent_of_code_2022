#[derive(Debug)]
enum Instruction {
    NOOP,
    ADDX(isize),
}

const SHOULD_PRINT: bool = true;

struct CrtController {
    pixel_width: usize,
}

impl CrtController {
    fn new(pixel_width: usize) -> Self {
        Self { pixel_width }
    }
    fn draw(&self, cycle: usize, reg: isize) {
        let on_sprite = ((cycle - 1) % self.pixel_width) as isize >= reg - 1
            && ((cycle - 1) % self.pixel_width) as isize <= reg + 1;

        if SHOULD_PRINT {
            print!("{}", if on_sprite { "X" } else { "." });
        }

        if cycle % self.pixel_width == 0 {
            println!("");
        }
    }
}
struct Machine {
    cycle: usize,
    reg: isize,
    collector: CycleSignalCollector,
    crt_controller: CrtController,
}

impl Machine {
    fn new(collector: CycleSignalCollector, crt_controller: CrtController) -> Self {
        Self {
            cycle: 0usize,
            reg: 1isize,
            collector,
            crt_controller,
        }
    }

    fn advance(&mut self, value: Option<isize>) {
        self.cycle += 1;
        self.collector.try_collect(self.cycle, self.reg);

        self.crt_controller.draw(self.cycle, self.reg);

        if let Some(v) = value {
            self.reg += v
        }
    }

    fn simulate_instruction(&mut self, ins: Instruction) {
        match ins {
            Instruction::NOOP => {
                self.advance(None);
            }
            Instruction::ADDX(value) => {
                self.advance(None);

                self.advance(Some(value));
            }
        }
    }
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let s: Vec<&str> = value.split(' ').collect();

        match s[0] {
            "noop" => Instruction::NOOP,
            "addx" => Instruction::ADDX(s[1].parse::<isize>().unwrap()),
            _ => panic!("Only noop and addx supported"),
        }
    }
}

struct CycleSignalCollector {
    collect_at_cycle: Vec<usize>,
    store: isize,
}

impl CycleSignalCollector {
    fn new(collect_at_cycle: Vec<usize>) -> Self {
        Self {
            collect_at_cycle,
            store: 0,
        }
    }

    fn try_collect(&mut self, cycle: usize, reg: isize) -> () {
        for &c in &self.collect_at_cycle {
            if c == cycle {
                self.store += reg * c as isize;
            }
        }
    }
}

pub fn solution(input: String) -> isize {
    let instructions = input.split('\n').map(Instruction::from);

    let collector = CycleSignalCollector::new(vec![20, 60, 100, 140, 180, 220]);
    let crt_contoller = CrtController::new(40);
    let mut machine = Machine::new(collector, crt_contoller);

    for ins in instructions {
        machine.simulate_instruction(ins)
    }

    machine.collector.store
}
