use std::ops::Range;
use rand::Rng;

const BOARD_SIZE: i32 = 5;
const RANDOM_RANGE: std::ops::Range<u32> = std::ops::Range::<u32> { start: 0, end: 100 };

fn main() {

    // generate boards
    let mut boards = generate_multiple_boards(3);
    println!("generated boards: {:?}", boards);

    println!("generating numbers ... ");
    let mut count = 0;
    loop {
        let mut found = false;
        let next = generate_number(RANDOM_RANGE);
        count+=1;
        mark_items(next, &mut boards);

        for board in &boards {
            found = check_result(next, &board);
            if found {
                let sum = sum_unmarked_items(&board);
                let score = sum*next;

                println!("generated {} numbers.", count);

                println!("last number drawn: {:?}", next);
                println!("sum of unmarked numbers: {:?}", sum);
                println!("The final score is {:?}", score);

                break;
            }
        }

        if found { break; }
    }
}

#[derive(Default, Debug)]
struct BoardItem {
    marked: bool,
    value: u32,
}

impl BoardItem {
    fn mark_item(&mut self) {
        self.marked = true
    }
}

#[derive(Default, Debug)]
struct Board (pub Vec<Vec<BoardItem>>);

impl Board {
    fn check_col(&self, index: usize) ->  bool {
        let mut matched: bool = true;
        for i in 0..self.rows_len() {
            if (self.0.get(i).unwrap().get(index).unwrap()).marked == false {
                matched = false;
            }
        }

        matched
    }

    fn check_row(&self, index: usize) ->  bool {
        let mut matched: bool = true;
        for i in 0..self.cols_len() {
            if (self.0.get(index).unwrap().get(i).unwrap()).marked == false {
                matched = false;
            }
        }

        matched
    }

    fn rows_len(&self) -> usize {
        self.0.len()
    }
    fn cols_len(&self) -> usize {
        if self.0.is_empty() {
            return 0
        }

        self.0.get(0).unwrap().len()
    }
}

fn mark_items(next: u32, boards: &mut Vec<Board>) {
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

fn sum_unmarked_items(board: &Board) -> u32 {
    let mut sum = 0;
    for rows in &board.0 {
        for r in rows {
            if !r.marked {
                sum+=r.value;
            }
        }
    }

    sum
}

fn check_result(_next: u32, board: &Board) -> bool {
    if board.0.len() == 0 { return false }

    let mut found_column = false;
    let mut found_row = false;
    let mut _ind:i32 = -1;

    for col_index in 0..board.cols_len() {
        _ind = col_index as i32;
        if board.check_col(col_index) {
            found_column = true;
            println!("board found: {:?}", board);
            println!("column found: {}", _ind);
            break;
        }
    }

    if !found_column {
        for row_index in 0..board.rows_len() {
            _ind = row_index as i32;
            if board.check_row(row_index) {
                found_row = true;
                println!("board found: {:?}", board);
                println!("row found: {}", _ind);
                break;
            }
        }
    }

    found_column || found_row
}

fn generate_number(r: Range<u32>) -> u32 {
    rand::thread_rng().gen_range(r)
}

fn generate_multiple_boards(n: i32) -> Vec<Board>{
    let mut boards: Vec<Board>  = Vec::new();
    for _count in 0..n {
        let b: Board = generate_board(RANDOM_RANGE);
        boards.push(b);
    }

    boards
}

fn generate_board(_r: Range<u32>) -> Board {
    let mut board_row: Vec<Vec<BoardItem>>  = Vec::new();
    for _i in 0..BOARD_SIZE {
        let mut board_col: Vec<BoardItem> = Vec::new();
        for _j in 0..BOARD_SIZE {
            let gen_val = generate_number(RANDOM_RANGE);
            let bi = build_item(false, gen_val);
            board_col.push(bi);
        }
        board_row.push(board_col);
    }

    let gen_board = Board(board_row);

    gen_board
}

fn build_item(marked: bool, value: u32) -> BoardItem {
    BoardItem {
        marked,
        value,
    }
}
