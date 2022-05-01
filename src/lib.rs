// the biggest thing is probably using a sprite to represent a square instead of
// drawing all the gridlines (which leads to bugs during selection anyway)

// instead of iterating over a bunch of squares to find a valid move, better
// would be to iterate over all of the squares and then validate those square
// moves

// the problem then, arises from when you're trying to validate a rook move. O, then I guess you can just check if there are any pieces in between, and if there are, just ignore the moves.

// // now we also have to update the captured pieces of both player one and two
// fn generateCapturesFrom(board: Vec<Pieces>) {
//     // let captures = {
//     //     [Player.ONE] = {
//     //         Pawn = {},
//     //         Lance = {},
//     //         Knight = {},
//     //         Silver = {},
//     //         Gold = {},
//     //         Bishop = {},
//     //         Rook = {}
//     //     },
//     //     [Player.TWO] = {
//     //         Pawn = {},
//     //         Lance = {},
//     //         Knight = {},
//     //         Silver = {},
//     //         Gold = {},
//     //         Bishop = {},
//     //         Rook = {}
//     //     }
//     // }
//     for piece in board {
//         if piece.X == 0 && piece.Y == 0 {
//             // put it in the extras of the corresponding opponent
//             table.insert(captures[piece.O][piece.Name], piece)
//         }

//     }
//     return captures
// }

// // fn buildRegistryFrom(board)
// //     local registry = {}
// //     // TODO needs to be rebuilt every time a capture occurs lol
// //     for _, piece in ipairs(board){
// //         registry[piece.Id] = piece
// //     }
// //     return registry
// // }

// // Global register for pieces at any location
// // hashmap, stores existing references to pieces at their coordinates
// // Must be updated (via recomputation) every time someone moves
// fn buildGetPieceAtFrom(board)
//     local getPieceAt = {}
//     for _, piece in ipairs(board){
//         if not getPieceAt[piece.X] then
//             getPieceAt[piece.X] = {}
//         }
//         getPieceAt[piece.X][piece.Y] = piece;
//     }
//     return getPieceAt
// }

// // start the central game loop

// enum Player {
//     One,
//     Two
// }

// enum Rank {
//     Normal,
//     Promoted
// }

// struct Piece {
//     name: String,
//     x: usize,
//     y: usize,
//     owner: Player,
//     rank: Rank
// }

// // fn swapPlayer(player)
// //     return player * -1
// // }

// // tf is tween service?
// // local tween = game:GetService("TweenService")

// fn getNewBoard() {
//     // local pieces = {};
//     // each piece is a class that holds 5 variables, K, X, Y, P, and O
//     // O is for ownership
//     // P is for ???
//     // X and Y are the coordinates of the piece
//     local pieces = { {
//         Name = "King",
//         X = 5,
//         Y = 1,
//         O = Player.ONE
//     },
//     {
//         Name = "King",
//         X = 5,
//         Y = 9,
//         O = Player.TWO
//     },
//     {
//         Name = "Gold",
//         X = 4,
//         Y = 1,
//         O = Player.ONE
//     },
//     {
//         Name = "Gold",
//         X = 6,
//         Y = 1,
//         O = Player.ONE
//     },
//     {
//         Name = "Silver",
//         X = 3,
//         Y = 1,
//         O = Player.ONE
//     },
//     {
//         Name = "Silver",
//         X = 7,
//         Y = 1,
//         O = Player.ONE
//     },
//     {
//         Name = "Knight",
//         X = 2,
//         Y = 1,
//         O = Player.ONE
//     },
//     {
//         Name = "Knight",
//         X = 8,
//         Y = 1,
//         O = Player.ONE
//     },
//     {
//         Name = "Lance",
//         X = 1,
//         Y = 1,
//         O = Player.ONE
//     },
//     {
//         Name = "Lance",
//         X = 9,
//         Y = 1,
//         O = Player.ONE
//     },
//     {
//         Name = "Rook",
//         X = 8,
//         Y = 2,
//         O = Player.ONE
//     },
//     {
//         Name = "Bishop",
//         X = 2,
//         Y = 2,
//         O = Player.ONE
//     },
//     {
//         Name = "Gold",
//         X = 4,
//         Y = 9,
//         O = Player.TWO
//     },
//     {
//         Name = "Gold",
//         X = 6,
//         Y = 9,
//         O = Player.TWO
//     },
//     {
//         Name = "Silver",
//         X = 3,
//         Y = 9,
//         O = Player.TWO
//     },
//     {
//         Name = "Silver",
//         X = 7,
//         Y = 9,
//         O = Player.TWO
//     },
//     {
//         Name = "Knight",
//         X = 2,
//         Y = 9,
//         O = Player.TWO
//     },
//     {
//         Name = "Knight",
//         X = 8,
//         Y = 9,
//         O = Player.TWO
//     },
//     {
//         Name = "Lance",
//         X = 1,
//         Y = 9,
//         O = Player.TWO
//     },
//     {
//         Name = "Lance",
//         X = 9,
//         Y = 9,
//         O = Player.TWO
//     },
//     {
//         Name = "Rook",
//         X = 2,
//         Y = 8,
//         O = Player.TWO
//     },
//     {
//         Name = "Bishop",
//         X = 8,
//         Y = 8,
//         O = Player.TWO
//     },

//     }

//     for i = 1, 9{
//         // "pawn1", "pawn2", etc.
//         table.insert(pieces, {
//             Name = "Pawn",
//             X = i,
//             Y = 3,
//             O = Player.ONE
//         });
//     }

//     for i = 1, 9{
//         // "pawn1", "pawn2", etc.
//         table.insert(pieces, {
//             Name = "Pawn",
//             X = i,
//             Y = 7,
//             O = Player.TWO
//         });
//     }

