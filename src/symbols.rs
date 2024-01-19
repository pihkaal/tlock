pub fn symbol_to_render_data(ch: char) -> [[bool; 6]; 5] {
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

const ONE: [[bool; 6]; 5] = [
    [O, O, O, O, X, X],
    [O, O, O, O, X, X],
    [O, O, O, O, X, X],
    [O, O, O, O, X, X],
    [O, O, O, O, X, X],
];

const TWO: [[bool; 6]; 5] = [
    [X, X, X, X, X, X],
    [O, O, O, O, X, X],
    [X, X, X, X, X, X],
    [X, X, O, O, O, O],
    [X, X, X, X, X, X],
];

const THREE: [[bool; 6]; 5] = [
    [X, X, X, X, X, X],
    [O, O, O, O, X, X],
    [X, X, X, X, X, X],
    [O, O, O, O, X, X],
    [X, X, X, X, X, X],
];

const FOUR: [[bool; 6]; 5] = [
    [X, X, O, O, X, X],
    [X, X, O, O, X, X],
    [X, X, X, X, X, X],
    [O, O, O, O, X, X],
    [O, O, O, O, X, X],
];

const FIVE: [[bool; 6]; 5] = [
    [X, X, X, X, X, X],
    [X, X, O, O, O, O],
    [X, X, X, X, X, X],
    [O, O, O, O, X, X],
    [X, X, X, X, X, X],
];

const SIX: [[bool; 6]; 5] = [
    [X, X, X, X, X, X],
    [X, X, O, O, O, O],
    [X, X, X, X, X, X],
    [X, X, O, O, X, X],
    [X, X, X, X, X, X],
];

const SEVEN: [[bool; 6]; 5] = [
    [X, X, X, X, X, X],
    [O, O, O, O, X, X],
    [O, O, O, O, X, X],
    [O, O, O, O, X, X],
    [O, O, O, O, X, X],
];

const EIGHT: [[bool; 6]; 5] = [
    [X, X, X, X, X, X],
    [X, X, O, O, X, X],
    [X, X, X, X, X, X],
    [X, X, O, O, X, X],
    [X, X, X, X, X, X],
];

const NINE: [[bool; 6]; 5] = [
    [X, X, X, X, X, X],
    [X, X, O, O, X, X],
    [X, X, X, X, X, X],
    [O, O, O, O, X, X],
    [X, X, X, X, X, X],
];

const ZERO: [[bool; 6]; 5] = [
    [X, X, X, X, X, X],
    [X, X, O, O, X, X],
    [X, X, O, O, X, X],
    [X, X, O, O, X, X],
    [X, X, X, X, X, X],
];

const DIV: [[bool; 6]; 5] = [
    [O, O, O, O, O, O],
    [O, O, X, X, O, O],
    [O, O, O, O, O, O],
    [O, O, X, X, O, O],
    [O, O, O, O, O, O],
];

const DASH: [[bool; 6]; 5] = [
    [O, O, O, O, O, O],
    [O, O, O, O, O, O],
    [O, X, X, X, X, O],
    [O, O, O, O, O, O],
    [O, O, O, O, O, O],
];

const ERR: [[bool; 6]; 5] = [
    [X, X, O, O, X, X],
    [O, X, X, X, X, O],
    [O, O, X, X, O, O],
    [O, X, X, X, X, O],
    [X, X, O, O, X, X],
];

const SPACE: [[bool; 6]; 5] = [
    [O, O, O, O, O, O],
    [O, O, O, O, O, O],
    [O, O, O, O, O, O],
    [O, O, O, O, O, O],
    [O, O, O, O, O, O],
];

const A: [[bool; 6]; 5] = [
    [X, X, X, X, X, X],
    [X, X, O, O, X, X],
    [X, X, X, X, X, X],
    [X, X, O, O, X, X],
    [X, X, O, O, X, X],
];

const P: [[bool; 6]; 5] = [
    [X, X, X, X, X, X],
    [X, X, O, O, X, X],
    [X, X, X, X, X, X],
    [X, X, O, O, O, O],
    [X, X, O, O, O, O],
];

const M: [[bool; 6]; 5] = [
    [X, X, X, X, X, X],
    [X, X, O, X, O, X],
    [X, X, O, X, O, X],
    [X, X, O, X, O, X],
    [X, X, O, X, O, X],
];
