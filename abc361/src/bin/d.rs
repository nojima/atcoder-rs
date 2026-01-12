use std::{collections::VecDeque, io::Read};

#[derive(Debug, Clone, Copy)]
struct State {
    field: [u8; 16],
    step: usize,
}

impl State {
    fn hash(&self, len: usize) -> usize {
        let mut b = 1;
        let mut ret = 0;
        for i in 0..len {
            let x = match self.field[i] {
                b'.' => 0,
                b'B' => 1,
                b'W' => 2,
                _ => unreachable!(),
            };
            ret += x * b;
            b *= 3;
        }
        ret
    }
}

#[allow(non_snake_case)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let mut words = input.split_ascii_whitespace();

    let N = words.next().unwrap().parse::<usize>()?;
    let L = N + 2;
    let S = words.next().unwrap().as_bytes().to_vec();
    let T = words.next().unwrap().as_bytes().to_vec();

    let mut queue = VecDeque::new();
    let mut initial_field = [b'.'; 16];
    initial_field[0..N].copy_from_slice(&S);
    queue.push_front(State {
        field: initial_field,
        step: 0,
    });
    let mut visited = vec![false; 3usize.pow(L as u32)];

    while let Some(state) = queue.pop_front() {
        if state.field[0..N] == T {
            println!("{}", state.step);
            return Ok(());
        }

        let field = &state.field[0..L];
        let (i_hole, _) = field.iter().enumerate().find(|(_, b)| **b == b'.').unwrap();
        for i in 0..(L-1) {
            // [i, i+1] と [i_hole, i_hole+1] が交わりを持たないことをチェック
            if i+1 < i_hole || i_hole+1 < i {
                let mut new_field = [b'.'; 16];
                new_field[0..L].copy_from_slice(field);
                new_field.swap(i, i_hole);
                new_field.swap(i+1, i_hole+1);

                let new_state = State {
                    field: new_field,
                    step: state.step + 1,
                };
                let h = new_state.hash(L);
                if visited[h] {
                    continue;
                }
                visited[h] = true;

                queue.push_back(new_state);
            }
        }
    }

    println!("-1");
    Ok(())
}
