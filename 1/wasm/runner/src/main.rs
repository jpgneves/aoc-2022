use std::{io::BufRead, iter::Peekable};

use wasmtime::{Engine, Module, Store, Linker, Caller};

struct RunnerState {
    lines: Peekable<std::io::Lines<std::io::BufReader<std::fs::File>>>,
}

fn elf(mut caller: Caller<'_, RunnerState>) -> i32 {
    i32::from(caller.data_mut().lines.peek().is_none())
}

fn calories(mut caller: Caller<'_, RunnerState>) -> i32 {
    if let Some(cal) = caller.data_mut().lines.next() {
        let cal = cal.unwrap();
        if cal.is_empty() {
            0
        } else {
            i32::from_str_radix(&cal, 10).unwrap()
        }
    } else {
        0
    }
}

fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = std::env::args().collect();
    
    let engine = Engine::default();

    let module = Module::from_file(&engine, args.get(1).unwrap())?;

    let data = std::fs::File::open(args.get(2).unwrap())?;
    let lines = std::io::BufReader::new(data).lines().peekable();

    let mut store = Store::new(&engine, RunnerState { lines });

    let mut linker = Linker::new(&engine);

    linker.func_wrap("runner", "elf", elf)?;

    linker.func_wrap("runner", "calories", calories)?;

    let instance = linker.instantiate(&mut store, &module)?;

    let run = instance.get_typed_func::<(), i32, _>(&mut store, "run")?;

    println!("{:?}", run.call(&mut store, ())?);

    Ok(())
}
