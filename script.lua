local tween = game:GetService("TweenService")
--player 1
local function setup(board)
	local origin = CFrame.new(board.Position)
	local pieces = {}
	local gameended = false
	pieces["King1"] = {}
	pieces["King1"].K = game.ReplicatedStorage.Pieces.King:Clone()
	pieces["King1"].X = 5
	pieces["King1"].Y = 1
	pieces["King1"].P = 1
	pieces["King1"].O = 1
	pieces["Gold1"] = {}
	pieces["Gold1"].K = game.ReplicatedStorage.Pieces.Gold:Clone()
	pieces["Gold1"].X = 4
	pieces["Gold1"].Y = 1
	pieces["Gold1"].P = 1
	pieces["Gold1"].O = 1
	pieces["Gold2"] = {}
	pieces["Gold2"].K = game.ReplicatedStorage.Pieces.Gold:Clone()
	pieces["Gold2"].X = 6
	pieces["Gold2"].Y = 1
	pieces["Gold2"].P = 1
	pieces["Gold2"].O = 1
	pieces["Silver1"] = {}
	pieces["Silver1"].K = game.ReplicatedStorage.Pieces.Silver:Clone()
	pieces["Silver1"].X = 3
	pieces["Silver1"].Y = 1
	pieces["Silver1"].P = 1
	pieces["Silver1"].O = 1
	pieces["Silver2"] = {}
	pieces["Silver2"].K = game.ReplicatedStorage.Pieces.Silver:Clone()
	pieces["Silver2"].X = 7
	pieces["Silver2"].Y = 1
	pieces["Silver2"].P = 1
	pieces["Silver2"].O = 1
	pieces["Knight1"] = {}
	pieces["Knight1"].K = game.ReplicatedStorage.Pieces.Knight:Clone()
	pieces["Knight1"].X = 2
	pieces["Knight1"].Y = 1
	pieces["Knight1"].P = 1
	pieces["Knight1"].O = 1
	pieces["Knight2"] = {}
	pieces["Knight2"].K = game.ReplicatedStorage.Pieces.Knight:Clone()
	pieces["Knight2"].X = 8
	pieces["Knight2"].Y = 1
	pieces["Knight2"].P = 1
	pieces["Knight2"].O = 1
	pieces["Lance1"] = {}
	pieces["Lance1"].K = game.ReplicatedStorage.Pieces.Lance:Clone()
	pieces["Lance1"].X = 1
	pieces["Lance1"].Y = 1
	pieces["Lance1"].P = 1
	pieces["Lance1"].O = 1
	pieces["Lance2"] = {}
	pieces["Lance2"].K = game.ReplicatedStorage.Pieces.Lance:Clone()
	pieces["Lance2"].X = 9
	pieces["Lance2"].Y = 1
	pieces["Lance2"].P = 1
	pieces["Lance2"].O = 1
	pieces["Rook1"] = {}
	pieces["Rook1"].K = game.ReplicatedStorage.Pieces.Rook:Clone()
	pieces["Rook1"].X = 8
	pieces["Rook1"].Y = 2
	pieces["Rook1"].P = 1
	pieces["Rook1"].O = 1
	pieces["Bishop1"] = {}
	pieces["Bishop1"].K = game.ReplicatedStorage.Pieces.Bishop:Clone()
	pieces["Bishop1"].X = 2
	pieces["Bishop1"].Y = 2
	pieces["Bishop1"].P = 1
	pieces["Bishop1"].O = 1
	for i = 1, 9 do
		pieces["Pawn"..i] = {}
		pieces["Pawn"..i].K = game.ReplicatedStorage.Pieces.Pawn:Clone()
		pieces["Pawn"..i].X = i
		pieces["Pawn"..i].Y = 3
		pieces["Pawn"..i].P = 1
		pieces["Pawn"..i].O = 1
	end
	--player 2
	pieces["King2"] = {}
	pieces["King2"].K = game.ReplicatedStorage.Pieces.Jewel:Clone()
	pieces["King2"].X = 5
	pieces["King2"].Y = 9
	pieces["King2"].P = 1
	pieces["King2"].O = -1
	pieces["Gold3"] = {}
	pieces["Gold3"].K = game.ReplicatedStorage.Pieces.Gold:Clone()
	pieces["Gold3"].X = 4
	pieces["Gold3"].Y = 9
	pieces["Gold3"].P = 1
	pieces["Gold3"].O = -1
	pieces["Gold4"] = {}
	pieces["Gold4"].K = game.ReplicatedStorage.Pieces.Gold:Clone()
	pieces["Gold4"].X = 6
	pieces["Gold4"].Y = 9
	pieces["Gold4"].P = 1
	pieces["Gold4"].O = -1
	pieces["Silver3"] = {}
	pieces["Silver3"].K = game.ReplicatedStorage.Pieces.Silver:Clone()
	pieces["Silver3"].X = 3
	pieces["Silver3"].Y = 9
	pieces["Silver3"].P = 1
	pieces["Silver3"].O = -1
	pieces["Silver4"] = {}
	pieces["Silver4"].K = game.ReplicatedStorage.Pieces.Silver:Clone()
	pieces["Silver4"].X = 7
	pieces["Silver4"].Y = 9
	pieces["Silver4"].P = 1
	pieces["Silver4"].O = -1
	pieces["Knight3"] = {}
	pieces["Knight3"].K = game.ReplicatedStorage.Pieces.Knight:Clone()
	pieces["Knight3"].X = 2
	pieces["Knight3"].Y = 9
	pieces["Knight3"].P = 1
	pieces["Knight3"].O = -1
	pieces["Knight4"] = {}
	pieces["Knight4"].K = game.ReplicatedStorage.Pieces.Knight:Clone()
	pieces["Knight4"].X = 8
	pieces["Knight4"].Y = 9
	pieces["Knight4"].P = 1
	pieces["Knight4"].O = -1
	pieces["Lance3"] = {}
	pieces["Lance3"].K = game.ReplicatedStorage.Pieces.Lance:Clone()
	pieces["Lance3"].X = 1
	pieces["Lance3"].Y = 9
	pieces["Lance3"].P = 1
	pieces["Lance3"].O = -1
	pieces["Lance4"] = {}
	pieces["Lance4"].K = game.ReplicatedStorage.Pieces.Lance:Clone()
	pieces["Lance4"].X = 9
	pieces["Lance4"].Y = 9
	pieces["Lance4"].P = 1
	pieces["Lance4"].O = -1
	pieces["Rook2"] = {}
	pieces["Rook2"].K = game.ReplicatedStorage.Pieces.Rook:Clone()
	pieces["Rook2"].X = 2
	pieces["Rook2"].Y = 8
	pieces["Rook2"].P = 1
	pieces["Rook2"].O = -1
	pieces["Bishop2"] = {}
	pieces["Bishop2"].K = game.ReplicatedStorage.Pieces.Bishop:Clone()
	pieces["Bishop2"].X = 8
	pieces["Bishop2"].Y = 8
	pieces["Bishop2"].P = 1
	pieces["Bishop2"].O = -1
	for i = 1, 9 do
		pieces["Pawn"..i+9] = {}
		pieces["Pawn"..i+9].K = game.ReplicatedStorage.Pieces.Pawn:Clone()
		pieces["Pawn"..i+9].X = i
		pieces["Pawn"..i+9].Y = 7
		pieces["Pawn"..i+9].P = 1
		pieces["Pawn"..i+9].O = -1
	end
	local gamestarted = true
	local player1 = nil
	board.Pillow1.BillboardGui.TextLabel.Text = "Empty"
	local bot1 = false
	board.Bot1.Color = Color3.new(1,0,0)
	board.Bot1.SurfaceGui.TextLabel.Text = "Bot: Off"
	local player2 = nil
	board.Pillow2.BillboardGui.TextLabel.Text = "Empty"
	local bot2 = false
	board.Bot2.Color = Color3.new(1,0,0)
	board.Bot2.SurfaceGui.TextLabel.Text = "Bot: Off"
	local cool = false
	local turn = -1
	board.BillboardGui.TextLabel.Text = "Player 1's Turn"
	board.Selected.Position = Vector3.new(0, -3.2, 17.5) + origin.p
	board.Pillow1.ClickDetector.MouseClick:Connect(function(player)
		if gameended == true then return end
		if player1 then
			player1 = game.Players:FindFirstChild(player1.Name)
		end
		if player2 then
			player2 = game.Players:FindFirstChild(player2.Name)
		end
		if player1 == nil then
			player1 = player
		elseif player1 == player then
			player1 = nil
		end
		if player1 or bot1 == true then
			if player2 or bot2 == true then
				gamestarted = true
			end
		end
		if player1 then
			board.Pillow1.BillboardGui.TextLabel.Text = player1.Name
		else
			board.Pillow1.BillboardGui.TextLabel.Text = "Empty"
		end
	end)
	board.Pillow2.ClickDetector.MouseClick:Connect(function(player)
		if gameended == true then return end
		if player1 then
			player1 = game.Players:FindFirstChild(player1.Name)
		end
		if player2 then
			player2 = game.Players:FindFirstChild(player2.Name)
		end
		if player2 == nil then
			player2 = player
		elseif player2 == player then
			player2 = nil
		end
		if player1 or bot1 == true then
			if player2 or bot2 == true then
				gamestarted = true
			end
		end
		if player2 then
			board.Pillow2.BillboardGui.TextLabel.Text = player2.Name
		else
			board.Pillow2.BillboardGui.TextLabel.Text = "Empty"
		end
	end)
	board.Bot1.ClickDetector.MouseClick:Connect(function(player)
		if gameended == true then return end
		if player1 == nil or player1 == player then
			if bot1 == true then
				bot1 = false
				board.Bot1.Color = Color3.new(1,0,0)
				board.Bot1.SurfaceGui.TextLabel.Text = "Bot: Off"
			else
				bot1 = true
				board.Bot1.Color = Color3.new(0,1,0)
				board.Bot1.SurfaceGui.TextLabel.Text = "Bot: On"
			end
		end
	end)
	board.Bot2.ClickDetector.MouseClick:Connect(function(player)
		if gameended == true then return end
		if player2 == nil or player2 == player then
			if bot2 == true then
				bot2 = false
				board.Bot2.Color = Color3.new(1,0,0)
				board.Bot2.SurfaceGui.TextLabel.Text = "Bot: Off"
			else
				bot2 = true
				board.Bot2.Color = Color3.new(0,1,0)
				board.Bot2.SurfaceGui.TextLabel.Text = "Bot: On"
			end
		end
	end)
	local function something(location, newtable)
		local pieces = newtable or pieces
		for i, v in pairs(pieces) do
			if v.X == location.X and v.Y == location.Y then
				return v
			end
		end
	end
	local function findavailable(piece, newtable)
		local pieces = newtable or pieces
		local call = pieces[piece]
		if call == nil then return end
		local available = {}
		if call.X == 0 or call.Y == 0 then
			for i = 1, 9 do
				local pawn = false
				if string.sub(piece, 1, 4) == "Pawn" then
					for v = 1, 9 do
						local tab = {}
						tab.X = i
						tab.Y = v
						local spot = something(tab, newtable)
						if spot then
							if spot.O == call.O and spot.K.Name == "Pawn" and spot.P == 1 then
								pawn = true
							end
						end
					end
				end
				if pawn == false then
					for v = 1, 9 do
						local num = #available + 1
						available[num] = {}
						available[num].X = i
						available[num].Y = v
						
						local spot = something(available[num], newtable)
						if spot then
							available[num] = nil
						elseif string.sub(piece, 1, 4) == "Pawn" or string.sub(piece, 1, 5) == "Lance" or string.sub(piece, 1, 6) == "Knight" then
							if call.O * 4 + 5 == v then
								available[num] = nil
							elseif call.O * 3 + 5 == v and string.sub(piece, 1, 6) == "Knight" then
								available[num] = nil
							end
						end
					end
				end
			end
		else
			local pawn = false
			local front = false
			local knight = false
			local silver = false
			local gold = false
			local lance = false
			local rook = false
			local bishop = false
			if string.sub(piece, 1, 4) == "Pawn" then
				pawn = true
				if call.P == 2 then
					front = true
					gold = true
				end
			elseif string.sub(piece, 1, 5) == "Lance" then
				if call.P == 2 then
					pawn = true
					front = true
					gold = true
				else
					lance = true
				end
			elseif string.sub(piece, 1, 6) == "Knight" then
				if call.P == 2 then
					pawn = true
					front = true
					gold = true
				else
					knight = true
				end
			elseif string.sub(piece, 1, 6) == "Silver" then
				pawn = true
				front = true
				if call.P == 2 then
					gold = true
				else
					silver = true
				end
			elseif string.sub(piece, 1, 4) == "Gold" then
				pawn = true
				front = true
				gold = true
			elseif string.sub(piece, 1, 4) == "King" then
				pawn = true
				front = true
				gold = true
				silver = true
			elseif string.sub(piece, 1, 4) == "Rook" then
				lance = true
				rook = true
				if call.P == 2 then
					front = true
					silver = true
				end
			elseif string.sub(piece, 1, 6) == "Bishop" then
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
				newavailable[#newavailable+1] = available[i]
			end
		end
		return newavailable
	end
	for i, v in pairs(pieces) do
		v.K.Parent = game.Workspace.Pieces
		v.K.CFrame = origin * CFrame.new(12.5-2.5*v.X,1.6,2.5 * v.Y - 12.5) * CFrame.Angles(0,math.pi/2*v.O, math.pi/36)
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
						goal.CFrame = origin * CFrame.new(12.5-2.5*spot.X,5,2.5 * spot.Y - 12.5) * CFrame.Angles(0,math.pi/2*spot.O, math.pi/36)
						tween:Create(spot.K, TweenInfo.new(0.5), goal):Play()
						spot.X = 0
						spot.Y = 0
						spot.P = 1
						spot.O = call.O
						wait(0.75)
						goal.CFrame = origin * CFrame.new(-16*spot.O, 5, -7.5*spot.O) * CFrame.Angles(0,math.pi/2*spot.O, math.pi/36)
						tween:Create(spot.K, TweenInfo.new(0.5), goal):Play()
						wait(0.5)
						if spot.K.Name == "Pawn" then
							local ran = math.random(1,3)-2
							goal.CFrame = origin * CFrame.new((-16+ran*2.5)*spot.O, 0.306, -5*spot.O) * CFrame.Angles(0,math.pi/2*spot.O, math.pi/36)
						elseif spot.K.Name == "Lance" then
							goal.CFrame = origin * CFrame.new(-13.5*spot.O, 0.306, -7.5*spot.O) * CFrame.Angles(0,math.pi/2*spot.O, math.pi/36)
						elseif spot.K.Name == "Knight" then
							goal.CFrame = origin * CFrame.new(-16*spot.O, 0.306, -7.5*spot.O) * CFrame.Angles(0,math.pi/2*spot.O, math.pi/36)
						elseif spot.K.Name == "Silver" then
							goal.CFrame = origin * CFrame.new(-18.5*spot.O, 0.306, -7.5*spot.O) * CFrame.Angles(0,math.pi/2*spot.O, math.pi/36)
						elseif spot.K.Name == "Gold" then
							goal.CFrame = origin * CFrame.new(-13.5*spot.O, 0.306, -10*spot.O) * CFrame.Angles(0,math.pi/2*spot.O, math.pi/36)
						elseif spot.K.Name == "Bishop" then
							goal.CFrame = origin * CFrame.new(-16*spot.O, 0.306, -10*spot.O) * CFrame.Angles(0,math.pi/2*spot.O, math.pi/36)
						elseif spot.K.Name == "Rook" then
							goal.CFrame = origin * CFrame.new(-18.5*spot.O, 0.306, -10*spot.O) * CFrame.Angles(0,math.pi/2*spot.O, math.pi/36)
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
					goal.CFrame = origin * CFrame.new(-16*call.O, 5, -7.5*call.O) * CFrame.Angles(0,math.pi/2*call.O, math.pi/36)
					tween:Create(call.K, TweenInfo.new(0.5), goal):Play()
					wait(0.75)
				elseif string.sub(piece, 1, 6) == "Knight" and call.P == 1 then
					goal.CFrame = origin * CFrame.new(12.5-2.5*call.X,5,2.5 * call.Y - 12.5) * CFrame.Angles(0,math.pi/2*call.O, math.pi/36)
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
					goal.CFrame = origin * CFrame.new(12.5-2.5*call.X,1.6,2.5 * call.Y - 12.5) * CFrame.Angles(0,-math.pi/2*call.O, math.pi)
				else
					goal.CFrame = origin * CFrame.new(12.5-2.5*call.X,1.6,2.5 * call.Y - 12.5) * CFrame.Angles(0,math.pi/2*call.O, math.pi/36)
				end
				tween:Create(call.K, TweenInfo.new(0.5), goal):Play()
				wait(0.5)
				if oldp == 1 and call.P == 2 then
					wait(0.25)
					goal.CFrame = origin * CFrame.new(12.5-2.5*call.X,5,2.5 * call.Y - 12.5) * CFrame.Angles(0,-math.pi/2*call.O, math.pi)
					tween:Create(call.K, TweenInfo.new(0.5), goal):Play()
					wait(0.75)
					goal.CFrame = origin * CFrame.new(12.5-2.5*call.X,5,2.5 * call.Y - 12.5) * CFrame.Angles(0,-math.pi/2*call.O, math.pi)
					tween:Create(call.K, TweenInfo.new(0.25), goal):Play()
					wait(0.25)
					goal.CFrame = origin * CFrame.new(12.5-2.5*call.X,1.6,2.5 * call.Y - 12.5) * CFrame.Angles(0,-math.pi/2*call.O, math.pi)
					tween:Create(call.K, TweenInfo.new(0.25), goal):Play()
					wait(0.25)
				end
				turn = turn * -1
				local addon = ""
				local player = turn/2 + 1.5
				local check = check("King"..player)
				if check == true then
					addon = "Check - "
				end
				board.BillboardGui.TextLabel.Text = addon.."Player "..player.."'s Turn"
				board.Selected.Position = Vector3.new(0, -3.2, -17.5*turn) + origin.p
				local numofmoves = 0
				for i, v in pairs(pieces) do
					if v.O == turn then
						local available = moves(i)
						numofmoves = numofmoves + #available
					end
				end
				if numofmoves == 0 then
					board.BillboardGui.TextLabel.Text = "Checkmate - Player "..player.." Wins"
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
				local movevalue = math.huge*-1
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
	board.Timer1.SurfaceGui.TextLabel.Text = "Time: "..math.floor(plr1time)
	local plr2time = 0
	board.Timer2.SurfaceGui.TextLabel.Text = "Time: "..math.floor(plr2time)
	repeat
		wait(0.1)
		if turn == -1 then
			if player1 ~= nil or bot1 == true then
				plr1time = plr1time + 0.1
				board.Timer1.SurfaceGui.TextLabel.Text = "Time: "..math.floor(plr1time)
			end
		elseif turn == 1 then
			if player2 ~= nil or bot2 == true then
				plr2time = plr2time + 0.1
				board.Timer2.SurfaceGui.TextLabel.Text = "Time: "..math.floor(plr2time)
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