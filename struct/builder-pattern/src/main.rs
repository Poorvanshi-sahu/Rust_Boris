#[derive(Debug)]

struct Computer{
    cpu: String,
    memory: u32,
    hard_drive_capacity: u32,
}

// tuple like structs
struct ShortDuration(u32, u32);

struct LongDuration(u32, u32);

// unit like structs
struct simple;

impl Computer{
    fn new(cpu: String, memory: u32, hard_drive_capacity: u32)->Self{
        Self{
            cpu,
            memory,
            hard_drive_capacity
        }
    }

    fn upgrade_cpu(&mut self, cpu: String)->&mut Self{
        self.cpu = cpu;
        self
    }

    fn upgrade_memory(&mut self, memory: u32)->&mut Self{
        self.memory = memory;
        self
    }

    fn upgrade_hard_drive_capacity(&mut self, hard_drive_capacity: u32)->&mut Self{
        self.hard_drive_capacity = hard_drive_capacity;
        self
    }

    fn print_value(&self){
        println!("cpu: {}", self.cpu);
        println!("memory: {}", self.memory);
        println!("hard drive: {}", self.hard_drive_capacity);
    }

    // named field
    // tuple like
    // uni like
}

fn print_shift(length: (u32, u32)){
    println!("{} {}", length.0, length.1)
}

fn main() {
    let mut Computer = Computer::new(String::from("intel"), 16, 512);

    Computer.print_value();
    Computer.upgrade_cpu(String::from("snap")).upgrade_memory(32).upgrade_hard_drive_capacity(1);
    Computer.print_value();

    println!("{Computer:#?}");

    // tuple like structs
    let small_shift = ShortDuration(12, 0);
    let big_shift = LongDuration(24, 0);

    // print_shift(big_shift);
}

