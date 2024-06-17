# The idea is

## Matchmaking

### Structure 1

Coral instances
Nginx TCP load balancer
Coraline clients (in Glaceon)

Client1 talks to Coral1, Client2 talks to Coral2.
When Coral2 finds a match with Client1 and Client2,

- he sends a message to Client 2 since he already has a connection to him.
- he sends a message to all services to tell them to talk to Client1 because he doesnt have a direct connection himself (maybe/prob do this through redis pubsub)
  - if we use messaging: we can run into race conditions where 2 services try to match the same person
  - if we use redis events: maybe can work

### Structure 2

Coral instances
Orchestrator
Coraline clients

- The orchestrator is lightweight, proxies the requests to microservices so he can handle a lot of clients
- Orchestrator assures synchronicity if 2 corals try to match the same ppl
- Orchestrator can communicate directly with coral services,
  - or through redis pubsub (ex: send action 'find a match for Client1')
  - or redis events ('put Client1 in queue1')

### Structure 3

Mover
Coral instances
Nginx TCP load balancer
Coraline clients

1. Coraline sends connects to a random Coral and sends a queue request
2. Coral finds a match, sends it to the Mover
3. Mover synchronizes matches.
4. Mover sends the match to all Coral services and one Moonstone
5. Coral services send match to Coralines
6. Coralines close connection to Coral
7. Onyxes connect to Moonstone

### Structure 4

Coral instances
Nginx TCP load balancer
Coraline clients

1. Coraline sends connects to a random Coral and sends a queue request
2. Coral finds a match:
   1. Set coralines state to InGame
   2. If it fails, you just cancel this match request
   3. Coraline stays connected and sends a new match request after 2 minutes if not found
3. Coral creates a Game object on redis with the list of clients and a Moonstone server address + token for the game
4. Coral services watching Game events
   1. If one of its Coraline clients is in a new Game, cancel their match request
5. Coral services send match to Coralines
6. Coralines close connection to Coral
7. Onyxes connect to Moonstone

### Structure 5 (Best so far 15 juin 2:53am)

Coral instances
Realm
Coraline clients

1. Glaceon connects to realm
2. Glaceon asks Realm to create a lobby
   1. Glaceon answers with a Coral service port to connect to
3. Glaceon can add friends to the lobby. People can join the lobbyh if they have the token/password
4. Glaceon host asks Coral to start the queue in the lobby
5. Coral finds match for lobby
6. Coral broadcasts to all servers the Prematch object containing all lobbies // notifies players in the lobby and other servers to send a message to other players in the Prematch
7. Once we agree on the Prematch, send the object to all the clients
8. Each Coral service sends game message to all players listed in the prematch
9. All Clients connect to Moonstone
10. Clients/Lobbies state moves from Prematch to Game

- Realm can communicate directly with coral services,
  - or through redis pubsub (ex: send action 'find a match for Client1')
  - or redis events ('put Client1 in queue1')

Interesting Redis info:

1. If Watched keys are modified during a transac before the Exec, the whole transac aborts.
2. Each Coral can Watch the Lobbies they're assigned to
3. When one Coral finds a match, it tries to modify all Lobbies to set them in the Prematch
~~4. Other nodes will be notified and can cancel the queue for those lobbies at that moment.~~ Not true. You may be able to use keyspace-notifications to get that.
4. Then that Coral will also need to send a pubsub message to other Corals to notify about the Prematch containing all those Lobbies

Fallback if Coral or Realm panics:

1. Low lifetime of lobbies and prematches: Coral has to keep them alive
2. When a lobby/prematch dies, someone should catch the event, make a new lobby and put clients back into queue if they were

Conditional transaction:

- Check that lobbies are not in Prematch state before exec the transaction (Ex: Coral1 modifies state and execs. Coral2 then also modifies state, but Coral1 was already done, so there's no race condition, so it doesnt fail even though we should. The state should be InQueue before xecuting the transaction, then it goes to Prematch)
- Maybe Lua <https://redis.io/ebook/part-3-next-steps/chapter-11-scripting-redis-with-lua/>

Refs:
<https://redis.io/docs/latest/develop/interact/transactions/#optimistic-locking-using-check-and-set>
<https://redis.io/blog/you-dont-need-transaction-rollbacks-in-redis/>
<https://redis.io/docs/latest/develop/use/keyspace-notifications/>

### Game server

Moonstone services
Onyx clients

Game server is probably coded in whatever language/library is given with the engine.
Ex C#: Godot's Multiplayer nodes, Unity Netcode
Ex C++: Unreal

## Gaming

When the coral services find a game, they tell the clients to connect to a game server instance:

### Structure

Moonstone instances
Moonstone clients (in Glaceon)

- Coral tell each of the clients: package Game[MoonstoneAddresse, GameId, Token...]
  - Coral instances might have to communicate between each other: [Client1Id, Client2Id, Client3Id]
  - Also tell the Moonstone server to open a game and wait for everyone to connect
- Glaceon starts the moonstone client, connects to the address given with a token for the game.
