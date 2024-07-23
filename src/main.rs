#[derive(Debug, Clone, Copy)]
enum PieceType {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy)]
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
            PieceType::Knight => 'k',
            PieceType::Rook => 'r',
            PieceType::Queen => 'q',
            PieceType::King => 'k',
        };
        match self.color {
            Color::White => ch.to_ascii_uppercase(),
            Color::Black => ch,
        } 
    }
}

const NONE_PIECE: Option<Piece> = None;

struct ChessBoard {
    pieces: [Option<Piece>; 64],
    turn: Color,
}

impl ChessBoard {
    fn new() -> Self {
        let mut pieces: Vec<Piece> = Vec::new();
        let mut board: ChessBoard = ChessBoard {pieces: [NONE_PIECE; 64], turn: Color::White};
        //add pawns
        for col in 0..8 {
            //add white pawns
            pieces.push(Piece {color: Color::White, piece: PieceType::Pawn, pos: BoardPos {row: 6, col}});
            //add black pawns
            pieces.push(Piece {color: Color::Black, piece: PieceType::Pawn, pos: BoardPos {row: 1, col}});
        }

        for (col, &piece) in vec![PieceType::Rook, PieceType::Knight, PieceType::Bishop, PieceType::Queen, PieceType::King, PieceType::Bishop, PieceType::Knight, PieceType::Rook].iter().enumerate() {
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
        for (idx, piece) in self.pieces.iter().enumerate() {
            let pos = BoardPos::from_idx(idx).unwrap();
            print!("[{}]", match piece {Some(p) => p.to_char(), None => ' '});
            if pos.col == 7 {
                print!("\n");
            }
        }
    }
}

fn main() {
    let board = ChessBoard::new(); 
    board.print();
}
