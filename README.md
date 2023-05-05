# Monai - Monopoly AI project

## File structure
- Board = Server, shows the game board and handles primary logic (Bevy)
- Player = Human player, provides UI to interact with board (Bevy + Egui)
- Computer = AI player, provides information on the AI and trains model (non-visual)
- Store = Shared information between server and clients

## Process
The game [board](board) is effectively a webserver, so it needs to be started first.
```sh
cd board
cargo run
```
By default, the board will run on the port 1095 (the sum of the ascii characters for MONAI), with a WebRTC port of 1096 for WASM. This can be configured in the file [server.rs](board/src/server.rs#L12). If running clients locally, you can connect using 127.0.0.1, however it is likely that you will have to open the port 1095 in your router. The default code is **MONAI**, [but you can change this.](board/src/server.rs#L22)

For clients, the process depends on whether the player is a [computer](player/computer) or [human](player/human). For a human player, you can either use the WASM client on [my website](https://binarysky.ai/monai-player) or compile and run locally. The WASM client uses wasm-bindgen and the index.html present in the [human's directory](player/human/src/index.html). To run locally, use the same process as the server.
```sh
cd player/human
cargo run
```
From there, you should be able to enter a player name, the authorization code, and the address and port of the board (e.g. 127.0.0.1:1095).

If you want to use the AI, you need to supply command-line arguments. The AI is headless, so no GUI will be present while running/training. It uses a DDQN for reinforcement learning. For settings with the model itself (like the shape, epsilon, optimizer, etc), check [model.rs](player/computer/src/model.rs). To run it, you have to give cargo a few arguments.
```sh
cd player/computer
cargo run -- SERVER CODE NAME NPZ
```
For example, `cargo run -- 127.0.0.1:1095 MONAI Bot1 models/Bot1.npz` would connect a bot player to our localhosted server named Bot1 using Bot1.npz. Some primitive models have been supplied in the repository. At the moment, the model is hardcoded to support a 40 tile board with 4 players, but you can expand this by changing the const values.
