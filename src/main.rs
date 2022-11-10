use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();

    let fen = setup(&mut rng);

    println!("{}", fen);
}

fn setup(rng: &mut impl Rng) -> String {
    let mut first_file = [None; 8];

    // dark square bishop
    let dark_square_bishop = rng.gen_range(0..4) * 2;

    first_file[dark_square_bishop] = Some(Piece {
        color: Color::White,
        piece_type: PieceType::Bishop,
    });

    // light square bishop
    let light_square_bishop = rng.gen_range(0..4) * 2 + 1;

    first_file[light_square_bishop] = Some(Piece {
        color: Color::White,
        piece_type: PieceType::Bishop,
    });

    // queen
    let mut queen_square = rng.gen_range(0..8);

    while first_file[queen_square].is_some() {
        queen_square = rng.gen_range(0..8);
    }

    first_file[queen_square] = Some(Piece {
        color: Color::White,
        piece_type: PieceType::Queen,
    });

    // first knight
    let mut first_knight_square = rng.gen_range(0..8);

    while first_file[first_knight_square].is_some() {
        first_knight_square = rng.gen_range(0..8);
    }

    first_file[first_knight_square] = Some(Piece {
        color: Color::White,
        piece_type: PieceType::Knight,
    });

    // second knight
    let mut second_knight_square = rng.gen_range(0..8);

    while first_file[second_knight_square].is_some() {
        second_knight_square = rng.gen_range(0..8);
    }

    first_file[second_knight_square] = Some(Piece {
        color: Color::White,
        piece_type: PieceType::Knight,
    });

    // first rook
    let mut first_rook_square = 0;

    while first_file[first_rook_square].is_some() {
        first_rook_square += 1;
    }

    first_file[first_rook_square] = Some(Piece {
        color: Color::White,
        piece_type: PieceType::Rook,
    });

    // second rook
    let mut second_rook_square = 7;

    while first_file[second_rook_square].is_some() {
        second_rook_square -= 1;
    }

    first_file[second_rook_square] = Some(Piece {
        color: Color::White,
        piece_type: PieceType::Rook,
    });

    // king
    let mut king_square = 0;

    while first_file[king_square].is_some() {
        king_square += 1;
    }

    first_file[king_square] = Some(Piece {
        color: Color::White,
        piece_type: PieceType::King,
    });

    let last_file = first_file.map(|piece| {
        Some(Piece {
            color: Color::Black,
            piece_type: piece.unwrap().piece_type,
        })
    });

    let mut fen = String::new();

    for piece in last_file {
        fen.push_str(&piece.unwrap().to_string());
    }

    fen.push('/');

    for _ in 0..8 {
        fen.push('p');
    }

    fen.push('/');

    for _ in 0..4 {
        fen.push_str("8/");
    }

    for _ in 0..8 {
        fen.push('P');
    }

    fen.push('/');

    for piece in first_file {
        fen.push_str(&piece.unwrap().to_string());
    }

    fen.push_str(" w KQkq - 0 1");

    fen
}

#[derive(Clone, Copy)]
struct Piece {
    color: Color,
    piece_type: PieceType,
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let piece_type = match self.piece_type {
            PieceType::Pawn => "p",
            PieceType::Knight => "n",
            PieceType::Bishop => "b",
            PieceType::Rook => "r",
            PieceType::Queen => "q",
            PieceType::King => "k",
        };

        match self.color {
            Color::White => write!(f, "{}", piece_type.to_uppercase()),
            Color::Black => write!(f, "{}", piece_type.to_lowercase()),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Color {
    White,
    Black,
}

#[derive(Clone, Copy)]
enum PieceType {
    #[allow(dead_code)]
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
