use std::collections::{HashMap, VecDeque};

use bitvec::prelude::*;
use itertools::Itertools;

use crate::PuzzleInput;

#[derive(Debug, Clone, Copy)]
enum Signal {
    Low,
    High,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ModuleNetworkState {
    memory: BitVec,
}

impl ModuleNetworkState {
    fn new(memory_size: usize) -> Self {
        Self {
            memory: bitvec!(0; memory_size),
        }
    }

    fn send_to_flipflop(&mut self, memory_offset: usize, signal: Signal) -> Option<Signal> {
        if matches!(signal, Signal::Low) {
            let mut on = self.memory.get_mut(memory_offset).unwrap();
            let was_on = *on;
            on.set(!was_on);
            Some(if was_on { Signal::Low } else { Signal::High })
        } else {
            None
        }
    }

    fn send_to_conjunction(
        &mut self,
        memory_offset: usize,
        input_index: usize,
        input_count: usize,
        signal: Signal,
    ) -> Signal {
        let memory = &mut self.memory[memory_offset..memory_offset + input_count];
        memory.set(input_index, matches!(signal, Signal::High));
        if memory.all() {
            Signal::Low
        } else {
            Signal::High
        }
    }

    fn iterate_button_press<'a>(
        &'a mut self,
        broadcast_ids: &[usize],
        module_outputs: &'a [Vec<Option<usize>>],
        module_memory: &'a Vec<(usize, Option<HashMap<usize, usize>>)>,
    ) -> NetworkSignalIterator<'_> {
        let signal_queue: VecDeque<(Signal, usize, Option<usize>)> = broadcast_ids
            .iter()
            .map(|b| (Signal::Low, usize::MAX, Some(*b)))
            .collect();

        NetworkSignalIterator {
            signal_queue,
            network_state: self,
            module_memory,
            module_outputs,
        }
    }
}

struct NetworkSignalIterator<'a> {
    signal_queue: VecDeque<(Signal, usize, Option<usize>)>,
    network_state: &'a mut ModuleNetworkState,
    module_outputs: &'a [Vec<Option<usize>>],
    module_memory: &'a [(usize, Option<HashMap<usize, usize>>)],
}

