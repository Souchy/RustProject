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
