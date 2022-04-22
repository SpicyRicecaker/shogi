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

-- sets up pieces for both players
local function setup(board)
	-- seems to be a 3d coordinate of some sort
	-- local origin = CFrame.new(board.Position)

	-- an array of all the pieces on the board
	-- except we're using a dictionary
	local pieces = getNewBoard()
	-- variable that stores if the game is not over
	local gameended = false

	-- local player1 = nil
	-- board.Pillow1.BillboardGui.TextLabel.Text = "Empty"
	-- local bot1 = false
	-- board.Bot1.Color = Color3.new(1, 0, 0)
	-- board.Bot1.SurfaceGui.TextLabel.Text = "Bot: Off"

	-- local player2 = nil
	-- board.Pillow2.BillboardGui.TextLabel.Text = "Empty"
	-- local bot2 = false
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
	local function findavailable(pieceName, newtable)
		-- board of current pieces
		local pieces = newtable or pieces

		-- location of piece
		local call = pieces[pieceName]
		-- we should never need this
		if call == nil then return end

		local available = {}

		-- hashmap
		local hashmap = {}
		for piece in pieces do
			hashmap[piece.X][piece.Y] = piece;
		end

		-- potential bug: if it's pawn then there's literally no available squares
		-- if we've clicked outside of the arena?
		if call.X == 0 or call.Y == 0 then
			-- if we're placing a pawn somewhere on the board, check if there is a pawn in that column already
			for h = 1, 9 do
				local pawn = false
				if pieceName == "Pawn" then
					for v = 1, 9 do
						-- tab is all coordinates of the board
						-- if there is a spot on the table for this location
						local spot = hashmap[h][v]
						if spot then
							if spot.O == call.O and spot.Name == "Pawn" and spot.P == 1 then
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
						-- we can only send knight up to third to last rank
						-- we can only send all other peaces up second to last rank
						if not spot then
							if pieceName == "Knight" then
								if call.O * 3 + 5 ~= v then
									table.insert(available, { X = h, Y = v })
								end
							else
								if call.O * 4 + 5 ~= v then
									table.insert(available, { X = h, Y = v })
								end
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
				if call.P == Rank.NORMAL then
					table.insert(dydxList, { -1 * call.O, 0 })
				else
					for y = -1, 1 do
						for x = -1, 1 do
							if ~(y == -1 and x == -1) and ~(y == -1 and x == 1) then
								table.insert(dydxList, { y * call.O, x })
							end
						end
					end
				end
				-- for lance, dynamically generate a list of dydx at runtime
			elseif pieceName == "Lance" then
				if call.P == Rank.NORMAL then
					if call.O == Player.ONE then
						-- might be a bug, we'll see
						for y = call.Y + 1, 9, 1 do
							table.insert(dydxList, { y - call.Y, 0 })
						end
					else
						for y = call.Y - 1, 1, -1 do
							table.insert(dydxList, { call.Y - y, 0 })
						end
					end
				else
					-- gold movement
					for y in -1, 1 do
						for x in -1, 1 do
							if ~(y == -1 and x == -1) and ~(y == -1 and x == 1) then
								table.insert(dydxList, { y * call.O, x })
							end
						end
					end
				end
				-- if it's knight promote to gold
			elseif pieceName == "Knight" then
				if call.P == Rank.NORMAL then
					for x in { -1, 1 } do
						table.insert(dydxList, { 2 * call.O, x })
					end
				else
					for y in -1, 1 do
						for x in -1, 1 do
							if ~(y == -1 and x == -1) and ~(y == -1 and x == 1) then
								table.insert(dydxList, { y * call.O, x })
							end
						end
					end
				end
				-- if it's silver promote to gold
			elseif pieceName == "Silver" then
				if call.P == Rank.NORMAL then
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
						table.insert(dydxList, { coordinate[0] * call.O, coordinate[1] })
					end
				else
					for y in -1, 1 do
						for x in -1, 1 do
							if ~(y == -1 and x == -1) and ~(y == -1 and x == 1) then
								table.insert(dydxList, { y * call.O, x })
							end
						end
					end
				end
				-- if it's gold promote to gold
			elseif pieceName == "Gold" then
				for y in -1, 1 do
					for x in -1, 1 do
						if ~(y == -1 and x == -1) and ~(y == -1 and x == 1) then
							table.insert(dydxList, { y * call.O, x })
						end
					end
				end
				-- if it's gold promote to gold
			elseif pieceName == "King" then
				for y in -1, 1 do
					for x in -1, 1 do
						table.insert(dydxList, { y * call.O, x })
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
					local currentX = call.X + dx
					local currentY = call.Y + dy
					-- get position of rook in this direction
					while currentX >= 1 and currentX <= 9 and currentY >= 1 and currentY <= 9 do
						local spot = hashmap[currentX][currentY]

						-- if there is a piece in the way, our rook cannot move further
						if spot then
							-- check ownership
							-- if we own, do not include the spot as a valid move
							if spot.O == call.O then
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

				if call.P == Rank.PROMOTED then
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
						table.insert(dydxList, { coordinate[0] * call.O, coordinate[1] })
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
					local currentX = call.X + dx
					local currentY = call.Y + dy
					-- get position of bishop in this direction
					while currentX >= 1 and currentX <= 9 and currentY >= 1 and currentY <= 9 do
						local spot = hashmap[currentX][currentY]

						-- if there is a piece in the way, our bishop cannot move further
						if spot then
							-- check ownership
							-- if we own, do not include the spot as a valid move
							if spot.O ~= call.O then
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

				if call.P == Rank.PROMOTED then
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
						table.insert(dydxList, { coordinate[0] * call.O, coordinate[1] })
					end
				end
			end

			for dy, dx in dydxList do
				local newY = call.Y + dy
				local newX = call.X + dx

				if ~(newY > 9 or newY < 1) and ~(newX > 9 or newX < 1) then
					local spot = hashmap[newX][newY]
					if spot then
						if spot.O ~= call.O then
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

	local function check(piece, newtable)
		local pieces = newtable or pieces
		local call = pieces[piece]
		if call == nil then return end
		local check = false
		for i, v in pairs(pieces) do
			if v.O == call.O * -1 then
				local spot = findavailable(i, newtable)
				for c = 1, #spot do
					if spot[c].X == call.X and spot[c].Y == call.Y then
						check = true
					end
				end
			end
		end
		return check
	end

	local function moves(piece, newtable)
		local pieces = newtable or pieces
		local call = pieces[piece]
		if call == nil then return end
		local newavailable = {}
		local available = findavailable(piece)
		for i = 1, #available do
			local newtable = {}
			for i, v in pairs(pieces) do
				newtable[i] = {}
				newtable[i].K = v.K
				newtable[i].X = v.X
				newtable[i].Y = v.Y
				newtable[i].P = v.P
				newtable[i].O = v.O
			end
			local spot = piece_exists_at(available[i], newtable)
			if spot then
				if spot.O ~= call.O then
					spot.X = 0
					spot.Y = 0
					spot.P = 1
					spot.O = call.O
				end
			end
			newtable[piece].X = available[i].X
			newtable[piece].Y = available[i].Y
			local check1 = nil
			if call.O == 1 then
				check1 = check("King1", newtable)
			elseif call.O == -1 then
				check1 = check("King2", newtable)
			end
			if check1 == false then
				newavailable[#newavailable + 1] = available[i]
			end
		end
		return newavailable
	end

	for i, v in pairs(pieces) do
		v.K.Parent = game.Workspace.Pieces
		v.K.CFrame = origin * CFrame.new(12.5 - 2.5 * v.X, 1.6, 2.5 * v.Y - 12.5) * CFrame.Angles(0, math.pi / 2 * v.O, math.pi / 36)
		v.K.ClickDetector.MouseClick:Connect(function(player)
			if cool == true then return end
			if gameended == true then return end
			if v.O == turn then
				local available = moves(i)
				game.ReplicatedStorage.RemoteEvent:FireClient(player, i, "place", available, v, board)
			else
				game.ReplicatedStorage.RemoteEvent:FireClient(player, nil, "attack", v)
			end
		end)
	end

	local function movepiece(plr, piece, x, y, promote)
		if gameended == true then return end
		local call = pieces[piece]
		if call == nil then return end
		if turn ~= call.O then return end
		if cool == true then return end
		cool = true
		if call.X == 0 or call.Y == 0 then
			promote = false
		end
		local available = moves(piece)
		for i = 1, #available do
			if available[i].X == x and available[i].Y == y then
				local spot = piece_exists_at(available[i])
				if spot then
					if spot.O ~= call.O then
						local goal = {}
						goal.CFrame = origin * CFrame.new(12.5 - 2.5 * spot.X, 5, 2.5 * spot.Y - 12.5) * CFrame.Angles(0, math.pi / 2 * spot.O, math.pi / 36)
						tween:Create(spot.K, TweenInfo.new(0.5), goal):Play()
						spot.X = 0
						spot.Y = 0
						spot.P = 1
						spot.O = call.O
						wait(0.75)
						goal.CFrame = origin * CFrame.new(-16 * spot.O, 5, -7.5 * spot.O) * CFrame.Angles(0, math.pi / 2 * spot.O, math.pi / 36)
						tween:Create(spot.K, TweenInfo.new(0.5), goal):Play()
						wait(0.5)
						if spot.K.Name == "Pawn" then
							local ran = math.random(1, 3) - 2
							goal.CFrame = origin * CFrame.new((-16 + ran * 2.5) * spot.O, 0.306, -5 * spot.O) * CFrame.Angles(0, math.pi / 2 * spot.O, math.pi / 36)
						elseif spot.K.Name == "Lance" then
							goal.CFrame = origin * CFrame.new(-13.5 * spot.O, 0.306, -7.5 * spot.O) * CFrame.Angles(0, math.pi / 2 * spot.O, math.pi / 36)
						elseif spot.K.Name == "Knight" then
							goal.CFrame = origin * CFrame.new(-16 * spot.O, 0.306, -7.5 * spot.O) * CFrame.Angles(0, math.pi / 2 * spot.O, math.pi / 36)
						elseif spot.K.Name == "Silver" then
							goal.CFrame = origin * CFrame.new(-18.5 * spot.O, 0.306, -7.5 * spot.O) * CFrame.Angles(0, math.pi / 2 * spot.O, math.pi / 36)
						elseif spot.K.Name == "Gold" then
							goal.CFrame = origin * CFrame.new(-13.5 * spot.O, 0.306, -10 * spot.O) * CFrame.Angles(0, math.pi / 2 * spot.O, math.pi / 36)
						elseif spot.K.Name == "Bishop" then
							goal.CFrame = origin * CFrame.new(-16 * spot.O, 0.306, -10 * spot.O) * CFrame.Angles(0, math.pi / 2 * spot.O, math.pi / 36)
						elseif spot.K.Name == "Rook" then
							goal.CFrame = origin * CFrame.new(-18.5 * spot.O, 0.306, -10 * spot.O) * CFrame.Angles(0, math.pi / 2 * spot.O, math.pi / 36)
						end
						tween:Create(spot.K, TweenInfo.new(0.5), goal):Play()
						wait(0.5)
					else
						cool = false
						return
					end
				end
				local oldp = call.P
				local oldx = call.X
				if promote == true and call.K.Name ~= "Gold" and call.K.Name ~= "King" and call.K.Name ~= "Jewel" then
					if call.O == -1 and call.Y <= 3 then
						call.P = 2
					elseif call.O == 1 and call.Y >= 7 then
						call.P = 2
					end
				end
				local goal = {}
				if oldx == 0 then
					goal.CFrame = origin * CFrame.new(-16 * call.O, 5, -7.5 * call.O) * CFrame.Angles(0, math.pi / 2 * call.O, math.pi / 36)
					tween:Create(call.K, TweenInfo.new(0.5), goal):Play()
					wait(0.75)
				elseif string.sub(piece, 1, 6) == "Knight" and call.P == 1 then
					goal.CFrame = origin * CFrame.new(12.5 - 2.5 * call.X, 5, 2.5 * call.Y - 12.5) * CFrame.Angles(0, math.pi / 2 * call.O, math.pi / 36)
					tween:Create(call.K, TweenInfo.new(0.5), goal):Play()
					wait(0.75)
				end
				call.X = x
				call.Y = y
				if promote == true and call.K.Name ~= "Gold" and call.K.Name ~= "King" and call.K.Name ~= "Jewel" then
					if call.O == -1 and call.Y <= 3 then
						call.P = 2
					elseif call.O == 1 and call.Y >= 7 then
						call.P = 2
					end
				end
				if oldp == 2 then
					goal.CFrame = origin * CFrame.new(12.5 - 2.5 * call.X, 1.6, 2.5 * call.Y - 12.5) * CFrame.Angles(0, -math.pi / 2 * call.O, math.pi)
				else
					goal.CFrame = origin * CFrame.new(12.5 - 2.5 * call.X, 1.6, 2.5 * call.Y - 12.5) * CFrame.Angles(0, math.pi / 2 * call.O, math.pi / 36)
				end
				tween:Create(call.K, TweenInfo.new(0.5), goal):Play()
				wait(0.5)
				if oldp == 1 and call.P == 2 then
					wait(0.25)
					goal.CFrame = origin * CFrame.new(12.5 - 2.5 * call.X, 5, 2.5 * call.Y - 12.5) * CFrame.Angles(0, -math.pi / 2 * call.O, math.pi)
					tween:Create(call.K, TweenInfo.new(0.5), goal):Play()
					wait(0.75)
					goal.CFrame = origin * CFrame.new(12.5 - 2.5 * call.X, 5, 2.5 * call.Y - 12.5) * CFrame.Angles(0, -math.pi / 2 * call.O, math.pi)
					tween:Create(call.K, TweenInfo.new(0.25), goal):Play()
					wait(0.25)
					goal.CFrame = origin * CFrame.new(12.5 - 2.5 * call.X, 1.6, 2.5 * call.Y - 12.5) * CFrame.Angles(0, -math.pi / 2 * call.O, math.pi)
					tween:Create(call.K, TweenInfo.new(0.25), goal):Play()
					wait(0.25)
				end
				turn = turn * -1
				local addon = ""
				local player = turn / 2 + 1.5
				local check = check("King" .. player)
				if check == true then
					addon = "Check - "
				end
				board.BillboardGui.TextLabel.Text = addon .. "Player " .. player .. "'s Turn"
				board.Selected.Position = Vector3.new(0, -3.2, -17.5 * turn) + origin.p
				local numofmoves = 0
				for i, v in pairs(pieces) do
					if v.O == turn then
						local available = moves(i)
						numofmoves = numofmoves + #available
					end
				end
				if numofmoves == 0 then
					board.BillboardGui.TextLabel.Text = "Checkmate - Player " .. player .. " Wins"
					gameended = true
				end
			end
		end
		cool = false
	end

	delay(0, function()
		while gameended == false do
			local bot = false
			if bot1 == true and turn == -1 then
				bot = true
			elseif bot2 == true and turn == 1 then
				bot = true
			end
			if bot == true then
				local piecetomove = {}
				local move = {}
				local movevalue = math.huge * -1
				for i, v in pairs(pieces) do
					if v.O == turn then
						local available = moves(i)
						for c = 1, #available do
							local moveworth = 0
							local indanger = check(i)
							if indanger == true then
								if v.P == 1 then
									if v.K.Name == "Pawn" then
										moveworth = moveworth + 1
									elseif v.K.Name == "Knight" then
										moveworth = moveworth + 4
									elseif v.K.Name == "Lance" then
										moveworth = moveworth + 6
									elseif v.K.Name == "Silver" then
										moveworth = moveworth + 9
									elseif v.K.Name == "Gold" then
										moveworth = moveworth + 10
									elseif v.K.Name == "Bishop" then
										moveworth = moveworth + 15
									elseif v.K.Name == "Rook" then
										moveworth = moveworth + 16
									end
								else
									if v.K.Name == "Pawn" then
										moveworth = moveworth + 8
									elseif v.K.Name == "Knight" then
										moveworth = moveworth + 8
									elseif v.K.Name == "Lance" then
										moveworth = moveworth + 8
									elseif v.K.Name == "Silver" then
										moveworth = moveworth + 9
									elseif v.K.Name == "Bishop" then
										moveworth = moveworth + 25
									elseif v.K.Name == "Rook" then
										moveworth = moveworth + 27
									end
								end
							end
							local newtable = {}
							for i, v in pairs(pieces) do
								newtable[i] = {}
								newtable[i].K = v.K
								newtable[i].X = v.X
								newtable[i].Y = v.Y
								newtable[i].P = v.P
								newtable[i].O = v.O
							end
							local spot = piece_exists_at(available[c], newtable)
							if spot then
								if spot.O ~= v.O then
									if spot.P == 1 then
										if spot.K.Name == "Pawn" then
											moveworth = moveworth + 1
										elseif spot.K.Name == "Knight" then
											moveworth = moveworth + 4
										elseif spot.K.Name == "Lance" then
											moveworth = moveworth + 6
										elseif spot.K.Name == "Silver" then
											moveworth = moveworth + 9
										elseif spot.K.Name == "Gold" then
											moveworth = moveworth + 10
										elseif spot.K.Name == "Bishop" then
											moveworth = moveworth + 15
										elseif spot.K.Name == "Rook" then
											moveworth = moveworth + 16
										end
									else
										if spot.K.Name == "Pawn" then
											moveworth = moveworth + 8
										elseif spot.K.Name == "Knight" then
											moveworth = moveworth + 8
										elseif spot.K.Name == "Lance" then
											moveworth = moveworth + 8
										elseif spot.K.Name == "Silver" then
											moveworth = moveworth + 9
										elseif spot.K.Name == "Bishop" then
											moveworth = moveworth + 25
										elseif spot.K.Name == "Rook" then
											moveworth = moveworth + 27
										end
									end
									spot.X = 0
									spot.Y = 0
									spot.P = 1
									spot.O = v.O
								end
							end
							newtable[i].X = available[c].X
							newtable[i].Y = available[c].Y
							wait()
							local check = false
							for b, n in pairs(pieces) do
								if n.O == v.O * -1 then
									local spot = findavailable(b, newtable)
									for c = 1, #spot do
										if spot[c].X == newtable[i].X and spot[c].Y == newtable[i].Y then
											check = true
										end
									end
								end
							end
							if check == true then
								if v.P == 1 then
									if v.K.Name == "Pawn" then
										moveworth = moveworth - 1
									elseif v.K.Name == "Knight" then
										moveworth = moveworth - 4
									elseif v.K.Name == "Lance" then
										moveworth = moveworth - 6
									elseif v.K.Name == "Silver" then
										moveworth = moveworth - 9
									elseif v.K.Name == "Gold" then
										moveworth = moveworth - 10
									elseif v.K.Name == "Bishop" then
										moveworth = moveworth - 15
									elseif v.K.Name == "Rook" then
										moveworth = moveworth - 16
									end
								else
									if v.K.Name == "Pawn" then
										moveworth = moveworth - 8
									elseif v.K.Name == "Knight" then
										moveworth = moveworth - 8
									elseif v.K.Name == "Lance" then
										moveworth = moveworth - 8
									elseif v.K.Name == "Silver" then
										moveworth = moveworth - 9
									elseif v.K.Name == "Bishop" then
										moveworth = moveworth - 25
									elseif v.K.Name == "Rook" then
										moveworth = moveworth - 27
									end
								end
							end
							if moveworth == movevalue then
								movevalue = moveworth
								local num = #piecetomove + 1
								piecetomove[num] = i
								move[num] = available[c]
							elseif moveworth > movevalue then
								movevalue = moveworth
								local num = 1
								piecetomove = {}
								move = {}
								piecetomove[num] = i
								move[num] = available[c]
							end
						end
					end
				end
				if #piecetomove >= 1 then
					local num = math.random(1, #piecetomove)
					movepiece(nil, piecetomove[num], move[num].X, move[num].Y, true)
				end
			end
			wait(0.1)
		end
	end)
	game.ReplicatedStorage.RemoteEvent.OnServerEvent:Connect(function(plr, piece, x, y, promote)
		if gameended == true then return end
		if plr ~= player1 and turn == -1 then return end
		if plr ~= player2 and turn == 1 then return end
		movepiece(plr, piece, x, y, promote)
	end)
	local plr1time = 0
	board.Timer1.SurfaceGui.TextLabel.Text = "Time: " .. math.floor(plr1time)
	local plr2time = 0
	board.Timer2.SurfaceGui.TextLabel.Text = "Time: " .. math.floor(plr2time)
	repeat
		wait(0.1)
		if turn == -1 then
			if player1 ~= nil or bot1 == true then
				plr1time = plr1time + 0.1
				board.Timer1.SurfaceGui.TextLabel.Text = "Time: " .. math.floor(plr1time)
			end
		elseif turn == 1 then
			if player2 ~= nil or bot2 == true then
				plr2time = plr2time + 0.1
				board.Timer2.SurfaceGui.TextLabel.Text = "Time: " .. math.floor(plr2time)
			end
		end
	until gameended == true
	wait(5)
	for i, v in pairs(pieces) do
		v.K:Destroy()
	end
end

local chil = game.Workspace.Boards:GetChildren()
for i = 1, #chil do
	delay(0, function()
		while true do
			setup(chil[i])
		end
	end)
end