impl<'a> Iterator for NetworkSignalIterator<'a> {
    type Item = (Signal, usize, Option<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((signal, sender_id, receiver_id_opt)) = self.signal_queue.pop_front() {
            if let Some(receiver_id) = receiver_id_opt {
                let outputs = &self.module_outputs[receiver_id];
                let (memory_offset, id_to_index_opt) = &self.module_memory[receiver_id];
                if let Some(next_signal) = if let Some(id_to_index) = id_to_index_opt {
                    Some(self.network_state.send_to_conjunction(
                        *memory_offset,
                        id_to_index[&sender_id],
                        id_to_index.len(),
                        signal,
                    ))
                } else {
                    self.network_state.send_to_flipflop(*memory_offset, signal)
                } {
                    self.signal_queue.extend(
                        outputs
                            .iter()
                            .map(|output_id| (next_signal, receiver_id, *output_id)),
                    );
                }
            }
            Some((signal, sender_id, receiver_id_opt))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ModuleSpec {
    FlipFlop,
    Conjunction,
}

fn run(input: PuzzleInput, part2: bool) -> usize {
    let modules: HashMap<String, (Option<ModuleSpec>, Vec<String>)> = input
        .map(|line| {
            let (label, output_nodes) = line.splitn(2, " -> ").collect_tuple().unwrap();
            let outputs = output_nodes
                .split(", ")
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect_vec();
            if label == "broadcaster" {
                (label.to_string(), (None, outputs))
            } else {
                (
                    label[1..].to_string(),
                    (
                        Some(if label.starts_with('%') {
                            ModuleSpec::FlipFlop
                        } else {
                            ModuleSpec::Conjunction
                        }),
                        outputs,
                    ),
                )
            }
        })
        .collect();

    // Note that broadcaster only has flipflop outputs
    let module_indices: HashMap<String, usize> = modules
        .iter()
        // Discard broadcaster module
        .filter(|(_, (m, _))| m.is_some())
        .enumerate()
        .map(|(i, (label, _))| (label.clone(), i))
        .collect();

    let broadcast_ids = modules["broadcaster"]
        .1
        .iter()
        .map(|o| module_indices[o])
        .collect_vec();

    let (module_specs, module_outputs): (Vec<ModuleSpec>, Vec<Vec<Option<usize>>>) = module_indices
        .iter()
        .sorted_by_key(|&(_, index)| index)
        .map(|(label, _)| {
            let (spec, outputs) = &modules[label];
            (
                spec.to_owned().unwrap(),
                outputs
                    .iter()
                    .map(|l| module_indices.get(l).copied())
                    .collect_vec(),
            )
        })
        .unzip();

    let module_inputs = {
        let mut module_inputs: Vec<Vec<usize>> =
            module_outputs.iter().map(|_| vec![]).collect_vec();
        for (index, outputs) in module_outputs.iter().enumerate() {
            for output in outputs.iter().flatten() {
                module_inputs[*output].push(index);
            }
        }
        module_inputs
    };
    let module_memory = module_specs
        .iter()
        .zip_eq(&module_inputs)
        .scan(0, |memory_offset, (m, inputs)| {
            Some(match m {
                ModuleSpec::Conjunction => {
                    let id_to_index: HashMap<usize, usize> = inputs
                        .iter()
                        .enumerate()
                        .map(|(index, id)| (*id, index))
                        .collect();
                    let offset = *memory_offset;
                    *memory_offset += inputs.len();
                    (offset, Some(id_to_index))
                }
                ModuleSpec::FlipFlop => {
                    let offset = *memory_offset;
                    *memory_offset += 1;
                    (offset, None)
                }
            })
        })
        .collect_vec();

    let mut network_state = ModuleNetworkState::new(
        module_memory
            .iter()
            .map(|(_, v)| {
                if let Some(id_to_index) = v {
                    id_to_index.len()
                } else {
                    1
                }
            })
            .sum(),
    );

    if part2 {
        // The input nework has a specific structure with flipflop cycles
        // and conjunctions before the `rx` target which wait for these cycles to align.
        // All cycles start with the first cycle.
        let last_conjunction = module_outputs
            .iter()
            .position(|m| m.iter().all(|output| output.is_none()))
            .unwrap();
        // If all cycle output conjunctions send HIGH signals
        // to the last conjunction, `rx` receives low.
        let cycle_output_conjunctions = &module_inputs[last_conjunction].clone();

        // Record cycle numbers where conjunctions emit a low signal
        let mut conjunction_low_signal: Vec<Option<usize>> =
            cycle_output_conjunctions.iter().map(|_| None).collect();
        let mut cycle = 0;
        while conjunction_low_signal.iter().any(|h| h.is_none()) {
            for (signal, sender_id, receiver_id_opt) in
                network_state.iterate_button_press(&broadcast_ids, &module_outputs, &module_memory)
            {
                if receiver_id_opt.is_some_and(|r| r == last_conjunction)
                    && matches!(signal, Signal::High)
                {
                    if let Some(cycle_low) = cycle_output_conjunctions
                        .iter()
                        .position(|c| *c == sender_id)
                    {
                        conjunction_low_signal[cycle_low] = Some(cycle);
                    }
                }
            }
            cycle += 1;
        }
        conjunction_low_signal
            .into_iter()
            .flatten()
            // A cycle restarts one cycle after the low signal is send
            .map(|low_singal_cycle| low_singal_cycle + 1)
            // All cycle lengths are prime => product gives cycle when all align
            .product()
    } else {
        let mut total_low = 0;
        let mut total_high = 0;

        for _ in 0..1000 {
            // One low for button press
            total_low += 1;
            for (signal, _, _) in
                network_state.iterate_button_press(&broadcast_ids, &module_outputs, &module_memory)
            {
                match signal {
                    Signal::Low => total_low += 1,
                    Signal::High => total_high += 1,
                }
            }
        }
        total_low * total_high
    }
}

pub fn solution(input: PuzzleInput, part2: bool) -> String {
    run(input, part2).to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run() {
        let test_input1 = r"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
        assert_eq!(run(test_input1.into(), false), 32000000);
        let test_input2 = r"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
        assert_eq!(run(test_input2.into(), false), 11687500);
        //assert_eq!(run(test_input.into(), true), 0);
    }
}