//     for i, piece in ipairs(pieces){
//         piece.P = Rank.NORMAL;
//         piece.Id = i
//     }

//     return pieces
// }

// // fn queryPiece(queryPiece, pieceList)
// // 	for piece in pieceList{
// // 		// comparing equality of objects
// // 		if table.concat(piece) == table.concat(queryPiece) then
// // 			return piece
// // 		}
// // 	}
// // 	return false
// // }

// // sets up pieces for both players
// fn setupPieces()
//     // seems to be a 3d coordinate of some sort
//     // local origin = CFrame.new(board.Position)

//     // an array of all the pieces on the board
//     // except we're using a dictionary
//     Pieces = getNewBoard()
//     // create a registry for pieces, returns the index of the array
//     // GlobalPieceRegistry = buildRegistryFrom(Pieces)

//     // variable that stores if the game is not over
//     // local running = true

//     // local player1 = nil
//     // board.Pillow1.BillboardGui.TextLabel.Text = "Empty"
//     // local bot1 = false
//     // board.Bot1.Color = Color3.new(1, 0, 0)
//     // board.Bot1.SurfaceGui.TextLabel.Text = "Bot: Off"

//     // local player2 = nil
//     // board.Pillow2.BillboardGui.TextLabel.Text = "Empty"
//     // local bot2 = false
//     // board.Bot2.Color = Color3.new(1, 0, 0)
//     // board.Bot2.SurfaceGui.TextLabel.Text = "Bot: Off"

//     // local turn = Player.ONE

//     // board.BillboardGui.TextLabel.Text = "Player 1's Turn"
//     // board.Selected.Position = Vector3.new(0, -3.2, 17.5) + origin.p

//     // if bot is clicked, light up with an indicator and show that bot is on,
//     // for both players

//     // checks if there is a piece at the current location, given a location (x,
//     // y) and an array of pieces
//     // fn piece_exists_at(location, hashmap)
//     // 	return hashmap[location.Y][location.X]
//     // }

//     // what is delay?
//     // delay(0, function()
//     // seems to be the core gameloop

//     // })

//     // game.ReplicatedStorage.RemoteEvent.OnServerEvent:Connect(function(plr, piece, x, y, promote)
//     // 	if running == true then return }
//     // 	if plr ~= player1 and turn == -1 then return }
//     // 	if plr ~= player2 and turn == 1 then return }
//     // 	movepiece(plr, piece, x, y, promote)
//     // })

// }

// // local chil = game.Workspace.Boards:GetChildren()
// // for i = 1, #chil{
// // 	delay(0, function()
// // 		while true{
// // setup(chil[i])
// // 		}
// // 	})
// // }

// // probably find all places we can move
// fn findavailable(selectedPiece, getPieceAt)
//     local pieceName = selectedPiece.Name

//     local available = {}

//     // // build a cache
//     // local getPieceAt = {}
//     // for _, piece in ipairs(Pieces){
//     //     if not getPieceAt[piece.X] then
//     //         getPieceAt[piece.X] = {}
//     //     }
//     //     getPieceAt[piece.X][piece.Y] = piece;
//     // }

//     // potential bug: if it's pawn then there's literally no available squares
//     // if we've clicked outside of the arena?
//     if selectedPiece.X == 0 or selectedPiece.Y == 0 then
//         // if we're placing a pawn somewhere on the board, check if there is a pawn in that column already
//         for h = 1, 9{
//             local pawn = false
//             if pieceName == "Pawn" then
//                 for v = 1, 9{
//                     // tab is all coordinates of the board
//                     // if there is a spot on the table for this location
//                     local spot = getPieceAt[h][v]
//                     if spot then
//                         if spot.O == selectedPiece.O and spot.Name == "Pawn" and spot.P == 1 then
//                             pawn = true
//                             break
//                         }
//                     }
//                 }
//             }
//             // if it's not a pawn
//             if pawn == false then
//                 for v = 1, 9{
//                     // make every spot available

//                     local spot = getPieceAt[h][v];

//                     // pieces can only be sent to where they can move one more time (so they can promote)

//                     // we can only s} knight up to third to last rank
//                     // we can only s} pawn and lace up to second to last rank

//                     // all other pieces can be sent to last row

//                     if not spot then
//                         if pieceName == "Pawn" or pieceName == "Lance" or pieceName == "Knight" then
//                             if selectedPiece.O * 4 + 5 ~= v then
//                                 table.insert(available, { X = h, Y = v })
//                             elseif selectedPiece.O * 3 + 5 ~= v and pieceName == "Knight" then
//                                 table.insert(available, { X = h, Y = v })
//                             }
//                         else
//                             table.insert(available, { X = h, Y = v })
//                         }
//                     }
//                 }
//             }
//         }
//     else
//         local dydxList = {}
//         // for ALL pieces, we generate a list of dydx
//         // we iterate over the dydx, and only then{ we actually{ boundary checking

//         // the problem currently is the player side. We have to treat each
//         // dy as negative or positive dep}ing on the side the opponent is
//         // on

//         // if it's pawn promote to gold
//         if pieceName == "Pawn" then
//             if selectedPiece.P == Rank.NORMAL then
//                 table.insert(dydxList, { 1, 0 })
//             else
//                 for y = -1, 1{
//                     for x = -1, 1{
//                         if not (y == -1 and x == -1) and not (y == -1 and x == 1) then
//                             table.insert(dydxList, { y, x })
//                         }
//                     }
//                 }
//             }
//             // for lance, dynamically generate a list of dydx at runtime
//         elseif pieceName == "Lance" then
//             if selectedPiece.P == Rank.NORMAL then
//                 local newX = selectedPiece.X
//                 local newY = selectedPiece.Y + selectedPiece.O
//                 // get position of rook in this direction
//                 while newX >= 1 and newX <= 9 and newY >= 1 and newY <= 9{
//                     local spot = getPieceAt[newX][newY]

