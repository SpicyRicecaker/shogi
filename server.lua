local open = ""
local opentable = {}
local spots = {}
local selec = nil
local promote = nil
local basicgui = script.Parent:WaitForChild("ScreenGui")
basicgui.Promote.Accept.MouseButton1Down:Connect(function()
	promote = true
	basicgui.Promote.Visible = false
end)
basicgui.Promote.Decline.MouseButton1Down:Connect(function()
	promote = false
	basicgui.Promote.Visible = false
end)
game.ReplicatedStorage.RemoteEvent.OnClientEvent:Connect(function(piece, event, available, tab, board)
	if event == "place" then
		local origin = board.Position + Vector3.new(0,0.05,0)
		for i = 1, #spots do
			spots[i].K:Destroy()
			spots[i] = nil
		end
		if selec then
			selec:Destroy()
		end
		if open ~= piece then
			selec = game.ReplicatedStorage.Selected:Clone()
			selec.Parent = game.Workspace
			if tab.P == 1 then
				selec.CFrame = tab.K.CFrame * CFrame.new(0,-0.05,0) * CFrame.Angles(0,0,-math.pi/36)
			elseif tab.P == 2 then
				selec.CFrame = tab.K.CFrame * CFrame.new(0,0.05,0)
			end
			open = piece
			opentable = tab
			for i = 1, #available do
				spots[i] = {}
				spots[i].K = game.ReplicatedStorage.Spot:Clone()
				spots[i].K.Parent = game.Workspace
				spots[i].K.Position = Vector3.new(12.5 - available[i].X * 2.5, 1.55, available[i].Y * 2.5 - 12.5) + origin
				spots[i].X = available[i].X
				spots[i].Y = available[i].Y
				spots[i].K.ClickDetector.MouseClick:Connect(function()
					promote = nil
					local ask = false
					if tab.O == -1 and tab.X ~= 0 and tab.P == 1 then
						if spots[i].Y <= 3 or tab.Y <= 3 then
							ask = true
							if spots[i].Y == 1 and tab.K.Name == "Pawn" then
								ask = false
								promote = true
							elseif spots[i].Y == 1 and tab.K.Name == "Lance" then
								ask = false
								promote = true
							elseif spots[i].Y <= 2 and tab.K.Name == "Knight" then
								ask = false
								promote = true
							end
						end
					elseif tab.O == 1 and tab.X ~= 0 and tab.P == 1 then
						if spots[i].Y >= 7 or tab.Y >= 7 then
							ask = true
							if spots[i].Y == 9 and tab.K.Name == "Pawn" then
								ask = false
								promote = true
							elseif spots[i].Y == 9 and tab.K.Name == "Lance" then
								ask = false
								promote = true
							elseif spots[i].Y >= 8 and tab.K.Name == "Knight" then
								ask = false
								promote = true
							end
						end
					end
					if ask == true then
						basicgui.Promote.Visible = true
						repeat
							wait(0.1)
						until promote
					end
					game.ReplicatedStorage.RemoteEvent:FireServer(piece, available[i].X, available[i].Y, promote)
					for i = 1, #spots do
						spots[i].K:Destroy()
						spots[i] = nil
					end
					selec:Destroy()
					selec = nil
				end)
			end
		else
			open = ""
			opentable = {}
		end
	elseif event == "attack" then
		for i = 1, #spots do
			if spots[i] then
				if spots[i].X == available.X and spots[i].Y == available.Y then
					promote = nil
					local ask = false
					if opentable.O == -1 and opentable.X ~= 0 and opentable.P == 1 then
						if spots[i].Y <= 3 or opentable.Y <= 3 then
							ask = true
							if spots[i].Y == 1 and opentable.K.Name == "Pawn" then
								ask = false
								promote = true
							elseif spots[i].Y == 1 and opentable.K.Name == "Lance" then
								ask = false
								promote = true
							elseif spots[i].Y <= 2 and opentable.K.Name == "Knight" then
								ask = false
								promote = true
							end
						end
					elseif opentable.O == 1 and opentable.X ~= 0 and opentable.P == 1 then
						if spots[i].Y >= 7 or opentable.Y >= 7 then
							ask = true
							if spots[i].Y == 9 and opentable.K.Name == "Pawn" then
								ask = false
								promote = true
							elseif spots[i].Y == 9 and opentable.K.Name == "Lance" then
								ask = false
								promote = true
							elseif spots[i].Y >= 8 and opentable.K.Name == "Knight" then
								ask = false
								promote = true
							end
						end
					end
					if ask == true then
						basicgui.Promote.Visible = true
						repeat
							wait(0.1)
						until promote
					end
					game.ReplicatedStorage.RemoteEvent:FireServer(open, available.X, available.Y, promote)
					for i = 1, #spots do
						spots[i].K:Destroy()
						spots[i] = nil
					end
					if selec then
						selec:Destroy()
						selec = nil
					end
				end
			end
		end
	end
end)