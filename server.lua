Player = {
	ONE = 1,
	TWO = -1
}

Rank = {
	NORMAL = 1,
	PROMOTED = 2
}

local function swapPlayer(player)
	return player * -1
end

-- tf is tween service?
-- local tween = game:GetService("TweenService")

local function getNewBoard()

	-- local pieces = {};
	-- each piece is a class that holds 5 variables, K, X, Y, P, and O
	-- O is for ownership
	-- P is for ???
	-- X and Y are the coordinates of the piece
	local pieces = { {
		Name = "King",
		X = 5,
		Y = 1,
		O = Player.ONE
	},
	{
		Name = "Gold",
		X = 4,
		Y = 1,
		O = Player.ONE
	},
	{
		Name = "Gold",
		X = 6,
		Y = 1,
		O = Player.ONE
	},
	{
		Name = "Silver",
		X = 3,
		Y = 1,
		O = Player.ONE
	},
	{
		Name = "Silver",
		X = 7,
		Y = 1,
		O = Player.ONE
	},
	{
		Name = "Knight",
		X = 2,
		Y = 1,
		O = Player.ONE
	},
	{
		Name = "Knight",
		X = 8,
		Y = 1,
		O = Player.ONE
	},
	{
		Name = "Lance",
		X = 1,
		Y = 1,
		O = Player.ONE
	},
	{
		Name = "Lance",
		X = 9,
		Y = 1,
		O = Player.ONE
	},
	{
		Name = "Rook",
		X = 8,
		Y = 2,
		O = Player.ONE
	},
	{
		Name = "Bishop",
		X = 2,
		Y = 2,
		O = Player.ONE
	},
	}

	for i = 1, 9 do
		-- "pawn1", "pawn2", etc.
		table.insert(pieces, {
			Name = "Pawn",
			X = i,
			Y = 3,
			O = Player.ONE
		});
	end

	local pieces = { {
		Name = "King",
		X = 5,
		Y = 9,
		O = Player.TWO
	},
	{
		Name = "Gold",
		X = 4,
		Y = 9,
		O = Player.TWO
	},
	{
		Name = "Gold",
		X = 6,
		Y = 9,
		O = Player.TWO
	},
	{
		Name = "Silver",
		X = 3,
		Y = 9,
		O = Player.TWO
	},
	{
		Name = "Silver",
		X = 7,
		Y = 9,
		O = Player.TWO
	},
	{
		Name = "Knight",
		X = 2,
		Y = 9,
		O = Player.TWO
	},
	{
		Name = "Knight",
		X = 8,
		Y = 9,
		O = Player.TWO
	},
	{
		Name = "Lance",
		X = 1,
		Y = 9,
		O = Player.TWO
	},
	{
		Name = "Lance",
		X = 9,
		Y = 9,
		O = Player.TWO
	},
	{
		Name = "Rook",
		X = 2,
		Y = 8,
		O = Player.TWO
	},
	{
		Name = "Bishop",
		X = 8,
		Y = 8,
		O = Player.TWO
	},
	}

	for i = 1, 9 do
		-- "pawn1", "pawn2", etc.
		table.insert(pieces, {
			Name = "Pawn",
			X = i,
			Y = 7,
			O = Player.TWO
		});
	end

	for piece in pieces do
		piece.P = Rank.NORMAL;
	end

	return pieces
end

-- local function queryPiece(queryPiece, pieceList)
-- 	for piece in pieceList do
-- 		-- comparing equality of objects
-- 		if table.concat(piece) == table.concat(queryPiece) then
-- 			return piece
-- 		end
-- 	end
-- 	return false
-- end