//                     // if there is a piece in the way, our rook cannot move further
//                     if spot then
//                         // check ownership
//                         // if we own,{ not include the spot as a valid move
//                         if spot.O ~= selectedPiece.O then
//                             // otherwise, add it as a valid move directly into the insert
//                             table.insert(available, {
//                                 X = newX,
//                                 Y = newY
//                             })
//                         }
//                         // break out
//                         break
//                     else
//                         table.insert(available, {
//                             X = newX,
//                             Y = newY
//                         })
//                     }

//                     newY = newY + selectedPiece.O
//                 }
//             else
//                 // gold movement
//                 for y = -1, 1{
//                     for x = -1, 1{
//                         if not (y == -1 and x == -1) and not (y == -1 and x == 1) then
//                             table.insert(dydxList, { y, x })
//                         }
//                     }
//                 }
//             }
//             // if it's knight promote to gold
//         elseif pieceName == "Knight" then
//             if selectedPiece.P == Rank.NORMAL then
//                 for _, x in ipairs({ -1, 1 }){
//                     table.insert(dydxList, { 2, x })
//                 }
//             else
//                 for y = -1, 1{
//                     for x = -1, 1{
//                         if not (y == -1 and x == -1) and not (y == -1 and x == 1) then
//                             table.insert(dydxList, { y, x })
//                         }
//                     }
//                 }
//             }
//             // if it's silver promote to gold
//         elseif pieceName == "Silver" then
//             if selectedPiece.P == Rank.NORMAL then
//                 for _, coordinate in ipairs({
//                     {
//                         1, -1
//                     }, {
//                         1, 0
//                     }, {
//                         1, 1
//                     },
//                     {
//                         -1, -1
//                     },
//                     {
//                         -1, 1
//                     }
//                 }){
//                     table.insert(dydxList, { coordinate[1], coordinate[2] })
//                 }
//             else
//                 for y = -1, 1{
//                     for x = -1, 1{
//                         if not (y == -1 and x == -1) and not (y == -1 and x == 1) then
//                             table.insert(dydxList, { y, x })
//                         }
//                     }
//                 }
//             }
//             // if it's gold promote to gold
//         elseif pieceName == "Gold" then
//             for y = -1, 1{
//                 for x = -1, 1{
//                     if not (y == -1 and x == -1) and not (y == -1 and x == 1) then
//                         table.insert(dydxList, { y, x })
//                     }
//                 }
//             }
//             // if it's gold promote to gold
//         elseif pieceName == "King" then
//             for y = -1, 1{
//                 for x = -1, 1{
//                     table.insert(dydxList, { y, x })
//                 }
//             }
//         elseif pieceName == "Rook" then
//             // the rook can go in four directions
//             local directionsDydx = {
//                 { 1, 0 },
//                 { -1, 0 },
//                 { 0, 1 },
//                 { 0, -1 }
//             }

//             // for each direction
//             for _, dydx in ipairs(directionsDydx){
//                 local dy = dydx[1]
//                 local dx = dydx[2]

//                 local newX = selectedPiece.X + dx
//                 local newY = selectedPiece.Y + dy
//                 // get position of rook in this direction
//                 while newX >= 1 and newX <= 9 and newY >= 1 and newY <= 9{
//                     local spot = getPieceAt[newX][newY]

//                     // if there is a piece in the way, our rook cannot move further
//                     if spot then
//                         // check ownership
//                         // if we own,{ not include the spot as a valid move
//                         if spot.O ~= selectedPiece.O then
//                             // otherwise, add it as a valid move directly into the insert
//                             table.insert(available, {
//                                 X = newX,
//                                 Y = newY
//                             })
//                         }
//                         // break out
//                         break
//                     else
//                         table.insert(available, {
//                             X = newX,
//                             Y = newY
//                         })
//                     }

//                     newX = newX + dx
//                     newY = newY + dy
//                 }
//             }

//             if selectedPiece.P == Rank.PROMOTED then
//                 for _, coordinate in ipairs({
//                     {
//                         1, 1
//                     }, {
//                         1, -1
//                     }, {
//                         -1, -1
//                     },
//                     {
//                         -1, 1
//                     }
//                 }){
//                     table.insert(dydxList, { coordinate[1], coordinate[2] })
//                 }
//             }
//         elseif pieceName == "Bishop" then
//             // the bishop can go in four directions
//             local directionsDydx = {
//                 { 1, 1 },
//                 { 1, -1 },
//                 { -1, 1 },
//                 { -1, -1 }
//             }

//             // for each direction
//             for _, dydx in ipairs(directionsDydx){
//                 local dy = dydx[1]
//                 local dx = dydx[2]

//                 local newX = selectedPiece.X + dx
//                 local newY = selectedPiece.Y + dy
//                 // get position of bishop in this direction
//                 while newX >= 1 and newX <= 9 and newY >= 1 and newY <= 9{
//                     local spot = getPieceAt[newX][newY]

//                     // if there is a piece in the way, our bishop cannot move further
//                     if spot then
//                         // check ownership
//                         // if we own,{ not include the spot as a valid move
//                         if spot.O ~= selectedPiece.O then
//                             // otherwise, add it as a valid move directly into the insert
//                             table.insert(available, {
//                                 X = newX,
//                                 Y = newY
//                             })
//                         }
//                         // break out
//                         break
//                     else
//                         table.insert(available, {
//                             X = newX,
//                             Y = newY
//                         })
//                     }

//                     newX = newX + dx
//                     newY = newY + dy
//                 }
//             }

