use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let width = parse_input!(inputs[0], i32);
    let height = parse_input!(inputs[1], i32);

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let start_row = parse_input!(inputs[0], i32);
    let start_col = parse_input!(inputs[1], i32);

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let num_maps = parse_input!(input_line, i32);

    let mut maps: Vec<Vec<Vec<u8>>> = Vec::new();
    for _ in 0..num_maps as usize {
        let mut single_map: Vec<Vec<u8>> = Vec::new();
        for _ in 0..height as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let map_row = input_line.trim_matches('\n').to_string();
            single_map.push(map_row.as_bytes().to_vec());
        }
        maps.push(single_map);
    }

    let mut path_len = Vec::<i32>::new();
    for map in maps {

        let mut current_row = start_row;
        let mut current_col = start_col;
        let mut length: i32 = 1;
        let mut first_move = true;
        loop {
            if current_row >= height || current_row < 0 || current_col >= width || current_col < 0
                || (!first_move && current_row == start_row && current_col == start_col) {
                length = -1;
                break;
            }

            first_move = false;

            match map[current_row as usize][current_col as usize] as char {
                '.' | '#' => {
                    length = -1;
                    break;
                },
                '^' => {
                    length += 1;
                    current_row -= 1;
                }
                'v' => {
                    length += 1;
                    current_row += 1;
                }
                '<' => {
                    length += 1;
                    current_col -= 1;
                }
                '>' => {
                    length += 1;
                    current_col += 1;
                }
                _ => { //T
                    break;
                }
            }

        }

        path_len.push(length);
    }

    let mut output = String::from("TRAP");
    let mut current = -1;
    for (i, len) in path_len.iter().enumerate() {
        if *len != -1 && (current == -1 || *len < current) {
            output = i.to_string();
            current = *len;
        }
    }

    println!("{output}")

}
