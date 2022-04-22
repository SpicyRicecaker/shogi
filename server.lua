Player = {
	ONE = 1,
	TWO = -1
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
	local function something(location, newtable)
		local pieces = newtable or pieces
		for _, v in pairs(pieces) do
			if v.X == location.X and v.Y == location.Y then
				return v
			end
		end
	end

	-- probably find all places we can move
	local function findavailable(pieceName, newtable)
		-- board of current pieces
		local pieces = newtable or pieces

		-- location of piece
		local call = pieces[pieceName]
		-- we should never need this
		if call == nil then return end

		local available = {}
		-- if we've clicked outside of the arena?
		if call.X == 0 or call.Y == 0 then
			-- if we're placing a pawn somewhere on the board, check if there is a pawn in that column already
			for h = 1, 9 do
				local pawn = false
				if string.sub(pieceName, 1, 4) == "Pawn" then
					for v = 1, 9 do
						-- tab is all coordinates of the board
						local location = {}
						location.X = h
						location.Y = v

						-- if there is a spot on the table for this location
						local spot = something(location, newtable)

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
						local num = #available + 1
						available[num] = {
							X = h,
							Y = v
						}

						local spot = something(available[num], newtable)

                        -- unless there's a piece already on there
						if spot then
							available[num] = nil
						elseif string.sub(pieceName, 1, 4) == "Pawn" or string.sub(pieceName, 1, 5) == "Lance" or string.sub(pieceName, 1, 6) == "Knight" then
							-- ????????????
							if call.O * 4 + 5 == v then
								available[num] = nil
							elseif call.O * 3 + 5 == v and string.sub(pieceName, 1, 6) == "Knight" then
								available[num] = nil
							end
						end
					end
				end
			end
		else
			-- promotion?
			local pawn = false
			local front = false
			local knight = false
			local silver = false
			local gold = false
			local lance = false
			local rook = false
			local bishop = false
			-- if it's pawn promote to gold
			if string.sub(pieceName, 1, 4) == "Pawn" then
				pawn = true
				if call.P == 2 then
					front = true
					gold = true
				end
			-- if it's lance promote to gold
			elseif string.sub(pieceName, 1, 5) == "Lance" then
				if call.P == 2 then
					pawn = true
					front = true
					gold = true
				else
					lance = true
				end
			-- if it's knight promote to gold
			elseif string.sub(pieceName, 1, 6) == "Knight" then
				if call.P == 2 then
					pawn = true
					front = true
					gold = true
				else
					knight = true
				end
			-- if it's silver promote to gold
			elseif string.sub(pieceName, 1, 6) == "Silver" then
				pawn = true
				front = true
				if call.P == 2 then
					gold = true
				else
					silver = true
				end
			-- if it's gold promote to gold
			elseif string.sub(pieceName, 1, 4) == "Gold" then
				pawn = true
				front = true
				gold = true
			-- if it's gold promote to gold
			elseif string.sub(pieceName, 1, 4) == "King" then
				pawn = true
				front = true
				gold = true
				silver = true
			elseif string.sub(pieceName, 1, 4) == "Rook" then
				lance = true
				rook = true
				if call.P == 2 then
					front = true
					silver = true
				end
			elseif string.sub(pieceName, 1, 6) == "Bishop" then
				bishop = true
				if call.P == 2 then
					pawn = true
					gold = true
				end
			end
			if pawn == true then
				local num = #available + 1
				available[num] = {}
				available[num].X = call.X
				available[num].Y = call.Y + 1 * call.O
				if available[num].X > 9 or available[num].X < 1 or available[num].Y > 9 or available[num].Y < 1 then
					available[num] = nil
				else
					local spot = something(available[num], newtable)
					if spot then
						if spot.O == call.O then
							available[num] = nil
						end
					end
				end
			end
			if front == true then
				local num = #available + 1
				available[num] = {}
				available[num].X = call.X - 1
				available[num].Y = call.Y + 1 * call.O
				if available[num].X > 9 or available[num].X < 1 or available[num].Y > 9 or available[num].Y < 1 then
					available[num] = nil
				else
					local spot = something(available[num], newtable)
					if spot then
						if spot.O == call.O then
							available[num] = nil
						end
					end
				end
				local num = #available + 1
				available[num] = {}
				available[num].X = call.X + 1
				available[num].Y = call.Y + 1 * call.O
				if available[num].X > 9 or available[num].X < 1 or available[num].Y > 9 or available[num].Y < 1 then
					available[num] = nil
				else
					local spot = something(available[num], newtable)
					if spot then
						if spot.O == call.O then
							available[num] = nil
						end
					end
				end
			end
			if knight == true then
				local num = #available + 1
				available[num] = {}
				available[num].X = call.X - 1
				available[num].Y = call.Y + 2 * call.O
				if available[num].X > 9 or available[num].X < 1 or available[num].Y > 9 or available[num].Y < 1 then
					available[num] = nil
				else
					local spot = something(available[num], newtable)
					if spot then
						if spot.O == call.O then
							available[num] = nil
						end
					end
				end
				local num = #available + 1
				available[num] = {}
				available[num].X = call.X + 1
				available[num].Y = call.Y + 2 * call.O
				if available[num].X > 9 or available[num].X < 1 or available[num].Y > 9 or available[num].Y < 1 then
					available[num] = nil
				else
					local spot = something(available[num], newtable)
					if spot then
						if spot.O == call.O then
							available[num] = nil
						end
					end
				end
			end
			if silver == true then
				local num = #available + 1
				available[num] = {}
				available[num].X = call.X - 1
				available[num].Y = call.Y - 1 * call.O
				if available[num].X > 9 or available[num].X < 1 or available[num].Y > 9 or available[num].Y < 1 then
					available[num] = nil
				else
					local spot = something(available[num], newtable)
					if spot then
						if spot.O == call.O then
							available[num] = nil
						end
					end
				end
				local num = #available + 1
				available[num] = {}
				available[num].X = call.X + 1
				available[num].Y = call.Y - 1 * call.O
				if available[num].X > 9 or available[num].X < 1 or available[num].Y > 9 or available[num].Y < 1 then
					available[num] = nil
				else
					local spot = something(available[num], newtable)
					if spot then
						if spot.O == call.O then
							available[num] = nil
						end
					end
				end
			end
			if gold == true then
				local num = #available + 1
				available[num] = {}
				available[num].X = call.X - 1
				available[num].Y = call.Y
				if available[num].X > 9 or available[num].X < 1 or available[num].Y > 9 or available[num].Y < 1 then
					available[num] = nil
				else
					local spot = something(available[num], newtable)
					if spot then
						if spot.O == call.O then
							available[num] = nil
						end
					end
				end
				local num = #available + 1
				available[num] = {}
				available[num].X = call.X + 1
				available[num].Y = call.Y
				if available[num].X > 9 or available[num].X < 1 or available[num].Y > 9 or available[num].Y < 1 then
					available[num] = nil
				else
					local spot = something(available[num], newtable)
					if spot then
						if spot.O == call.O then
							available[num] = nil
						end
					end
				end
				local num = #available + 1
				available[num] = {}
				available[num].X = call.X
				available[num].Y = call.Y - 1 * call.O
				if available[num].X > 9 or available[num].X < 1 or available[num].Y > 9 or available[num].Y < 1 then
					available[num] = nil
				else
					local spot = something(available[num], newtable)
					if spot then
						if spot.O == call.O then
							available[num] = nil
						end
					end
				end
			end
			if lance == true then
				local hit = false
				for i = 1, 8 do
					if hit == false then
						local num = #available + 1
						available[num] = {}
						available[num].X = call.X
						available[num].Y = call.Y + i * call.O
						if available[num].X > 9 or available[num].X < 1 or available[num].Y > 9 or available[num].Y < 1 then
							available[num] = nil
						else
							local spot = something(available[num], newtable)
							if spot then
								hit = true
								if spot.O == call.O then
									available[num] = nil
								end
							end
						end
					end
				end
			end
			if rook == true then
				local hit = false
				for i = 1, 8 do
					if hit == false then
						local num = #available + 1
						available[num] = {}
						available[num].X = call.X
						available[num].Y = call.Y - i * call.O
						if available[num].X > 9 or available[num].X < 1 or available[num].Y > 9 or available[num].Y < 1 then
							available[num] = nil
						else
							local spot = something(available[num], newtable)
							if spot then
								hit = true
								if spot.O == call.O then
									available[num] = nil
								end
							end
						end
					end
				end
				local hit = false
				for i = 1, 8 do
					if hit == false then
						local num = #available + 1
						available[num] = {}
						available[num].X = call.X + i
						available[num].Y = call.Y
						if available[num].X > 9 or available[num].X < 1 or available[num].Y > 9 or available[num].Y < 1 then
							available[num] = nil
						else
							local spot = something(available[num], newtable)
							if spot then
								hit = true
								if spot.O == call.O then
									available[num] = nil
								end
							end
						end
					end
				end
				local hit = false
				for i = 1, 8 do
					if hit == false then
						local num = #available + 1
						available[num] = {}
						available[num].X = call.X - i
						available[num].Y = call.Y
						if available[num].X > 9 or available[num].X < 1 or available[num].Y > 9 or available[num].Y < 1 then
							available[num] = nil
						else
							local spot = something(available[num], newtable)
							if spot then
								hit = true
								if spot.O == call.O then
									available[num] = nil
								end
							end
						end
					end
				end
			end
			if bishop == true then
				local hit = false
				for i = 1, 8 do
					if hit == false then
						local num = #available + 1
						available[num] = {}
						available[num].X = call.X - i
						available[num].Y = call.Y - i * call.O
						if available[num].X > 9 or available[num].X < 1 or available[num].Y > 9 or available[num].Y < 1 then
							available[num] = nil
						else
							local spot = something(available[num], newtable)
							if spot then
								hit = true
								if spot.O == call.O then
									available[num] = nil
								end
							end
						end
					end
				end
				local hit = false
				for i = 1, 8 do
					if hit == false then
						local num = #available + 1
						available[num] = {}
						available[num].X = call.X + i
						available[num].Y = call.Y - i * call.O
						if available[num].X > 9 or available[num].X < 1 or available[num].Y > 9 or available[num].Y < 1 then
							available[num] = nil
						else
							local spot = something(available[num], newtable)
							if spot then
								hit = true
								if spot.O == call.O then
									available[num] = nil
								end
							end
						end
					end
				end
				local hit = false
				for i = 1, 8 do
					if hit == false then
						local num = #available + 1
						available[num] = {}
						available[num].X = call.X - i
						available[num].Y = call.Y + i * call.O
						if available[num].X > 9 or available[num].X < 1 or available[num].Y > 9 or available[num].Y < 1 then
							available[num] = nil
						else
							local spot = something(available[num], newtable)
							if spot then
								hit = true
								if spot.O == call.O then
									available[num] = nil
								end
							end
						end
					end
				end
				local hit = false
				for i = 1, 8 do
					if hit == false then
						local num = #available + 1
						available[num] = {}
						available[num].X = call.X + i
						available[num].Y = call.Y + i * call.O
						if available[num].X > 9 or available[num].X < 1 or available[num].Y > 9 or available[num].Y < 1 then
							available[num] = nil
						else
							local spot = something(available[num], newtable)
							if spot then
								hit = true
								if spot.O == call.O then
									available[num] = nil
								end
							end
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
			local spot = something(available[i], newtable)
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
				local spot = something(available[i])
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
							local spot = something(available[c], newtable)
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
