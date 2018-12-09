use day07::{Steps, Workers};

#[test]
fn examples() {
    let input = [
        "Step C must be finished before step A can begin.",
        "Step C must be finished before step F can begin.",
        "Step A must be finished before step B can begin.",
        "Step A must be finished before step D can begin.",
        "Step B must be finished before step E can begin.",
        "Step D must be finished before step E can begin.",
        "Step F must be finished before step E can begin.",
    ]
    .join("\n");

    let mut steps: Steps = input.parse().unwrap();
    let num_workers = 2;
    let base_step_time = 0;
    let mut workers = Workers::new(steps.clone(), num_workers, base_step_time);
    assert_eq!(steps.get_serial_order(), "CABDFE");
    assert_eq!(workers.time_all_steps(), 15);
}
