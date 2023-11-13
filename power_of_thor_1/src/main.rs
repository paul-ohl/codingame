use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 * ---
 * Hint: You can use the debug stream to print initialTX and initialTY, if Thor seems not follow your orders.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let light_x = parse_input!(inputs[0], i32); // the X position of the light of power
    let light_y = parse_input!(inputs[1], i32); // the Y position of the light of power
    let mut tx = parse_input!(inputs[2], i32); // Thor's starting X position
    let mut ty = parse_input!(inputs[3], i32); // Thor's starting Y position

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let _remaining_turns = parse_input!(input_line, i32); // The remaining amount of turns Thor can move.

        let mut output = String::new();
        if light_y > ty {
            output.push('S');
            ty += 1;
        } else if light_y < ty {
            output.push('N');
            ty -= 1;
        }
        if light_x > tx {
            output.push('E');
            tx += 1;
        } else if light_x < tx {
            output.push('W');
            tx -= 1;
        }

        // A single line providing the move to be made: N NE E SE S SW W or NW
        println!("{}", output);
    }
}