-- sets up pieces for both players
local function setup()
	-- seems to be a 3d coordinate of some sort
	-- local origin = CFrame.new(board.Position)

	-- an array of all the pieces on the board
	-- except we're using a dictionary
	local pieces = getNewBoard()
	-- create a registry for pieces, returns the index of the array
	local pieceRegistry = {}
	for idx, piece in pairs(pieces) do
		pieceRegistry[piece.Name][piece.O] = idx
	end

	-- variable that stores if the game is not over
	local running = true

	-- local player1 = nil
	-- board.Pillow1.BillboardGui.TextLabel.Text = "Empty"
	local bot1 = false
	-- board.Bot1.Color = Color3.new(1, 0, 0)
	-- board.Bot1.SurfaceGui.TextLabel.Text = "Bot: Off"

	-- local player2 = nil
	-- board.Pillow2.BillboardGui.TextLabel.Text = "Empty"
	local bot2 = false
	-- board.Bot2.Color = Color3.new(1, 0, 0)
	-- board.Bot2.SurfaceGui.TextLabel.Text = "Bot: Off"

	-- ?????????
	local cool = false

	local turn = Player.ONE

	-- board.BillboardGui.TextLabel.Text = "Player 1's Turn"
	-- board.Selected.Position = Vector3.new(0, -3.2, 17.5) + origin.p

	-- if bot is clicked, light up with an indicator and show that bot is on,
	-- for both players

	-- checks if there is a piece at the current location, given a location (x,
	-- y) and an array of pieces
	-- local function piece_exists_at(location, hashmap)
	-- 	return hashmap[location.Y][location.X]
	-- end

	-- probably find all places we can move
	local function findavailable(pieceInQuestion, newtable)
		-- board of current pieces
		local pieces = newtable or pieces

		-- we should never need this
		if pieceInQuestion == nil then return end
		local pieceName = pieceInQuestion.Name

		local available = {}

		-- hashmap, stores existing pieces at their coordinates
		local hashmap = {}
		for piece in pieces do
			hashmap[piece.X][piece.Y] = piece;
		end

		-- potential bug: if it's pawn then there's literally no available squares
		-- if we've clicked outside of the arena?
		if pieceInQuestion.X == 0 or pieceInQuestion.Y == 0 then
			-- if we're placing a pawn somewhere on the board, check if there is a pawn in that column already
			for h = 1, 9 do
				local pawn = false
				if pieceName == "Pawn" then
					for v = 1, 9 do
						-- tab is all coordinates of the board
						-- if there is a spot on the table for this location
						local spot = hashmap[h][v]
						if spot then
							if spot.O == pieceInQuestion.O and spot.Name == "Pawn" and spot.P == 1 then
								pawn = true
								break
							end
						end
					end
				end
				-- if it's not a pawn
				if pawn == false then
					for v = 1, 9 do
						-- make every spot available

						local spot = hashmap[h][v];

						-- pieces can only be sent to where they can move one more time (so they can promote)

						-- we can only send knight up to third to last rank
						-- we can only send pawn and lace up to second to last rank

						-- all other pieces can be sent to last row

						if not spot then
							if pieceName == "Pawn" or pieceName == "Lance" or pieceName == "Knight" then
								if pieceInQuestion.O * 4 + 5 == v then
									table.insert(available, { X = h, Y = v })
								elseif pieceInQuestion.O * 3 + 5 == v and pieceName == "Knight" then
									table.insert(available, { X = h, Y = v })
								end
							else
								table.insert(available, { X = h, Y = v })
							end
						end
					end
				end
			end
		else
			local block = false
			local dydxList = {}
			-- for ALL pieces, we generate a list of dydx
			-- we iterate over the dydx, and only then do we actually do boundary checking

			-- the problem currently is the player side. We have to treat each
			-- dy as negative or positive depending on the side the opponent is
			-- on

			-- if it's pawn promote to gold
			if pieceName == "Pawn" then
				if pieceInQuestion.P == Rank.NORMAL then
					table.insert(dydxList, { -1 * pieceInQuestion.O, 0 })
				else
					for y = -1, 1 do
						for x = -1, 1 do
							if ~(y == -1 and x == -1) and ~(y == -1 and x == 1) then
								table.insert(dydxList, { y * pieceInQuestion.O, x })
							end
						end
					end
				end
				-- for lance, dynamically generate a list of dydx at runtime
			elseif pieceName == "Lance" then
				if pieceInQuestion.P == Rank.NORMAL then
					local currentX = pieceInQuestion.X
					local currentY = pieceInQuestion.Y + pieceInQuestion.O
					-- get position of rook in this direction
					while currentX >= 1 and currentX <= 9 and currentY >= 1 and currentY <= 9 do
						local spot = hashmap[currentX][currentY]

						-- if there is a piece in the way, our rook cannot move further
						if spot then
							-- check ownership
							-- if we own, do not include the spot as a valid move
							if spot.O == pieceInQuestion.O then
								-- otherwise, add it as a valid move directly into the insert
								table.insert(available, {
									X = currentX,
									Y = currentY
								})
							end
							-- break out
							break
						else
							table.insert(available, {
								X = currentX,
								Y = currentY
							})
						end

						currentX = currentX
						currentY = currentY + pieceInQuestion.O
					end
				else
					-- gold movement
					for y in -1, 1 do
						for x in -1, 1 do
							if ~(y == -1 and x == -1) and ~(y == -1 and x == 1) then
								table.insert(dydxList, { y * pieceInQuestion.O, x })
							end
						end
					end
				end
				-- if it's knight promote to gold
			elseif pieceName == "Knight" then
				if pieceInQuestion.P == Rank.NORMAL then
					for x in { -1, 1 } do
						table.insert(dydxList, { 2 * pieceInQuestion.O, x })
					end
				else
					for y in -1, 1 do
						for x in -1, 1 do
							if ~(y == -1 and x == -1) and ~(y == -1 and x == 1) then
								table.insert(dydxList, { y * pieceInQuestion.O, x })
							end
						end
					end
				end
				-- if it's silver promote to gold
			elseif pieceName == "Silver" then
				if pieceInQuestion.P == Rank.NORMAL then
					for coordinate in {
						{
							1, -1
						}, {
							1, 0
						}, {
							1, 1
						},
						{
							-1, -1
						},
						{
							-1, 1
						}
					} do
						table.insert(dydxList, { coordinate[0] * pieceInQuestion.O, coordinate[1] })
					end
				else
					for y in -1, 1 do
						for x in -1, 1 do
							if ~(y == -1 and x == -1) and ~(y == -1 and x == 1) then
								table.insert(dydxList, { y * pieceInQuestion.O, x })
							end
						end
					end
				end
				-- if it's gold promote to gold
			elseif pieceName == "Gold" then
				for y in -1, 1 do
					for x in -1, 1 do
						if ~(y == -1 and x == -1) and ~(y == -1 and x == 1) then
							table.insert(dydxList, { y * pieceInQuestion.O, x })
						end
					end
				end
				-- if it's gold promote to gold
			elseif pieceName == "King" then
				for y in -1, 1 do
					for x in -1, 1 do
						table.insert(dydxList, { y * pieceInQuestion.O, x })
					end
				end
			elseif pieceName == "Rook" then
				-- the rook can go in four directions
				local directionsDydx = {
					{ 1, 0 },
					{ -1, 0 },
					{ 0, 1 },
					{ 0, -1 }
				}

				-- for each direction
				for dy, dx in directionsDydx do
					local currentX = pieceInQuestion.X + dx
					local currentY = pieceInQuestion.Y + dy
					-- get position of rook in this direction
					while currentX >= 1 and currentX <= 9 and currentY >= 1 and currentY <= 9 do
						local spot = hashmap[currentX][currentY]

						-- if there is a piece in the way, our rook cannot move further
						if spot then
							-- check ownership
							-- if we own, do not include the spot as a valid move
							if spot.O == pieceInQuestion.O then
								-- otherwise, add it as a valid move directly into the insert
								table.insert(available, {
									X = currentX,
									Y = currentY
								})
							end
							-- break out
							break
						else
							table.insert(available, {
								X = currentX,
								Y = currentY
							})
						end

						currentX = currentX + dx
						currentY = currentY + dy
					end
				end

				if pieceInQuestion.P == Rank.PROMOTED then
					for coordinate in {
						{
							1, 1
						}, {
							1, -1
						}, {
							-1, -1
						},
						{
							-1, 1
						}
					} do
						table.insert(dydxList, { coordinate[0] * pieceInQuestion.O, coordinate[1] })
					end
				end
			elseif pieceName == "Bishop" then
				-- the bishop can go in four directions
				local directionsDydx = {
					{ 1, 1 },
					{ 1, -1 },
					{ -1, 1 },
					{ -1, -1 }
				}

				-- for each direction
				for dy, dx in directionsDydx do
					local currentX = pieceInQuestion.X + dx
					local currentY = pieceInQuestion.Y + dy
					-- get position of bishop in this direction
					while currentX >= 1 and currentX <= 9 and currentY >= 1 and currentY <= 9 do
						local spot = hashmap[currentX][currentY]

						-- if there is a piece in the way, our bishop cannot move further
						if spot then
							-- check ownership
							-- if we own, do not include the spot as a valid move
							if spot.O ~= pieceInQuestion.O then
								-- otherwise, add it as a valid move directly into the insert
								table.insert(available, {
									X = currentX,
									Y = currentY
								})
							end
							-- break out
							break
						else
							table.insert(available, {
								X = currentX,
								Y = currentY
							})
						end

						currentX = currentX + dx
						currentY = currentY + dy
					end
				end

				if pieceInQuestion.P == Rank.PROMOTED then
					for coordinate in {
						{
							1, 0
						}, {
							-1, 0
						}, {
							0, -1
						},
						{
							0, 1
						}
					} do
						table.insert(dydxList, { coordinate[0] * pieceInQuestion.O, coordinate[1] })
					end
				end
			end

			for dy, dx in dydxList do
				local newY = pieceInQuestion.Y + dy
				local newX = pieceInQuestion.X + dx

				if ~(newY > 9 or newY < 1) and ~(newX > 9 or newX < 1) then
					local spot = hashmap[newX][newY]
					if spot then
						if spot.O ~= pieceInQuestion.O then
							table.insert(available, {
								X = newX,
								Y = newY
							})
						end
					end
				end
			end

		end
		return available
	end

	-- function that takes the name of a piece and a table, returns if it checks
	-- the king?
	-- how are we getting the name in piece ?
	local function check(piece, newtable)
		-- iterate over the array until we find owner
		-- this is less efficient than a hashmap (what we were using before)...
		-- pieces are immutable anyway so using a registry might be best
		-- for now we'll iterate

		local check = false

		for piece in pieces do
			if piece.O == piece.O * -1 then
				for spot in findavailable(piece, newtable) do
					if spot.X == piece.X and spot.Y == piece.Y then
						check = true
					end
				end
			end
		end
		return check
	end

	-- don't know, might just be for the AI
	local function moves(pieceName, newtable, registry)
		local pieces = newtable or pieces
		-- again, not sure how we're getting the piece name
		-- local pieceInQuestion = pieces[pieceName]

		if pieceInQuestion == nil then return end

		local newavailable = {}
		-- local available = findavailable(pieceInQuestion)

		local hashmap = {}
		for piece in pieces do
			hashmap[piece.X][piece.Y] = piece;
		end

		for available in findavailable(pieceInQuestion) do
			-- clone the current table
			local newtable = {}
			local newPieceInQuestion = nil
			for i, v in pairs(pieces) do
				newtable[i] = v
				if v.X == pieceInQuestion.X and v.Y == pieceInQuestion.Y then
					newPieceInQuestion = newtable[i]
				end
			end

			local spot = hashmap[available.X][available.Y]

			-- captured pieces in shogi go to the opponent, in which they can play anywhere
			if spot then
				if spot.O ~= pieceInQuestion.O then
					spot.X = 0
					spot.Y = 0
					spot.P = 1
					spot.O = pieceInQuestion.O
				end
			end

			-- move the piece in question to its new spot, in the new table
			newPieceInQuestion.X = available.X
			newPieceInQuestion.Y = available.Y

			local kingPiece = registry[pieceInQuestion.O]["King"]
			if not check(kingPiece, newtable) then
				table.insert(newavailable, available)
			end
		end
		return newavailable
	end

	-- add a listener to every piece
	for i, v in pairs(pieces) do
		-- on click
		-- TODO: attach with love2d's API
		local function onClick()
			-- ?????????
			-- if cool == true then return end

			-- if gameended == true then return end

			-- if we're on the right turn
			if v.O == turn then
				-- create available move
				local available = moves(i)

				-- render available move?
				-- game.ReplicatedStorage.RemoteEvent:FireClient(player, i, "place", available, v, board)
			else
				-- game.ReplicatedStorage.RemoteEvent:FireClient(player, nil, "attack", v)
				-- confirm available move(kill)
			end
		end
	end

	-- move the piece
	local function movepiece(plr, pieceName, x, y, promote, registry)
		if running == true then return end

		-- not sure how we're getting the piecename
		local call = pieces[pieceName]
		if call == nil then return end
		if turn ~= call.O then return end
		-- what is TRUE
		if cool == true then return end
		cool = true

		local hashmap = {}
		for piece in pieces do
			hashmap[piece.X][piece.Y] = piece;
		end

		-- unpromote the piece if we're moving it to storage
		if call.X == 0 or call.Y == 0 then
			promote = false
		end

		local hashmap = {}
		for piece in pieces do
			hashmap[piece.X][piece.Y] = piece;
		end

		-- get all available moves
		for available in moves(pieceName) do
			if available.X == x and available.Y == y then
				local spot = hashmap[available.X][available.Y]
				if spot then
					if spot.O ~= call.O then
						local goal = {}
						-- goal.CFrame = origin * CFrame.new(12.5 - 2.5 * spot.X, 5, 2.5 * spot.Y - 12.5) * CFrame.Angles(0, math.pi / 2 * spot.O, math.pi / 36)
						-- tween:Create(spot.K, TweenInfo.new(0.5), goal):Play()
						spot.X = 0
						spot.Y = 0
						spot.P = 1
						spot.O = call.O
						-- wait(0.75)
						-- goal.CFrame = origin * CFrame.new(-16 * spot.O, 5, -7.5 * spot.O) * CFrame.Angles(0, math.pi / 2 * spot.O, math.pi / 36)
						-- tween:Create(spot.K, TweenInfo.new(0.5), goal):Play()
						-- wait(0.5)

						-- I'm assuming this is to animate piece movement and move the camera angle
						-- if spot.Name == "Pawn" then
						-- 	-- local ran = math.random(1, 3) - 2
						-- 	-- goal.CFrame = origin * CFrame.new((-16 + ran * 2.5) * spot.O, 0.306, -5 * spot.O) * CFrame.Angles(0, math.pi / 2 * spot.O, math.pi / 36)
						-- elseif spot.Name == "Lance" then
						-- 	-- goal.CFrame = origin * CFrame.new(-13.5 * spot.O, 0.306, -7.5 * spot.O) * CFrame.Angles(0, math.pi / 2 * spot.O, math.pi / 36)
						-- elseif spot.Name == "Knight" then
						-- 	-- goal.CFrame = origin * CFrame.new(-16 * spot.O, 0.306, -7.5 * spot.O) * CFrame.Angles(0, math.pi / 2 * spot.O, math.pi / 36)
						-- elseif spot.Name == "Silver" then
						-- 	-- goal.CFrame = origin * CFrame.new(-18.5 * spot.O, 0.306, -7.5 * spot.O) * CFrame.Angles(0, math.pi / 2 * spot.O, math.pi / 36)
						-- elseif spot.Name == "Gold" then
						-- 	-- goal.CFrame = origin * CFrame.new(-13.5 * spot.O, 0.306, -10 * spot.O) * CFrame.Angles(0, math.pi / 2 * spot.O, math.pi / 36)
						-- elseif spot.Name == "Bishop" then
						-- 	-- goal.CFrame = origin * CFrame.new(-16 * spot.O, 0.306, -10 * spot.O) * CFrame.Angles(0, math.pi / 2 * spot.O, math.pi / 36)
						-- elseif spot.Name == "Rook" then
						-- 	-- goal.CFrame = origin * CFrame.new(-18.5 * spot.O, 0.306, -10 * spot.O) * CFrame.Angles(0, math.pi / 2 * spot.O, math.pi / 36)
						-- end
						-- tween:Create(spot.K, TweenInfo.new(0.5), goal):Play()
						-- wait(0.5)
					else
						cool = false
						return
					end
				end

				-- the old piece that we're moving
				local oldp = call.P
				local oldx = call.X

				-- promote depending on the player and row
				if promote == true and call.Name ~= "Gold" and call.Name ~= "King" and call.Name ~= "Jewel" then
					if call.O == Player.TWO and call.Y <= 3 then
						call.P = Rank.PROMOTED
					elseif call.O == Player.ONE and call.Y >= 7 then
						call.P = Rank.PROMOTED
					end
				end

				local goal = {}

				-- special animations for knight
				-- if oldx == 0 then
				-- 	-- goal.CFrame = origin * CFrame.new(-16 * call.O, 5, -7.5 * call.O) * CFrame.Angles(0, math.pi / 2 * call.O, math.pi / 36)
				-- 	-- tween:Create(call.K, TweenInfo.new(0.5), goal):Play()
				-- 	-- wait(0.75)
				-- elseif string.sub(pieceName, 1, 6) == "Knight" and call.P == 1 then
				-- 	-- goal.CFrame = origin * CFrame.new(12.5 - 2.5 * call.X, 5, 2.5 * call.Y - 12.5) * CFrame.Angles(0, math.pi / 2 * call.O, math.pi / 36)
				-- 	-- tween:Create(call.K, TweenInfo.new(0.5), goal):Play()
				-- 	-- wait(0.75)
				-- end

				call.X = x
				call.Y = y
				if promote == true and call.K.Name ~= "Gold" and call.K.Name ~= "King" and call.K.Name ~= "Jewel" then
					if call.O == -1 and call.Y <= 3 then
						call.P = 2
					elseif call.O == 1 and call.Y >= 7 then
						call.P = 2
					end
				end

				-- if oldp == 2 then
				-- 	goal.CFrame = origin * CFrame.new(12.5 - 2.5 * call.X, 1.6, 2.5 * call.Y - 12.5) * CFrame.Angles(0, -math.pi / 2 * call.O, math.pi)
				-- else
				-- 	goal.CFrame = origin * CFrame.new(12.5 - 2.5 * call.X, 1.6, 2.5 * call.Y - 12.5) * CFrame.Angles(0, math.pi / 2 * call.O, math.pi / 36)
				-- end

				-- tween:Create(call.K, TweenInfo.new(0.5), goal):Play()
				-- wait(0.5)

				-- if oldp == 1 and call.P == 2 then
				-- 	wait(0.25)
				-- 	goal.CFrame = origin * CFrame.new(12.5 - 2.5 * call.X, 5, 2.5 * call.Y - 12.5) * CFrame.Angles(0, -math.pi / 2 * call.O, math.pi)
				-- 	tween:Create(call.K, TweenInfo.new(0.5), goal):Play()
				-- 	wait(0.75)
				-- 	goal.CFrame = origin * CFrame.new(12.5 - 2.5 * call.X, 5, 2.5 * call.Y - 12.5) * CFrame.Angles(0, -math.pi / 2 * call.O, math.pi)
				-- 	tween:Create(call.K, TweenInfo.new(0.25), goal):Play()
				-- 	wait(0.25)
				-- 	goal.CFrame = origin * CFrame.new(12.5 - 2.5 * call.X, 1.6, 2.5 * call.Y - 12.5) * CFrame.Angles(0, -math.pi / 2 * call.O, math.pi)
				-- 	tween:Create(call.K, TweenInfo.new(0.25), goal):Play()
				-- 	wait(0.25)
				-- end

				turn = turn * -1

				local addon = ""

				local player = turn / 2 + 1.5

				local kingPiece = registry[player]["King"]
				local check = check(kingPiece)

				if check == true then
					addon = "Check - "
				end

				-- update text for player's turn
				-- board.BillboardGui.TextLabel.Text = addon .. "Player " .. player .. "'s Turn"
				-- board.Selected.Position = Vector3.new(0, -3.2, -17.5 * turn) + origin.p

				local numofmoves = 0
				for piece in pieces do
					if piece.O == turn then
						local available = moves(piece.Name)
						numofmoves = numofmoves + #available
					end
				end

				if numofmoves == 0 then
					-- create toast for player winning and losing
					-- board.BillboardGui.TextLabel.Text = "Checkmate - Player " .. player .. " Wins"
					running = false
				end
			end
		end
		cool = false
	end

	-- what is delay?
	-- delay(0, function()
	-- seems to be the core gameloop

	while running do
		local hashmap = {}
		for piece in pieces do
			hashmap[piece.X][piece.Y] = piece;
		end

		local bot = false

		if bot1 == true and turn == -1 then
			bot = true
		elseif bot2 == true and turn == 1 then
			bot = true
		end

		-- if we've enabled a bot
		if bot == true then
			local piecetomove = {}
			local move = {}
			local movevalue = math.huge * -1

			-- iterate over every piece
			for pieceIdx, piece in pairs(pieces) do
				-- if it's our piece
				if piece.O == turn then
					-- check the available moves
					for availableIdx, available in pairs(moves(piece)) do
						local moveworth = 0
						local indanger = check(piece)

						-- if we're in check then (probably) decide on the
						-- value of pieces to sacrifice
						if indanger == true then
							if piece.P == Player.ONE then
								if piece.Name == "Pawn" then
									moveworth = moveworth + 1
								elseif piece.Name == "Knight" then
									moveworth = moveworth + 4
								elseif piece.Name == "Lance" then
									moveworth = moveworth + 6
								elseif piece.Name == "Silver" then
									moveworth = moveworth + 9
								elseif piece.Name == "Gold" then
									moveworth = moveworth + 10
								elseif piece.Name == "Bishop" then
									moveworth = moveworth + 15
								elseif piece.Name == "Rook" then
									moveworth = moveworth + 16
								end
							else
								if piece.Name == "Pawn" then
									moveworth = moveworth + 8
								elseif piece.Name == "Knight" then
									moveworth = moveworth + 8
								elseif piece.Name == "Lance" then
									moveworth = moveworth + 8
								elseif piece.Name == "Silver" then
									moveworth = moveworth + 9
								elseif piece.Name == "Bishop" then
									moveworth = moveworth + 25
								elseif piece.Name == "Rook" then
									moveworth = moveworth + 27
								end
							end
						end

						-- copy over table
						local newtable = {}
						for piece in pieces do
							table.insert(newtable, {
								X = piece.X,
								Y = piece.Y,
								P = piece.P,
								O = piece.O
							})
						end

						-- check if we're moving into a spot that has an opponent's piece
						local spot = hashmap(available, newtable)
						if spot then
							if spot.O ~= piece.O then
								if spot.P == 1 then
									if spot.Name == "Pawn" then
										moveworth = moveworth + 1
									elseif spot.Name == "Knight" then
										moveworth = moveworth + 4
									elseif spot.Name == "Lance" then
										moveworth = moveworth + 6
									elseif spot.Name == "Silver" then
										moveworth = moveworth + 9
									elseif spot.Name == "Gold" then
										moveworth = moveworth + 10
									elseif spot.Name == "Bishop" then
										moveworth = moveworth + 15
									elseif spot.Name == "Rook" then
										moveworth = moveworth + 16
									end
								else
									if spot.Name == "Pawn" then
										moveworth = moveworth + 8
									elseif spot.Name == "Knight" then
										moveworth = moveworth + 8
									elseif spot.Name == "Lance" then
										moveworth = moveworth + 8
									elseif spot.Name == "Silver" then
										moveworth = moveworth + 9
									elseif spot.Name == "Bishop" then
										moveworth = moveworth + 25
									elseif spot.Name == "Rook" then
										moveworth = moveworth + 27
									end
								end
								-- if so, move the piece to our posession
								spot.X = 0
								spot.Y = 0
								spot.P = 1
								spot.O = piece.O
							end
						end
						-- update that piece's location to its new available location
						newtable[pieceIdx].X = available[availableIdx].X
						newtable[pieceIdx].Y = available[availableIdx].Y

						local check = false
						for tPieceIdx, tPiece in pairs(pieces) do
							-- if the piece is not owned by the player
							if tPiece.O == piece.O * -1 then
								for spot in findavailable(tPieceIdx, newtable) do
									if spot.X == newtable[pieceIdx].X and spot.Y == newtable[pieceIdx].Y then
										check = true
									end
								end
							end
						end

						if check == true then
							if piece.P == 1 then
								if piece.Name == "Pawn" then
									moveworth = moveworth - 1
								elseif piece.Name == "Knight" then
									moveworth = moveworth - 4
								elseif piece.Name == "Lance" then
									moveworth = moveworth - 6
								elseif piece.Name == "Silver" then
									moveworth = moveworth - 9
								elseif piece.Name == "Gold" then
									moveworth = moveworth - 10
								elseif piece.Name == "Bishop" then
									moveworth = moveworth - 15
								elseif piece.Name == "Rook" then
									moveworth = moveworth - 16
								end
							else
								if piece.Name == "Pawn" then
									moveworth = moveworth - 8
								elseif piece.Name == "Knight" then
									moveworth = moveworth - 8
								elseif piece.Name == "Lance" then
									moveworth = moveworth - 8
								elseif piece.Name == "Silver" then
									moveworth = moveworth - 9
								elseif piece.Name == "Bishop" then
									moveworth = moveworth - 25
								elseif piece.Name == "Rook" then
									moveworth = moveworth - 27
								end
							end
						end

						if moveworth == movevalue then
							movevalue = moveworth
							local num = #piecetomove + 1
							piecetomove[num] = pieceIdx
							move[num] = available[availableIdx]
						elseif moveworth > movevalue then
							movevalue = moveworth
							local num = 1
							piecetomove = {}
							move = {}
							piecetomove[num] = pieceIdx
							move[num] = available[availableIdx]
						end
					end
				end
			end
			if #piecetomove >= 1 then
				local num = math.random(1, #piecetomove)
				movepiece(nil, piecetomove[num], move[num].X, move[num].Y, true)
			end
		end
		-- wait(0.1)
	end
	-- end)

	-- game.ReplicatedStorage.RemoteEvent.OnServerEvent:Connect(function(plr, piece, x, y, promote)
	-- 	if running == true then return end
	-- 	if plr ~= player1 and turn == -1 then return end
	-- 	if plr ~= player2 and turn == 1 then return end
	-- 	movepiece(plr, piece, x, y, promote)
	-- end)

	local plr1time = 0
	-- label board
	local timeLabelOne = "Time: " .. math.floor(plr1time)
	-- board.Timer1.SurfaceGui.TextLabel.Text = "Time: " .. math.floor(plr1time)
	local plr2time = 0
	local timeLabelTwo = "Time: " .. math.floor(plr2time)
	-- board.Timer2.SurfaceGui.TextLabel.Text = "Time: " .. math.floor(plr2time)

	repeat
		-- wait(0.1)
		if turn == -1 then
			-- if player1 ~= nil or bot1 == true then
			plr1time = plr1time + 0.1
			timeLabelOne = "Time: " .. math.floor(plr1time)
			-- end
		elseif turn == 1 then
			-- if player2 ~= nil or bot2 == true then
			plr2time = plr2time + 0.1
			timeLabelTwo = "Time: " .. math.floor(plr2time)
			-- end
		end
	until running == false
end

-- local chil = game.Workspace.Boards:GetChildren()
-- for i = 1, #chil do
-- 	delay(0, function()
-- 		while true do
-- setup(chil[i])
-- 		end
-- 	end)
-- end

setup()
