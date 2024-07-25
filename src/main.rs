#[derive(Debug, Clone, Copy)]
enum PieceType {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy)]
struct BoardPos {
    row: u8,
    col: u8,
}

impl BoardPos {
    fn to_idx(&self) -> usize {
        return (self.col + self.row * 8).into()
    }

    fn from_idx(idx: usize) -> Option<Self> {
        if idx >= 64 { return None }
        return Some(BoardPos {
            row: (idx / 8).try_into().unwrap(),
            col: (idx % 8).try_into().unwrap()
        })
    }

    fn parse(string: &str) -> Option<BoardPos> {
        if string.len() == 2 {
            let col: u8 = string.chars().nth(0).unwrap() as u8;
            let row: u8 = string.chars().nth(1).unwrap() as u8;

            return match (col, row) {
                (b'a'..=b'h', b'1'..=b'8') => Some({
                    BoardPos {row: b'8' - row, col: col - b'a'}
                }),
                _ => None
            }
        }
        None
    }
}

#[derive(Debug, Clone, Copy)]
struct Piece {
    color: Color,
    piece: PieceType,
    pos: BoardPos,
}

impl Piece {
    fn to_char(&self) -> char {
        let ch = match self.piece {
            PieceType::Pawn => 'p',
            PieceType::Bishop => 'b',
            PieceType::Knight => 'n',
            PieceType::Rook => 'r',
            PieceType::Queen => 'q',
            PieceType::King => 'k',
        };
        match self.color {
            Color::White => ch.to_ascii_uppercase(),
            Color::Black => ch,
        } 
    }
    fn from_char(ch: char) -> Option<Self> {
        let color = match ch.is_uppercase() {
            true => Color::White,
            false => Color::Black,
        };

        let piece = match ch.to_ascii_lowercase() {
            'p' => PieceType::Pawn,
            'b' => PieceType::Bishop,
            'n' => PieceType::Knight,
            'r' => PieceType::Rook,
            'q' => PieceType::Queen,
            'k' => PieceType::King,
            _ => {return None;}
        };

        Some(Piece {piece, color, pos: BoardPos::from_idx(0).unwrap()})
    }
}

const NONE_PIECE: Option<Piece> = None;

#[derive(Debug, Clone)]
struct ChessBoard {
    pieces: [Option<Piece>; 64],
    turn: Color,
    winner: Option<Color>,
}

fn row_to_display(row: u8) -> u8 {
    8 - row
}

struct Move {
    from: BoardPos,
    to: BoardPos,
}

impl Move {
    fn parse(string: &str) -> Option<Self> {
        if string.len() == 4 {
            let from = BoardPos::parse(&string[0..2]);
            println!("from: {:#?}", from);
            let to = BoardPos::parse(&string[2..4]);
            println!("to: {:#?}", to);
            if from.is_some() && to.is_some() {
                let (from, to) = (from.unwrap(), to.unwrap());
                return Some(Move { from, to });
            }
            return None;
        } 
        None
    }
}

impl ChessBoard {
    fn new() -> Self {
        let mut pieces: Vec<Piece> = Vec::new();
        let mut board: ChessBoard = ChessBoard {pieces: [NONE_PIECE; 64], turn: Color::White, winner: None};
        //add pawns
        for col in 0..8 {
            //add white pawns
            pieces.push(Piece {color: Color::White, piece: PieceType::Pawn, pos: BoardPos {row: 6, col}});
            //add black pawns
            pieces.push(Piece {color: Color::Black, piece: PieceType::Pawn, pos: BoardPos {row: 1, col}});
        }

        for (col, piece) in "rnbqkbnr".chars().map(|c| Piece::from_char(c).unwrap().piece).enumerate() {
            let col = col.try_into().unwrap();
            //add white pawns
            pieces.push(Piece {color: Color::White, piece, pos: BoardPos {row: 7, col}});
            //add black pawns
            pieces.push(Piece {color: Color::Black, piece, pos: BoardPos {row: 0, col}});
        }

        for piece in pieces {
            board.pieces[piece.pos.to_idx()] = Some(piece);
        }
        return board;
    }

    fn print(&self) {
        println!("{}'s turn", match self.turn { Color::White => "White", Color::Black => "Black" });
        println!("   a  b  c  d  e  f  g  h");
        for (idx, piece) in self.pieces.iter().enumerate() {
            let pos = BoardPos::from_idx(idx).unwrap();
            if pos.col == 0 {
                print!("{} ", row_to_display(pos.row));
            }
            print!("[{}]", match piece {Some(p) => p.to_char(), None => ' '});
            if pos.col == 7 {
                print!(" {}\n", row_to_display(pos.row));
            }
        }
        println!("   a  b  c  d  e  f  g  h");
    }

    fn valid_moves(&self) -> Vec<Move> {
        let _pieces: Vec<Piece> = self.pieces.iter().filter(|p| p.is_some() && p.unwrap().color == self.turn).map(|p| p.unwrap()).collect();
        todo!();
    }

    fn execute(&mut self, mve: &Move) {
        let from_idx = mve.from.to_idx();
        let from_piece = self.pieces[from_idx];
        let to_idx = mve.to.to_idx();
        if from_piece.is_some() && from_piece.unwrap().color == self.turn {
            self.pieces[to_idx] = from_piece;
            self.pieces[from_idx] = None;
            self.turn = match self.turn {
                Color::White => Color::Black,
                Color::Black => Color::White
            };
        } else {
            println!("move not valid, wrong color piece (or no piece)");
        }
    }
}



fn main() {
    let mut board = ChessBoard::new(); 
    let mut input = String::new();
    while board.winner.is_none() {
        board.print();
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        input = input.as_str().trim().to_string();
        let player_move: Move = match Move::parse(&input) {
            Some(m) => m,
            None => { println!("invalid move"); continue; }
        };
        board.execute(&player_move);
    }
}
