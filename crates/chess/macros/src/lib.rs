use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    token::Underscore,
    Error, Ident, Result,
};

fn namespace(span: proc_macro2::Span) -> Ident {
    let crate_name = std::env::var("CARGO_PKG_NAME").unwrap();
    if crate_name == "chess" {
        Ident::new("crate", span)
    } else {
        Ident::new("chess", span)
    }
}

struct Board(TokenStream);

impl Parse for Board {
    fn parse(input: ParseStream) -> Result<Self> {
        let ns = namespace(input.span());
        let mut pieces: Vec<proc_macro2::TokenStream> = vec![];
        while pieces.len() < 64 {
            let lookahead = input.lookahead1();
            let col = (pieces.len() % 8) as u8;
            let row = (pieces.len() / 8) as u8;
            if lookahead.peek(Ident) {
                let span = input.span();
                let ident: Ident = input.parse()?;
                match ident.to_string().as_str() {
                    "p" => {
                        pieces.push(quote! {
                            board.set_piece(
                                #ns::Position(#col, #row),
                                Some(#ns::Piece::new(#ns::PieceKind::Pawn, #ns::PieceColor::White)),
                            );
                        });
                    }
                    "r" => {
                        pieces.push(quote! {
                            board.set_piece(
                                #ns::Position(#col, #row),
                                Some(#ns::Piece::new(#ns::PieceKind::Rook, #ns::PieceColor::White)),
                            );
                        });
                    }
                    "n" => {
                        pieces.push(quote! {
                            board.set_piece(
                                #ns::Position(#col, #row),
                                Some(#ns::Piece::new(#ns::PieceKind::Knight, #ns::PieceColor::White)),
                            );
                        });
                    }
                    "b" => {
                        pieces.push(quote! {
                            board.set_piece(
                                #ns::Position(#col, #row),
                                Some(#ns::Piece::new(#ns::PieceKind::Bishop, #ns::PieceColor::White)),
                            );
                        });
                    }
                    "q" => {
                        pieces.push(quote! {
                            board.set_piece(
                                #ns::Position(#col, #row),
                                Some(#ns::Piece::new(#ns::PieceKind::Queen, #ns::PieceColor::White)),
                            );
                        });
                    }
                    "k" => {
                        pieces.push(quote! {
                            board.set_piece(
                                #ns::Position(#col, #row),
                                Some(#ns::Piece::new(#ns::PieceKind::King, #ns::PieceColor::White)),
                            );
                        });
                    }
                    "P" => {
                        pieces.push(quote! {
                            board.set_piece(
                                #ns::Position(#col, #row),
                                Some(#ns::Piece::new(#ns::PieceKind::Pawn, #ns::PieceColor::Black)),
                            );
                        });
                    }
                    "R" => {
                        pieces.push(quote! {
                            board.set_piece(
                                #ns::Position(#col, #row),
                                Some(#ns::Piece::new(#ns::PieceKind::Rook, #ns::PieceColor::Black)),
                            );
                        });
                    }
                    "N" => {
                        pieces.push(quote! {
                            board.set_piece(
                                #ns::Position(#col, #row),
                                Some(#ns::Piece::new(#ns::PieceKind::Knight, #ns::PieceColor::Black)),
                            );
                        });
                    }
                    "B" => {
                        pieces.push(quote! {
                            board.set_piece(
                                #ns::Position(#col, #row),
                                Some(#ns::Piece::new(#ns::PieceKind::Bishop, #ns::PieceColor::Black)),
                            );
                        });
                    }
                    "Q" => {
                        pieces.push(quote! {
                            board.set_piece(
                                #ns::Position(#col, #row),
                                Some(#ns::Piece::new(#ns::PieceKind::Queen, #ns::PieceColor::Black)),
                            );
                        });
                    }
                    "K" => {
                        pieces.push(quote! {
                            board.set_piece(
                                #ns::Position(#col, #row),
                                Some(#ns::Piece::new(#ns::PieceKind::King, #ns::PieceColor::Black)),
                            );
                        });
                    }
                    _ => {
                        return Err(Error::new(span, "invalid chess board character"));
                    }
                }
            } else if lookahead.peek(Underscore) {
                let _: Underscore = input.parse()?;
                pieces.push(quote! {
                    board.set_piece(#ns::Position(#col, #row), None);
                });
            } else {
                return Err(lookahead.error());
            }
        }
        Ok(Board(TokenStream::from(quote! {
            {
                let mut board = #ns::Board::new();
                #(#pieces)*
                board
            }
        })))
    }
}

#[proc_macro]
pub fn board(tokens: TokenStream) -> TokenStream {
    let board = parse_macro_input!(tokens as Board);
    board.0
}

struct BoardMove(TokenStream);

const COLS: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
const ROWS: [char; 8] = ['8', '7', '6', '5', '4', '3', '2', '1'];
fn parse_square(input: ParseStream) -> Result<(u8, u8)> {
    let span = input.span();
    let ident: Ident = input.parse()?;
    let name = ident.to_string().to_ascii_lowercase();
    if name.len() != 2 {
        return Err(Error::new(span, "invalid chess move"));
    }
    let chars: Vec<char> = name.chars().collect();
    let col = COLS
        .iter()
        .position(|c| *c == chars[0])
        .expect("invalid chess move");
    let row = ROWS
        .iter()
        .position(|c| *c == chars[1])
        .expect("invalid chess move");
    Ok((col as u8, row as u8))
}

impl Parse for BoardMove {
    fn parse(input: ParseStream) -> Result<Self> {
        let ns = namespace(input.span());
        let (from_col, from_row) = parse_square(input)?;
        let (to_col, to_row) = parse_square(input)?;
        Ok(BoardMove(TokenStream::from(quote! {
            #ns::BoardMove {
                from: #ns::Position(#from_col, #from_row),
                to: #ns::Position(#to_col, #to_row),
            }
        })))
    }
}

#[proc_macro]
pub fn board_move(tokens: TokenStream) -> TokenStream {
    let board_move = parse_macro_input!(tokens as BoardMove);
    board_move.0
}
