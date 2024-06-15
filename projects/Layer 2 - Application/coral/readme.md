# Matchmaking service

We could have a service to manager them (ex: keep a list of online corals, ping them for alive, etc)

But, we could let the Corals manage themselves. Ex:
- Coral starts online
- Adds itself to the list of nodes in the redis db (ports list)
- Send heartbeat to all nodes in the list ("hello world")
- Nodes reply with the leader id (leader id could be in redis, and all nodes should watch it)
- Each node sends heartbeats to the leader
- Elect a leader node, etc.
- When a node timesout, remove it from the list in db (Leader or Realm or Manager node does this)

Realm will see the number of nodes and players and can decide to start a new Coral service.