//             if selectedPiece.P == Rank.PROMOTED then
//                 for _, coordinate in ipairs({
//                     {
//                         1, 0
//                     }, {
//                         -1, 0
//                     }, {
//                         0, -1
//                     },
//                     {
//                         0, 1
//                     }
//                 }){
//                     table.insert(dydxList, { coordinate[1], coordinate[2] })
//                 }
//             }
//         }

//         for _, dydx in ipairs(dydxList){
//             local dy = dydx[1]
//             local dx = dydx[2]

//             local newY = selectedPiece.Y + dy * selectedPiece.O
//             local newX = selectedPiece.X + dx

//             if newY >= 1 and newY <= 9 and newX >= 1 and newX <= 9 then
//                 local spot = getPieceAt[newX][newY]
//                 if spot then
//                     if spot.O ~= selectedPiece.O then
//                         table.insert(available, {
//                             X = newX,
//                             Y = newY
//                         })
//                     }
//                 else
//                     table.insert(available, {
//                         X = newX,
//                         Y = newY
//                     })
//                 }
//             }
//         }

//     }
//     return available
// }

// // function that takes the name of a piece and a table, returns if it checks
// // the king?
// // how are we getting the name in piece ?
// fn check(selectedPiece, newBoard, getPieceAt)
//     // for every piece in this current board
//     for _, piece in ipairs(newBoard){
//         // for every piece that{es not have the same owner
//         if selectedPiece.O == piece.O * -1 then
//             for _, available in ipairs(findavailable(piece, getPieceAt)){
//                 if available.X == selectedPiece.X and available.Y == selectedPiece.Y then
//                     return true
//                     // check = true
//                 }
//             }
//         }
//     }
//     // return check
//     return false
// }

// // Process the available moves, only allow those in which the king is not in
// // we basically test every single available move, by making the move, then
// // check if the king is still checked after that move. If the king is
// // still checked, it's not a valid move
// // if there are no valid moves, it's GG, checkmate
// fn moves(selectedPiece)
//     // local pieces = newBoard or Pieces

//     local newAvailableSquares = {}

//     // First we want to select the selectedPiece

//     // for each available square, create a new board with that piece moved
//     for _, availableSquare in ipairs(findavailable(selectedPiece, GetPieceAt)){

//         // clone the current table
//         local tempBoardWithSelectedPieceMoved = {}
//         for i, piece in ipairs(Pieces){
//             tempBoardWithSelectedPieceMoved[i] = {
//                 Name = piece.Name,
//                 X = piece.X,
//                 Y = piece.Y,
//                 O = piece.O,
//                 P = piece.P
//             }
//         }

//         // print(dump(tempBoardWithSelectedPieceMoved))
//         // print(selectedPieceIdx)

//         // we also want to delete the piece that we're moving to, if there's actually something there
//         // this variable isn't mutated
//         local availablePieceBeingOverwritten = GetPieceAt[availableSquare.X][availableSquare.Y]

//         // captured pieces in shogi go to the opponent, in which they can play anywhere
//         if availablePieceBeingOverwritten and availablePieceBeingOverwritten.O ~= selectedPiece.O then
//             local t = tempBoardWithSelectedPieceMoved[availablePieceBeingOverwritten.Id]

//             t.X = 0
//             t.Y = 0
//             t.P = 1
//             t.O = selectedPiece.O
//         }

//         // move the piece in question to its new spot, in the new table
//         local movedPiece = tempBoardWithSelectedPieceMoved[selectedPiece.Id]
//         movedPiece.X = availableSquare.X
//         movedPiece.Y = availableSquare.Y

//         local kingPiece = tempBoardWithSelectedPieceMoved[(selectedPiece.O * -1) / 2 + 1.5]

//         // build a cache for the new table
//         local getPieceAt = {}
//         for _, piece in ipairs(tempBoardWithSelectedPieceMoved){
//             if not getPieceAt[piece.X] then
//                 getPieceAt[piece.X] = {}
//             }
//             getPieceAt[piece.X][piece.Y] = piece;
//         }

//         // print(dump(kingPiece))

//         if not check(kingPiece, tempBoardWithSelectedPieceMoved, getPieceAt) then
//             table.insert(newAvailableSquares, availableSquare)
//         }
//     }

//     return newAvailableSquares
// }

// // move the piece
// fn movePiece(pieceFrom, x, y)

//     local promote = true
//     //{ not promote the piece if we're moving fromt storage
//     if pieceFrom.X == 0 or pieceFrom.Y == 0 then
//         promote = false
//     }

//     // get all available moves
//     for _, available in ipairs(Available){
//         if available.X == x and available.Y == y then
//             local pieceAboutToBeOverwritten = GetPieceAt[available.X][available.Y]
//             if pieceAboutToBeOverwritten then
//                 if pieceAboutToBeOverwritten.O ~= pieceFrom.O then
//                     // local goal = {}
//                     // goal.CFrame = origin * CFrame.new(12.5 - 2.5 * spot.X, 5, 2.5 * spot.Y - 12.5) * CFrame.Angles(0, math.pi / 2 * spot.O, math.pi / 36)
//                     // tween:Create(spot.K, TweenInfo.new(0.5), goal):Play()

//                     // PUT IT IN THE DISCARD PILE
//                     pieceAboutToBeOverwritten.X = 0
//                     pieceAboutToBeOverwritten.Y = 0
//                     pieceAboutToBeOverwritten.P = 1
//                     pieceAboutToBeOverwritten.O = pieceFrom.O
//                     // wait(0.75)
//                     // goal.CFrame = origin * CFrame.new(-16 * spot.O, 5, -7.5 * spot.O) * CFrame.Angles(0, math.pi / 2 * spot.O, math.pi / 36)
//                     // tween:Create(spot.K, TweenInfo.new(0.5), goal):Play()
//                     // wait(0.5)

