pub const SYMBOL_WIDTH: usize = 6;
pub const SYMBOL_HEIGHT: usize = 5;

pub fn symbol_to_render_data(ch: char) -> [[bool; SYMBOL_WIDTH]; SYMBOL_HEIGHT] {
    match ch {
        '1' => ONE,
        '2' => TWO,
        '3' => THREE,
        '4' => FOUR,
        '5' => FIVE,
        '6' => SIX,
        '7' => SEVEN,
        '8' => EIGHT,
        '9' => NINE,
        '0' => ZERO,
        ':' => DIV,
        '-' => DASH,
        ' ' => SPACE,
        'A' => A,
        'P' => P,
        'M' => M,
        _ => ERR,
    }
}

const O: bool = false;
const X: bool = true;

const ONE: [[bool; SYMBOL_WIDTH]; SYMBOL_HEIGHT] = [
    [O, O, O, O, X, X],
    [O, O, O, O, X, X],
    [O, O, O, O, X, X],
    [O, O, O, O, X, X],
    [O, O, O, O, X, X],
];

const TWO: [[bool; SYMBOL_WIDTH]; SYMBOL_HEIGHT] = [
    [X, X, X, X, X, X],
    [O, O, O, O, X, X],
    [X, X, X, X, X, X],
    [X, X, O, O, O, O],
    [X, X, X, X, X, X],
];

const THREE: [[bool; SYMBOL_WIDTH]; SYMBOL_HEIGHT] = [
    [X, X, X, X, X, X],
    [O, O, O, O, X, X],
    [X, X, X, X, X, X],
    [O, O, O, O, X, X],
    [X, X, X, X, X, X],
];

const FOUR: [[bool; SYMBOL_WIDTH]; SYMBOL_HEIGHT] = [
    [X, X, O, O, X, X],
    [X, X, O, O, X, X],
    [X, X, X, X, X, X],
    [O, O, O, O, X, X],
    [O, O, O, O, X, X],
];

const FIVE: [[bool; SYMBOL_WIDTH]; SYMBOL_HEIGHT] = [
    [X, X, X, X, X, X],
    [X, X, O, O, O, O],
    [X, X, X, X, X, X],
    [O, O, O, O, X, X],
    [X, X, X, X, X, X],
];

const SIX: [[bool; SYMBOL_WIDTH]; SYMBOL_HEIGHT] = [
    [X, X, X, X, X, X],
    [X, X, O, O, O, O],
    [X, X, X, X, X, X],
    [X, X, O, O, X, X],
    [X, X, X, X, X, X],
];

const SEVEN: [[bool; SYMBOL_WIDTH]; SYMBOL_HEIGHT] = [
    [X, X, X, X, X, X],
    [O, O, O, O, X, X],
    [O, O, O, O, X, X],
    [O, O, O, O, X, X],
    [O, O, O, O, X, X],
];

const EIGHT: [[bool; SYMBOL_WIDTH]; SYMBOL_HEIGHT] = [
    [X, X, X, X, X, X],
    [X, X, O, O, X, X],
    [X, X, X, X, X, X],
    [X, X, O, O, X, X],
    [X, X, X, X, X, X],
];

const NINE: [[bool; SYMBOL_WIDTH]; SYMBOL_HEIGHT] = [
    [X, X, X, X, X, X],
    [X, X, O, O, X, X],
    [X, X, X, X, X, X],
    [O, O, O, O, X, X],
    [X, X, X, X, X, X],
];

const ZERO: [[bool; SYMBOL_WIDTH]; SYMBOL_HEIGHT] = [
    [X, X, X, X, X, X],
    [X, X, O, O, X, X],
    [X, X, O, O, X, X],
    [X, X, O, O, X, X],
    [X, X, X, X, X, X],
];

const DIV: [[bool; SYMBOL_WIDTH]; SYMBOL_HEIGHT] = [
    [O, O, O, O, O, O],
    [O, O, X, X, O, O],
    [O, O, O, O, O, O],
    [O, O, X, X, O, O],
    [O, O, O, O, O, O],
];

const DASH: [[bool; SYMBOL_WIDTH]; SYMBOL_HEIGHT] = [
    [O, O, O, O, O, O],
    [O, O, O, O, O, O],
    [O, X, X, X, X, O],
    [O, O, O, O, O, O],
    [O, O, O, O, O, O],
];

const ERR: [[bool; SYMBOL_WIDTH]; SYMBOL_HEIGHT] = [
    [X, X, O, O, X, X],
    [O, X, X, X, X, O],
    [O, O, X, X, O, O],
    [O, X, X, X, X, O],
    [X, X, O, O, X, X],
];

const SPACE: [[bool; SYMBOL_WIDTH]; SYMBOL_HEIGHT] = [
    [O, O, O, O, O, O],
    [O, O, O, O, O, O],
    [O, O, O, O, O, O],
    [O, O, O, O, O, O],
    [O, O, O, O, O, O],
];

const A: [[bool; SYMBOL_WIDTH]; SYMBOL_HEIGHT] = [
    [X, X, X, X, X, X],
    [X, X, O, O, X, X],
    [X, X, X, X, X, X],
    [X, X, O, O, X, X],
    [X, X, O, O, X, X],
];

const P: [[bool; SYMBOL_WIDTH]; SYMBOL_HEIGHT] = [
    [X, X, X, X, X, X],
    [X, X, O, O, X, X],
    [X, X, X, X, X, X],
    [X, X, O, O, O, O],
    [X, X, O, O, O, O],
];

const M: [[bool; SYMBOL_WIDTH]; SYMBOL_HEIGHT] = [
    [X, X, X, X, X, X],
    [X, X, O, X, O, X],
    [X, X, O, X, O, X],
    [X, X, O, X, O, X],
    [X, X, O, X, O, X],
];
