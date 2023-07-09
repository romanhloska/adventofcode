use std::ops::Range;
use rand::Rng;

const BOARD_SIZE: i32 = 5;

fn main() {

    // generate boards
    let mut boards = generate_multiple_boards(3);

    loop {
        let next = generate_number(1..100);
        println!("next: {:?}", next);
        mark_items(next, &mut boards);

        let res = check_result(next, &boards);

        if res { break; }
    }
}

#[derive(Default, Debug)]
struct BoardItem {
    marked: bool,
    value: i32,
}

impl BoardItem {
    fn mark_item(&mut self) {
        self.marked = true
    }
}

#[derive(Default, Debug)]
struct Board (pub Vec<Vec<BoardItem>>);

fn mark_items(next: i32, boards: &mut Vec<Board>) {
    for board in boards {
        for items in &mut board.0 {
            for i in items {
                if i.value == next {
                    i.mark_item()
                }
            }
        }
    }
}

fn check_result(_next: i32, boards: &Vec<Board>) -> bool {
    println!("boards in fn: {:?}", boards);

    true
}

fn generate_number(r: Range<i32>) -> i32 {
    rand::thread_rng().gen_range(r)
}

fn generate_multiple_boards(n: i32) -> Vec<Board>{
    let mut boards: Vec<Board>  = Vec::new();
    for _count in 0..n {
        let b: Board = generate_board(1..100);
        boards.push(b);
    }

    boards
}

fn generate_board(_r: Range<i32>) -> Board {
    let mut board_row: Vec<Vec<BoardItem>>  = Vec::new();
    for _i in 0..BOARD_SIZE {
        let mut board_col: Vec<BoardItem> = Vec::new();
        for _j in 0..BOARD_SIZE {
            let gen_val = generate_number(1..100);
            let bi = build_item(false, gen_val);
            board_col.push(bi);
        }
        board_row.push(board_col);
    }

    let gen_board = Board(board_row);

    gen_board
}

fn build_item(marked: bool, value: i32) -> BoardItem {
    BoardItem {
        marked,
        value,
    }
}
