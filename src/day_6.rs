use std::collections::HashMap;

pub fn first_puzzle() -> String
{
    let (all_cycles, _) = cycles_count(BANKS);
    format!("{}", all_cycles)
}

pub fn second_puzzle() -> String
{
    let (_, interval_cycles) = cycles_count(BANKS);
    format!("{}", interval_cycles)
}

static BANKS: &[u32] = &[14, 0, 15, 12, 11, 11, 3, 5, 1, 6, 8, 4, 9, 1, 8, 4];

fn cycles_count(banks: &[u32]) -> (u32, u32)
{
    let mut cycles = 0;
    let mut state = Vec::from(banks);
    let mut known_states = HashMap::new();
    known_states.insert(state.clone(), cycles);

    loop
    {
        let max_idx = max_el_idx(&state);
        let free_blocks = state[max_idx];
        state[max_idx] = 0;

        for idx in (0 .. state.len()).cycle()
                                     .skip(max_idx + 1)
                                     .take(free_blocks as usize) 
        {
            state[idx] += 1;
        }

        cycles += 1;

        if known_states.contains_key(&state)
        {
            return (cycles, cycles - known_states.get(&state).unwrap())
        }
        else
        {
            known_states.insert(state.clone(), cycles);
        }
    }
}

fn max_el_idx(elems: &[u32]) -> usize
{
    let max_idx = (0 .. elems.len()).fold(0, |idx_max, idx| 
        {
            if elems[idx] > elems[idx_max] 
            {
                idx
            }
            else {
                idx_max
            }
        });

    max_idx
}

#[cfg(test)] 
mod tests 
{ 
    use super::*; 
 
    #[test] 
    fn both_puzzles() 
    { 
        assert_eq!(1, max_el_idx(&[0, 3, 0, 1, 3]));
        assert_eq!(3, max_el_idx(&[4, 3, 6, 8, 0]));
        assert_eq!((5, 4), cycles_count(&mut [0, 2, 7, 0]))
    } 
}