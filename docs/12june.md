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

### Structure 2

Coral instances
Orchestrator
Coraline clients

- The orchestrator is lightweight, proxies the requests to microservices so he can handle a lot of clients
- Orchestrator can communicate directly with coral services,
  - or through redis pubsub (ex: send action 'find a match for Client1')
  - or redis events ('put Client1 in queue1')

## Gaming

When the coral services find a game, they tell the clients to connect to a game server instance:

### Structure

Moonstone instances
Moonstone clients (in Glaceon)

- Coral tell each of the clients: package Game[MoonstoneAddresse, GameId, Token...]
  - Coral instances might have to communicate between each other: [Client1Id, Client2Id, Client3Id]
  - Also tell the Moonstone server to open a game and wait for everyone to connect
- Glaceon starts the moonstone client, connects to the address given with a token for the game.