//                     // I'm assuming this is to animate piece movement and move the camera angle
//                     // if spot.Name == "Pawn" then
//                     // 	// local ran = math.random(1, 3) - 2
//                     // 	// goal.CFrame = origin * CFrame.new((-16 + ran * 2.5) * spot.O, 0.306, -5 * spot.O) * CFrame.Angles(0, math.pi / 2 * spot.O, math.pi / 36)
//                     // elseif spot.Name == "Lance" then
//                     // 	// goal.CFrame = origin * CFrame.new(-13.5 * spot.O, 0.306, -7.5 * spot.O) * CFrame.Angles(0, math.pi / 2 * spot.O, math.pi / 36)
//                     // elseif spot.Name == "Knight" then
//                     // 	// goal.CFrame = origin * CFrame.new(-16 * spot.O, 0.306, -7.5 * spot.O) * CFrame.Angles(0, math.pi / 2 * spot.O, math.pi / 36)
//                     // elseif spot.Name == "Silver" then
//                     // 	// goal.CFrame = origin * CFrame.new(-18.5 * spot.O, 0.306, -7.5 * spot.O) * CFrame.Angles(0, math.pi / 2 * spot.O, math.pi / 36)
//                     // elseif spot.Name == "Gold" then
//                     // 	// goal.CFrame = origin * CFrame.new(-13.5 * spot.O, 0.306, -10 * spot.O) * CFrame.Angles(0, math.pi / 2 * spot.O, math.pi / 36)
//                     // elseif spot.Name == "Bishop" then
//                     // 	// goal.CFrame = origin * CFrame.new(-16 * spot.O, 0.306, -10 * spot.O) * CFrame.Angles(0, math.pi / 2 * spot.O, math.pi / 36)
//                     // elseif spot.Name == "Rook" then
//                     // 	// goal.CFrame = origin * CFrame.new(-18.5 * spot.O, 0.306, -10 * spot.O) * CFrame.Angles(0, math.pi / 2 * spot.O, math.pi / 36)
//                     // }
//                     // tween:Create(spot.K, TweenInfo.new(0.5), goal):Play()
//                     // wait(0.5)
//                 else
//                     // cool = false
//                     return
//                 }
//             }

//             // the old piece that we're moving
//             // local oldp = pieceFrom.P
//             // local oldx = pieceFrom.X

//             // promote dep}ing on the player and row
//             if promote == true and pieceFrom.Name ~= "Gold" and pieceFrom.Name ~= "King" and pieceFrom.Name ~= "Jewel" then
//                 if pieceFrom.O == Player.TWO and pieceFrom.Y <= 3 then
//                     pieceFrom.P = Rank.PROMOTED
//                 elseif pieceFrom.O == Player.ONE and pieceFrom.Y >= 7 then
//                     pieceFrom.P = Rank.PROMOTED
//                 }
//             }

//             // local goal = {}

//             // special animations for knight
//             // if oldx == 0 then
//             // 	// goal.CFrame = origin * CFrame.new(-16 * call.O, 5, -7.5 * call.O) * CFrame.Angles(0, math.pi / 2 * call.O, math.pi / 36)
//             // 	// tween:Create(call.K, TweenInfo.new(0.5), goal):Play()
//             // 	// wait(0.75)
//             // elseif string.sub(pieceName, 1, 6) == "Knight" and call.P == 1 then
//             // 	// goal.CFrame = origin * CFrame.new(12.5 - 2.5 * call.X, 5, 2.5 * call.Y - 12.5) * CFrame.Angles(0, math.pi / 2 * call.O, math.pi / 36)
//             // 	// tween:Create(call.K, TweenInfo.new(0.5), goal):Play()
//             // 	// wait(0.75)
//             // }

//             // move the piece
//             pieceFrom.X = x
//             pieceFrom.Y = y

//             // if oldp == 2 then
//             // 	goal.CFrame = origin * CFrame.new(12.5 - 2.5 * call.X, 1.6, 2.5 * call.Y - 12.5) * CFrame.Angles(0, -math.pi / 2 * call.O, math.pi)
//             // else
//             // 	goal.CFrame = origin * CFrame.new(12.5 - 2.5 * call.X, 1.6, 2.5 * call.Y - 12.5) * CFrame.Angles(0, math.pi / 2 * call.O, math.pi / 36)
//             // }

//             // tween:Create(call.K, TweenInfo.new(0.5), goal):Play()
//             // wait(0.5)

//             // if oldp == 1 and call.P == 2 then
//             // 	wait(0.25)
//             // 	goal.CFrame = origin * CFrame.new(12.5 - 2.5 * call.X, 5, 2.5 * call.Y - 12.5) * CFrame.Angles(0, -math.pi / 2 * call.O, math.pi)
//             // 	tween:Create(call.K, TweenInfo.new(0.5), goal):Play()
//             // 	wait(0.75)
//             // 	goal.CFrame = origin * CFrame.new(12.5 - 2.5 * call.X, 5, 2.5 * call.Y - 12.5) * CFrame.Angles(0, -math.pi / 2 * call.O, math.pi)
//             // 	tween:Create(call.K, TweenInfo.new(0.25), goal):Play()
//             // 	wait(0.25)
//             // 	goal.CFrame = origin * CFrame.new(12.5 - 2.5 * call.X, 1.6, 2.5 * call.Y - 12.5) * CFrame.Angles(0, -math.pi / 2 * call.O, math.pi)
//             // 	tween:Create(call.K, TweenInfo.new(0.25), goal):Play()
//             // 	wait(0.25)
//             // }

//             Turn = Turn * -1

//             local kingPiece = Pieces[(Turn * -1) / 2 + 1.5]

//             // rebuild piecea
//             GetPieceAt = buildGetPieceAtFrom(Pieces)

