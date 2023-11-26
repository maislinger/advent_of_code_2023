fn main() -> anyhow::Result<()> {
    rayon::ThreadPoolBuilder::new()
        .num_threads(16)
        .build_global()
        .unwrap();

    let inputs = {
        let mut inputs = Vec::with_capacity(25);
        for i in 1..=25 {
            let path = format!("./inputs/input_{:02}", i);
            let input = read_input_file(&path)?;
            inputs.push(input);
        }
        inputs
    };

    let mut solutions: Vec<anyhow::Result<String>> = Vec::with_capacity(25);
    for _ in 0..25 {
        solutions.push(Ok("".into()));
    }

    let funcs = [d01::solve, d01::solve];

    let t = std::time::Instant::now();

    rayon::scope(|s| {
        let mut sols: &mut [anyhow::Result<String>] = &mut solutions;
        for (f, input) in funcs.iter().zip(inputs.iter()) {
            let (single, rest) = sols.split_at_mut(1);
            s.spawn(move |_| single[0] = f(input));
            sols = rest;
        }
    });

    let dt = t.elapsed();

    for (i, solution) in solutions.iter().enumerate() {
        if let Err(e) = solution {
            println!("There was an error (day {:02}): {}", i + 1, e)
        }
    }

    for solution in solutions.iter().flatten() {
        print!("{}", solution);
    }

    println!("Elapsed time for all solutions: {:?}", dt);

    Ok(())
}

fn read_input_file(path: &str) -> anyhow::Result<String> {
    let r = std::fs::read_to_string(path)?;
    Ok(r)
}
