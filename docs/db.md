# Database schemas

## Redis structure 1

```yaml
moonstone:
   ports: [3300, 3301, 3302, 3304]

coral:
   ports: [3400, 3401, 3402, 3404]
   queues: [1, 2, 3]

monitor:
   ports: [3001]

data:
   users:
      1:
         state: InQueue1
         mmr: 2010
         recentMatchedPlayers: [1, 2, 3]
         geolocalisation: 1234
      2:
         state: InGame
         state: InQueue1
      3:
         state: Idle
     
   lobbies:
      1:
         coral: 3304
         state: Idle/InQueue
         queue: 1
         players: [1, 2, 3]
         token: "29435t0g78n"
   games:
      1:
         players: [1, 2, 3]
         startTime: 123445
         type: 0
```

## Mongo structure 1

```json
users: [
 {
  id: ""
  name: "",
  money: 2,
  email: "",
  mmr: 2010,
  recentMatchedPlayers: [1, 2, 3],
  geolocalisation: 1234
  etc: ""
 }
]
```