//             local check = check(kingPiece, Pieces, GetPieceAt)

//             if check then
//                 HUD.check = "Check!"
//             }

//             // update text for player's turn
//             // board.BillboardGui.TextLabel.Text = addon .. "Player " .. player .. "'s Turn"
//             // board.Selected.Position = Vector3.new(0, -3.2, -17.5 * turn) + origin.p

//             local numofmoves = 0

//             for _, piece in ipairs(Pieces){
//                 if piece.O == Turn then
//                     local available = moves(piece)
//                     numofmoves = numofmoves + #available
//                 }
//             }

//             if numofmoves == 0 then
//                 // create toast for player winning and losing
//                 // board.BillboardGui.TextLabel.Text = "Checkmate - Player " .. player .. " Wins"
//                 Running = false
//             }
//         }
//     }
// }

// // while running{
// //     local hashmap = {}
// //     for _, piece in ipairs(pieces){
// //         hashmap[piece.X] = {}
// //         hashmap[piece.X][piece.Y] = piece
// //     }

// //     local bot = false

// //     if bot1 == true and turn == -1 then
// //         bot = true
// //     elseif bot2 == true and turn == 1 then
// //         bot = true
// //     }

// //     // if we've enabled a bot
// //     if bot == true then
// //         local piecetomove = {}
// //         local move = {}
// //         local movevalue = math.huge * -1

// //         // iterate over every piece
// //         for pieceIdx, piece in pairs(pieces){
// //             // if it's our piece
// //             if piece.O == turn then
// //                 // check the available moves
// //                 for availableIdx, available in pairs(moves(piece)){
// //                     local moveworth = 0
// //                     local indanger = check(piece)

// //                     // if we're in check then (probably) decide on the
// //                     // value of pieces to sacrifice
// //                     if indanger == true then
// //                         if piece.P == Player.ONE then
// //                             if piece.Name == "Pawn" then
// //                                 moveworth = moveworth + 1
// //                             elseif piece.Name == "Knight" then
// //                                 moveworth = moveworth + 4
// //                             elseif piece.Name == "Lance" then
// //                                 moveworth = moveworth + 6
// //                             elseif piece.Name == "Silver" then
// //                                 moveworth = moveworth + 9
// //                             elseif piece.Name == "Gold" then
// //                                 moveworth = moveworth + 10
// //                             elseif piece.Name == "Bishop" then
// //                                 moveworth = moveworth + 15
// //                             elseif piece.Name == "Rook" then
// //                                 moveworth = moveworth + 16
// //                             }
// //                         else
// //                             if piece.Name == "Pawn" then
// //                                 moveworth = moveworth + 8
// //                             elseif piece.Name == "Knight" then
// //                                 moveworth = moveworth + 8
// //                             elseif piece.Name == "Lance" then
// //                                 moveworth = moveworth + 8
// //                             elseif piece.Name == "Silver" then
// //                                 moveworth = moveworth + 9
// //                             elseif piece.Name == "Bishop" then
// //                                 moveworth = moveworth + 25
// //                             elseif piece.Name == "Rook" then
// //                                 moveworth = moveworth + 27
// //                             }
// //                         }
// //                     }

// //                     // copy over table
// //                     local newtable = {}
// //                     for piece in pieces{
// //                         table.insert(newtable, {
// //                             X = piece.X,
// //                             Y = piece.Y,
// //                             P = piece.P,
// //                             O = piece.O
// //                         })
// //                     }

// //                     // check if we're moving into a spot that has an opponent's piece
// //                     local spot = hashmap(available, newtable)
// //                     if spot then
// //                         if spot.O ~= piece.O then
// //                             if spot.P == 1 then
// //                                 if spot.Name == "Pawn" then
// //                                     moveworth = moveworth + 1
// //                                 elseif spot.Name == "Knight" then
// //                                     moveworth = moveworth + 4
// //                                 elseif spot.Name == "Lance" then
// //                                     moveworth = moveworth + 6
// //                                 elseif spot.Name == "Silver" then
// //                                     moveworth = moveworth + 9
// //                                 elseif spot.Name == "Gold" then
// //                                     moveworth = moveworth + 10
// //                                 elseif spot.Name == "Bishop" then
// //                                     moveworth = moveworth + 15
// //                                 elseif spot.Name == "Rook" then
// //                                     moveworth = moveworth + 16
// //                                 }
// //                             else
// //                                 if spot.Name == "Pawn" then
// //                                     moveworth = moveworth + 8
// //                                 elseif spot.Name == "Knight" then
// //                                     moveworth = moveworth + 8
// //                                 elseif spot.Name == "Lance" then
// //                                     moveworth = moveworth + 8
// //                                 elseif spot.Name == "Silver" then
// //                                     moveworth = moveworth + 9
// //                                 elseif spot.Name == "Bishop" then
// //                                     moveworth = moveworth + 25
// //                                 elseif spot.Name == "Rook" then
// //                                     moveworth = moveworth + 27
// //                                 }
// //                             }
// //                             // if so, move the piece to our posession
// //                             spot.X = 0
// //                             spot.Y = 0
// //                             spot.P = 1
// //                             spot.O = piece.O
// //                         }
// //                     }
// //                     // update that piece's location to its new available location
// //                     newtable[pieceIdx].X = available[availableIdx].X
// //                     newtable[pieceIdx].Y = available[availableIdx].Y

// //                     local check = false
// //                     for tPieceIdx, tPiece in pairs(pieces){
// //                         // if the piece is not owned by the player
// //                         if tPiece.O == piece.O * -1 then
// //                             for spot in findavailable(tPieceIdx, newtable){
// //                                 if spot.X == newtable[pieceIdx].X and spot.Y == newtable[pieceIdx].Y then
// //                                     check = true
// //                                 }
// //                             }
// //                         }
// //                     }

