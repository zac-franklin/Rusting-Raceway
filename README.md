# Rusting-Raceway
Recreation of Mario Party’s [Rocking Raceway](https://www.mariowiki.com/Rockin%27_Raceway) - purely for the purpose of learning Bevy. 

# Prereqs
- [Rust](https://www.rust-lang.org/)
- [matchbox_server](https://github.com/johanhelsing/matchbox/tree/main/matchbox_server)
```
cargo install matchbox_server
```

# Build
## Development
```
cargo build
```

## Production
```
cargo build --release
```

# Run
## With Matchmaking
- Open Three terminals.
- In Teremina 1 start the matchbox server: 
```
matchbox_server
```

- In Terminal 2 start the first game session:
```
cargo run
```

- In Terminal 3 start the second game session:  
```
cargo run
```

## Without Matchmacking
- Start a game session disabling the live matchmaking. If you don't override this but would like to develop without the matchbox_server, the game will be unable to find enough players and quit.
```
cargo run -- -l
```

# Usesful sources
## Multiplayer
The multiplayer code in this game follows: https://johanhelsing.studio/posts/extreme-bevy

# TODO
- [ ] Error Handling
- [ ] Tests
- [ ] Menu screen
- [ ] Custom WebRTC server
- [ ] Production servers 