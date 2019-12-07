use intcode::*;
use permutohedron::LexicalPermutation;
use std::collections::VecDeque;

fn main() -> DynResult<()> {
    let amp_code = parse_intcode(&std::fs::read("day7-input.txt")?)?;
    let mut phases = [0, 1, 2, 3, 4];
    let mut phases_feedback = [5, 6, 7, 8, 9];

    let mut max_output = 0;
    let mut max_output_feedback = 0;
    let mut num_perms = 0;

    loop {
        let output = run_thruster_amps(&amp_code, phases, false);
        let output_feedback = run_thruster_amps(&amp_code, phases_feedback, true);

        max_output = max_output.max(output.into_iter().last().unwrap());
        max_output_feedback = max_output_feedback.max(output_feedback.into_iter().last().unwrap());
        num_perms += 1;

        if !phases.next_permutation() || !phases_feedback.next_permutation() {
            break;
        }
    }

    println!("Permutations: {}", num_perms);
    println!("Max output without feedback: {}", max_output);
    println!("Max output with feedback: {}", max_output_feedback);
    Ok(())
}

fn run_thruster_amps(code: &[Word], phases: [Word; 5], feedback: bool) -> VecDeque<Word> {
    let mut amp_a = Machine::new(code.to_vec());
    let mut amp_b = Machine::new(code.to_vec());
    let mut amp_c = Machine::new(code.to_vec());
    let mut amp_d = Machine::new(code.to_vec());
    let mut amp_e = Machine::new(code.to_vec());

    let mut buf0 = IoBuffer::with_data(&[phases[0], 0]);
    let mut buf1 = IoBuffer::with_data(&[phases[1]]);
    let mut buf2 = IoBuffer::with_data(&[phases[2]]);
    let mut buf3 = IoBuffer::with_data(&[phases[3]]);
    let mut buf4 = IoBuffer::with_data(&[phases[4]]);
    let mut buf5 = IoBuffer::new();

    loop {
        let res_a = amp_a.step(&mut PipedIo::new(&mut buf0, &mut buf1));
        let res_b = amp_b.step(&mut PipedIo::new(&mut buf1, &mut buf2));
        let res_c = amp_c.step(&mut PipedIo::new(&mut buf2, &mut buf3));
        let res_d = amp_d.step(&mut PipedIo::new(&mut buf3, &mut buf4));
        let res_e_out = if feedback { &mut buf0 } else { &mut buf5 };
        let res_e = amp_e.step(&mut PipedIo::new(&mut buf4, res_e_out));

        match res_a.join(res_b).join(res_c).join(res_d).join(res_e) {
            StepResult::Continue => continue,
            StepResult::Halt => break,
            StepResult::IoBlocked => panic!("All amplifiers blocked on IO"),
        }
    }

    if feedback {
        buf0.into_inner()
    } else {
        buf5.into_inner()
    }
}

#[test]
fn test_thrusters_example1() {
    let output = run_thruster_amps(
        &[
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ],
        [4, 3, 2, 1, 0],
        false,
    );
    assert_eq!(output, &[43210]);
}

#[test]
fn test_thrusters_example2() {
    let output = run_thruster_amps(
        &[
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ],
        [0, 1, 2, 3, 4],
        false,
    );
    assert_eq!(output, &[54321]);
}

#[test]
fn test_thrusters_example3() {
    let output = run_thruster_amps(
        &[
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ],
        [1, 0, 4, 3, 2],
        false,
    );
    assert_eq!(output, &[65210]);
}

#[test]
fn test_thrusters_feedback_example1() {
    let output = run_thruster_amps(
        &[
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ],
        [9, 8, 7, 6, 5],
        true,
    );
    assert_eq!(output, &[139629729]);
}

#[test]
fn test_thrusters_feedback_example2() {
    let output = run_thruster_amps(
        &[
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ],
        [9, 7, 8, 5, 6],
        true,
    );
    assert_eq!(output, &[18216]);
}
