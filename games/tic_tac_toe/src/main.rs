// use std::io;
// use tic_tac_toe::*;

fn main() {
    tic_tac_toe::new()
    .run();
}

// fn main() {
//     println!("Tic Tac Toe with MCTS");
//     println!("You are O and the computer is X.");
//     println!("Cells are numbered 1 through 9 as follows:");
//     println!(" 1 | 2 | 3 ");
//     println!("-----------");
//     println!(" 4 | 5 | 6 ");
//     println!("-----------");
//     println!(" 7 | 8 | 9 ");
//     println!();

//     let mut board = TicTacToeBoard::default();
//     let computer = Player::X;

//     board.print();

//     while !board.is_terminal() {
//         if board.player == computer {
//             println!("Computer's turn...");
//             // Run MCTS for a fixed number of iterations (adjust as desired).
//             let m = mcts_search(board.clone(), 1000);
//             board.make_move(m);
//         } else {
//             println!("Your turn. Enter a move (1-9): ");
//             let mut input = String::new();
//             io::stdin()
//                 .read_line(&mut input)
//                 .expect("Failed to read input");
//             let trimmed = input.trim();
//             let pos = match trimmed.parse::<usize>() {
//                 Ok(num) if num >= 1 && num <= 9 => num - 1,
//                 _ => {
//                     println!("Please enter a valid number between 1 and 9.");
//                     continue;
//                 }
//             };
//             if !board.make_move(pos) {
//                 println!("That cell is already occupied. Try again.");
//                 continue;
//             }
//         }
//         board.print();
//     }

//     if let Some(winner) = board.check_winner() {
//         match winner {
//             Player::X => println!("Computer wins!"),
//             Player::O => println!("You win!"),
//         }
//     } else {
//         println!("It's a draw!");
//     }
// }