# Messaging

## Realm messaging

### Glaceon->Realm: Create a Lobby

```json
msg: {
 queue: 1,
}
```

### Realm->Glaceon: Created a Lobby

```json
msg: {
 queue: 1,
 lobby: 1,
 coral: 3304,
 token: "29435t0g78n"
}
```

Coraline connects to Coral 3304.

### Glaceon->Realm: Send invitation to lobby

```json
msg: {
 queue: 1,
 playerTo: 2,
}
```

### Realm->Glaceon: Relay invitation to lobby

```json
msg: {
 queue: 1,
 playerFrom: 1,
 lobby: 1,
 token: "29435t0g78n"
}
```

### Glaceon->Realm: Join player's lobby

```json
msg: {
 queue: 1,
 lobby: 1,
 token: "29435t0g78n"
}
```

### Realm->Glaceon: Broadcast to lobby added player

```json
msg: {
 player: 1
}
```

## Coral messaging

### Coraline->Coral: Request match (set in queue)

```json
msg: {
 queue: 1,
 players: [1, 2, 3]
}
```

Problem: Coral can't send messages to teammates

### Coral->Coraline: Broadcast to lobby: Set in queue

```json
msg: {
 queue: 1,
 state: InQueue
}
```

### Coral->Coraline: Coral sends match to Coralines

```json
msg: {
 moonstone: 3304,
 players: [1, 2, 3],
 token: "25o843r7yhfgb"
}
```
