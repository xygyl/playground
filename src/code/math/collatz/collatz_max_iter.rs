use inquire::{Confirm, CustomType};
use num_format::{Locale, ToFormattedString};
use std::collections::HashMap;

use crate::code::math::collatz::collatz::collatz_arg;

/// Single-threaded: DP + path compression + fused odd steps (odd = 2 steps).
pub fn collatz_max_iter() -> Option<()> {
    let n: u64 = CustomType::new("Enter n:")
        .with_help_message("The range of number to calculate (1..=input)")
        .prompt()
        .ok()?;

    let show = Confirm::new("Show collatz sequence of result?")
        .with_default(false)
        .prompt()
        .ok()?;

    if n == 0 {
        println!("\nNo numbers processed.");
        return None;
    }
    if n == 1 {
        println!(
            "\n{} took the most iterations: {}\n",
            1u64.to_formatted_string(&Locale::en),
            0u64.to_formatted_string(&Locale::en)
        );
        if show {
            collatz_arg(1);
        }
        return Some(());
    }

    // Dense cache for [1..=n]; index 0 unused. Stores total steps to reach 1.
    let mut in_range: Vec<Option<u64>> = vec![None; (n as usize) + 1];
    in_range[1] = Some(0);

    // Reused buffers to avoid allocations in the loop.
    let mut nodes: Vec<u128> = Vec::with_capacity(128);
    let mut costs: Vec<u64> = Vec::with_capacity(128);
    let mut overflow: HashMap<u128, u64> = HashMap::new();

    #[inline]
    fn collatz_next_fused(x: u128) -> (u128, u64) {
        if x & 1 == 0 {
            (x >> 1, 1) // even: one step
        } else {
            (((x * 3) + 1) >> 1, 2) // odd fused: two steps
        }
    }

    let mut best_num = 1u64;
    let mut best_len = 0u64;

    for start in 2..=n {
        nodes.clear();
        costs.clear();
        overflow.clear();

        let mut x = start as u128;

        // Walk until we hit a cached value (either in-range or in the tiny overflow map).
        let base_len: u64 = loop {
            if x <= n as u128 {
                if let Some(len) = in_range[x as usize] {
                    break len;
                }
            } else if let Some(len) = overflow.get(&x) {
                break *len;
            }

            nodes.push(x);
            let (nx, step_cost) = collatz_next_fused(x);
            costs.push(step_cost);
            x = nx;
        };

        // Back-fill exact totals along the path: total_len(node_i) = base_len + sum(costs[i..])
        let mut acc = base_len;
        for i in (0..nodes.len()).rev() {
            acc = acc.saturating_add(costs[i]);
            let v = nodes[i];
            if v <= n as u128 {
                let slot = &mut in_range[v as usize];
                if slot.is_none() {
                    *slot = Some(acc);
                }
            } else {
                overflow.entry(v).or_insert(acc);
            }
        }

        // We just computed start's length during back-fill; unwrap is safe.
        let len_start = in_range[start as usize].unwrap();
        if len_start > best_len {
            best_len = len_start;
            best_num = start;
        }
    }

    println!(
        "\n{} took the most iterations: {}\n",
        best_num.to_formatted_string(&Locale::en),
        best_len.to_formatted_string(&Locale::en)
    );

    if show {
        collatz_arg(best_num as u128);
    }
    Some(())
}