// //                     if check == true then
// //                         if piece.P == 1 then
// //                             if piece.Name == "Pawn" then
// //                                 moveworth = moveworth - 1
// //                             elseif piece.Name == "Knight" then
// //                                 moveworth = moveworth - 4
// //                             elseif piece.Name == "Lance" then
// //                                 moveworth = moveworth - 6
// //                             elseif piece.Name == "Silver" then
// //                                 moveworth = moveworth - 9
// //                             elseif piece.Name == "Gold" then
// //                                 moveworth = moveworth - 10
// //                             elseif piece.Name == "Bishop" then
// //                                 moveworth = moveworth - 15
// //                             elseif piece.Name == "Rook" then
// //                                 moveworth = moveworth - 16
// //                             }
// //                         else
// //                             if piece.Name == "Pawn" then
// //                                 moveworth = moveworth - 8
// //                             elseif piece.Name == "Knight" then
// //                                 moveworth = moveworth - 8
// //                             elseif piece.Name == "Lance" then
// //                                 moveworth = moveworth - 8
// //                             elseif piece.Name == "Silver" then
// //                                 moveworth = moveworth - 9
// //                             elseif piece.Name == "Bishop" then
// //                                 moveworth = moveworth - 25
// //                             elseif piece.Name == "Rook" then
// //                                 moveworth = moveworth - 27
// //                             }
// //                         }
// //                     }

// //                     if moveworth == movevalue then
// //                         movevalue = moveworth
// //                         local num = #piecetomove + 1
// //                         piecetomove[num] = pieceIdx
// //                         move[num] = available[availableIdx]
// //                     elseif moveworth > movevalue then
// //                         movevalue = moveworth
// //                         local num = 1
// //                         piecetomove = {}
// //                         move = {}
// //                         piecetomove[num] = pieceIdx
// //                         move[num] = available[availableIdx]
// //                     }
// //                 }
// //             }
// //         }
// //         if #piecetomove >= 1 then
// //             local num = math.random(1, #piecetomove)
// //             movepiece(nil, piecetomove[num], move[num].X, move[num].Y, true)
// //         }
// //     }
// //     // wait(0.1)
// // }

// // // add a listener to every piece
// // for i, v in pairs(pieces){
// //     // on click
// //     // TODO: attach with love2d's API
// //     fn onClick()
// //         // ?????????
// //         // if cool == true then return }

// //         // if game}ed == true then return }

// //         // if we're on the right turn
// //         if v.O == turn then
// //             // create available move
// //             local available = moves(i)

// //             // r}er available move?
// //             // game.ReplicatedStorage.RemoteEvent:FireClient(player, i, "place", available, v, board)
// //         else
// //             // game.ReplicatedStorage.RemoteEvent:FireClient(player, nil, "attack", v)
// //             // confirm available move(kill)
// //         }
// //     }
// // }

// // local plr1time = 0
// // // label board
// // local timeLabelOne = "Time: " .. math.floor(plr1time)
// // // board.Timer1.SurfaceGui.TextLabel.Text = "Time: " .. math.floor(plr1time)
// // local plr2time = 0
// // local timeLabelTwo = "Time: " .. math.floor(plr2time)
// // // board.Timer2.SurfaceGui.TextLabel.Text = "Time: " .. math.floor(plr2time)

// // repeat
// //     // wait(0.1)
// //     if turn == -1 then
// //         // if player1 ~= nil or bot1 == true then
// //         plr1time = plr1time + 0.1
// //         timeLabelOne = "Time: " .. math.floor(plr1time)
// //         // }
// //     elseif turn == 1 then
// //         // if player2 ~= nil or bot2 == true then
// //         plr2time = plr2time + 0.1
// //         timeLabelTwo = "Time: " .. math.floor(plr2time)
// //         // }
// //     }
// // until running == false

// function love.load()
//     // setup the board
//     setupPieces()
//     // setup other things
//     // TODO we need to update all of these on board resize

//     ActualWidth = love.graphics.getWidth() * 0.8
//     ActualHeight = love.graphics.getHeight() * 0.75

//     PaddingLeft = (love.graphics.getWidth() - ActualWidth) / 2
//     PaddingRight = PaddingLeft

//     PaddingTop = (love.graphics.getHeight() - ActualHeight) / 2
//     PaddingBottom = PaddingTop

//     // define the point at which we scale by
//     BoardLength = math.min(ActualHeight, ActualWidth)
//     SquareSideLength = BoardLength / 9

//     // padding between board and placement
//     ExtraBoardLength = SquareSideLength * 7
//     ExtraPaddingLeft = (BoardLength - ExtraBoardLength) / 2
//     YOffset = 0.4 * SquareSideLength

//     // now we need to center the square, because padding{esn't quite cover it now
//     CenterPaddingLeft = (ActualWidth - BoardLength) / 2
//     CenterPaddingTop = (ActualHeight - BoardLength) / 2

//     // now we finally have our origin coordinate. This corresponds to the top left of the board
//     Origin = {
//         x = PaddingLeft + CenterPaddingLeft,
//         y = PaddingTop + CenterPaddingTop,
//     }

//     ExtraOrigin = {
//         [Player.ONE] = {
//             x = Origin.x + ExtraPaddingLeft,
//             y = Origin.y + BoardLength + YOffset
//         },
//         [Player.TWO] = {
//             x = Origin.x + ExtraPaddingLeft,
//             y = Origin.y - YOffset - SquareSideLength
//         }
//     }

