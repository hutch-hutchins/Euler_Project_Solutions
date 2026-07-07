use std::{env, process, time::Duration};

fn main() {
    let mut command = None;
    let mut show_time = false;

    for arg in env::args().skip(1) {
        match arg.as_str() {
            "--time" => show_time = true,
            _ if command.is_none() => command = Some(arg),
            _ => print_usage_and_exit(),
        }
    }

    let command = command.unwrap_or_else(|| print_usage_and_exit());

    if command == "--all" {
        for id in euler::problems::solved_problem_ids() {
            let (answer, elapsed) =
                solve_with_timing(*id).expect("listed problem should be solved");
            if show_time {
                println!("{id:04}: {answer} in {}", format_duration(elapsed));
            } else {
                println!("{id:04}: {answer}");
            }
        }
        return;
    }

    let id = match command.parse::<u32>() {
        Ok(id) => id,
        Err(_) => print_usage_and_exit(),
    };

    match solve_with_timing(id) {
        Some((answer, elapsed)) => {
            if show_time {
                println!("{answer} in {}", format_duration(elapsed));
            } else {
                println!("{answer}");
            }
        }
        None => {
            eprintln!("Problem {id} is not solved yet.");
            process::exit(1);
        }
    }
}

fn solve_with_timing(id: u32) -> Option<(String, Duration)> {
    let start = std::time::Instant::now();
    let answer = euler::problems::solve(id)?;
    Some((answer, start.elapsed()))
}

fn format_duration(duration: Duration) -> String {
    let seconds = duration.as_secs_f64();

    if seconds >= 1.0 {
        format!("{seconds:.3} s")
    } else if seconds >= 0.001 {
        format!("{:.3} ms", seconds * 1_000.0)
    } else if seconds >= 0.000_001 {
        format!("{:.3} us", seconds * 1_000_000.0)
    } else {
        format!("{} ns", duration.as_nanos())
    }
}

fn print_usage_and_exit() -> ! {
    eprintln!("Usage:");
    eprintln!("  cargo run -- <problem-number>");
    eprintln!("  cargo run -- --all");
    eprintln!("  cargo run -- <problem-number> --time");
    eprintln!("  cargo run -- --all --time");
    process::exit(2);
}
