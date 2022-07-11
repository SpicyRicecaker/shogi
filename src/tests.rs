// use crate::{is_path_clear, Position};

// #[test]
// fn test_is_path_clear() {
//     let pieces = vec![&Position { x: 1, y: 1 }, &Position { x: 2, y: 2 }];
//     assert!(!is_path_clear(
//         &Position { x: 0, y: 0 },
//         &Position { x: 3, y: 3 },
//         &pieces
//     ));
//     assert!(is_path_clear(
//         &Position { x: 0, y: 0 },
//         &Position { x: 1, y: 1 },
//         &pieces
//     ));
//     assert!(is_path_clear(
//         &Position { x: 1, y: 1 },
//         &Position { x: 0, y: 0 },
//         &pieces
//     ));
// }

// #[test]
// fn test_is_path_clear_rook() {
//     let pieces = vec![];

//     let start = &Position { x: 1, y: 7 };
//     let end = &Position { x: 2, y: 7 };

//     assert!(is_path_clear(start, end, &pieces));
// }