//     PieceGraphic = {
//         // i have no idea if this is the right kanji for each piece but we're
//         // rolling with it
//         // https://shogi.fandom.com/wiki/Pieces
//         [Rank.NORMAL] = {
//             King = "玉",
//             Gold = "金",
//             Silver = "銀",
//             Knight = "桂",
//             Lance = "香",
//             Rook = "飛",
//             Bishop = "角",
//             Pawn = "歩"
//         },
//         [Rank.PROMOTED] = {
//             King = "玉",
//             Gold = "金",
//             Silver = "全",
//             Knight = "今",
//             Lance = "杏",
//             Rook = "竜",
//             Bishop = "馬",
//             Pawn = "と"
//         }
//     }

//     // load default japanese font
//     PieceFont = love.graphics.newFont("res/YujiBoku-Regular.ttf", 40)

//     // Player turn
//     Turn = Player.ONE

//     GetPieceAt = buildGetPieceAtFrom(Pieces)

//     // Determines what we're r}ering?
//     // ShowAvailable = false
//     Available = {}
//     SelectedPiece = nil

//     HUD = {}
//     Running = true

//     Captures = generateCapturesFrom(Pieces)
// }

// function love.update()
// }

// function love.draw()
//     love.graphics.clear()

//     // draw the grid

//     // 0 -> 9 inclusive, since we want to draw the left and right edges of the board

//     for i = 0, 9{
//         local offset = SquareSideLength * i

//         // draw a horizontal line from left to right, incrementing horizontal
//         love.graphics.line(Origin.x, Origin.y + offset, Origin.x + BoardLength, Origin.y + offset)

//         // draw a vertical line
//         love.graphics.line(Origin.x + offset, Origin.y, Origin.x + offset, Origin.y + BoardLength)

//     }

//     // draw the taken pieces board
//     for i = 0, 7{
//         local offset = SquareSideLength * i
//         for _, playerOrigin in pairs(ExtraOrigin){
//             love.graphics.line(playerOrigin.x + offset, playerOrigin.y, playerOrigin.x + offset, playerOrigin.y + SquareSideLength)
//         }
//         // draw the taken pieces vertical line
//         // love.graphics.line(POneOrigin.x + offset, POneOrigin.y, POneOrigin.x + offset, POneOrigin.y + SquareSideLength)
//     }

//     // draw the 4 taken pieces horizontal lines
//     for _, playerOrigin in pairs(ExtraOrigin){
//         for _, y in ipairs({ 0, SquareSideLength }){
//             love.graphics.line(playerOrigin.x, playerOrigin.y + y, playerOrigin.x + ExtraBoardLength, playerOrigin.y + y)
//         }
//     }

//     // draw the pieces

//     love.graphics.setFont(PieceFont)

//     for _, piece in ipairs(Pieces){
//         // use our piece graphic to r}er its place on the grid
//         // constant offset to the right by 6
//         // constant offset up by 8
//         // in order to center the pieces
//         if piece.X ~= 0 and piece.X ~= 0 then
//             love.graphics.print(PieceGraphic[piece.P][piece.Name], Origin.x + (piece.X - 1) * SquareSideLength + 6, Origin.y + (9 - piece.Y) * SquareSideLength - 8)
//         }
//     }

//     // draw the available moves
//     if SelectedPiece then
//         for _, square in ipairs(Available){
//             // just draw green squares i guess
//             love.graphics.rectangle("fill", Origin.x + (square.X - 1) * SquareSideLength, Origin.y + (9 - square.Y) * SquareSideLength, SquareSideLength, SquareSideLength)
//         }
//     }

//     // draw the taken pieces next

//     // first is player one, second is player two
//     for player, sevenList in pairs(Captures){
//         local i = 0
//         for _, pieces in pairs(sevenList){
//             i = i + 1
//             // print("hello")
//             // print(dump(pieces))

//             local piece = pieces[1]
//             if piece then
//                 love.graphics.print(PieceGraphic[piece.P][piece.Name], ExtraOrigin[player].x + (i - 1) * SquareSideLength + 6, ExtraOrigin[player].y - 8)
//                 // love.graphics.print(PieceGraphic[piece.P][piece.Name], ExtraOrigin[player].x + (i - 1) * SquareSideLength + 6, ExtraOrigin[player].y)
//                 // love.graphics.print("DSFSDF", ExtraOrigin[player].x + (i - 1) * SquareSideLength + 6, ExtraOrigin[player].y)

//                 // love.graphics.print("ACK", ExtraOrigin[player].x, ExtraOrigin[player].y)

//             }
//         }
//     }

//     // probably draw ui here as well

// }

// function love.mousepressed(x, y, button, istouch)
//     // https://love2d.org/wiki/love.mousepressed 1 is primary mouse button, 2 is secondary, 3 is scroll wheel

//     if button ~= 1 then
//         return
//     }
//     // check if the mouse click happened within the board
//     if x >= Origin.x and y >= Origin.y and x <= Origin.x + BoardLength and y <= Origin.y + BoardLength then
//         // calculate index
//         local pieceX = math.floor((x - Origin.x) / SquareSideLength) + 1
//         local pieceY = 9 - math.floor((y - Origin.y) / SquareSideLength)

//         // if we haven't selected a previous piece yet
//         if not SelectedPiece then
//             // get piece at that location
//             SelectedPiece = GetPieceAt[pieceX][pieceY]

//             // if piece's owner{es not belong to the person that's moving, or a piece{esn't exist
//             if not SelectedPiece or SelectedPiece.O ~= Turn then
//                 // deselect everything
//                 SelectedPiece = nil
//                 return
//             }
//             // populate the available list!
//             Available = moves(SelectedPiece)
//         else
//             // if in available
//             movePiece(SelectedPiece, pieceX, pieceY)
//             SelectedPiece = false

//             // ok, now we're all good
//             Captures = generateCapturesFrom(Pieces)
//         }
//     }
//     // TODO need to take into account the captured pieces later
// }