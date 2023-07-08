## Description
This project aims to enhance the readability of decompiled output, making it more understandable,

## Demonstration

### An example of sentinel v3 decompiled output*
```lua
local v_u_1 = game:GetService("ReplicatedStorage")
local v_u_2 = game:GetService("Workspace")

local v_u_3 = require(v_u_1.Modules.QuestData)
local v_u_4 = require(v_u_1.Modules.GamepassData)

local v_u_5 = v_u_2:WaitForChild("GameObject")
local v_u_6 = game.Players.LocalPlayer

local v_u_7 = {}

for v_u_8 = 1, 10 do
    if v_u_2:GetChildren()[v_u_8].Name == v_u_5.Name then
        local v_u_9 = v_u_2:GetChildren()[v_u_8]
        local v_u_10 = v_u_3.Function(v_u_6, v_u_3.Quests[1])

        local v_u_11 = v_u_4.Check(v_u_6, v_u_9, v_u_10)
        table.insert(v_u_7, v_u_11)
    end
end

return v_u_7
```

### After running it through the output cleaner
```lua
local ReplicatedStorage = game:GetService("ReplicatedStorage")
local Workspace = game:GetService("Workspace")

local QuestData = require(ReplicatedStorage.Modules.QuestData)
local GamepassData = require(ReplicatedStorage.Modules.GamepassData)

local GameObject = Workspace:WaitForChild("GameObject")
local Players_LocalPlayer = game.Players.LocalPlayer

local table_1 = {}

for v_u_8 = 1, 10 do
    if Workspace:GetChildren()[v_u_8].Name == GameObject.Name then
        local v_u_9 = Workspace:GetChildren()[v_u_8]
        local v_u_10 = QuestData.Function(Players_LocalPlayer, QuestData.Quests[1])

        local v_u_11 = GamepassData.Check(Players_LocalPlayer, v_u_9, v_u_10)
        table.insert(table_1, v_u_11)
    end
end

return table_1
```

### Usage

## Websocket

1. Download `cleaner_websocket.exe` from releases
2. Run the file, a message should pop up with "running at ws://`address`:`port`
3. Connect to the webscoket however you like
4. The websocket will take in a string of lua code and return it cleaned

## Executable

1. Download `cleaner.exe` from releases
2. Create an `input.lua` file in the same directory
3. Place desired code into the `input.lua` file
4. Run `cleaner.exe`, output will be placed in the `output.lua` file (created automatically)

### Executable will be changed to be more intuitive at a later date
