
# Separate service

Realm is whoever's game server.
They handle their clients themselves.
No clients for us in the experimentation
Realm sends match requests to Corals instances
 lobby info: { queue, playersList }
Coral returns match to Realm

## Create a lib for Realm games to use

- Has messages for Coral
- Has objects for Lobby, Player..
- Has messages to create test players and lobbies
- Has messages to put redis players in lobby and in queue (simulation)

## Make a example Realm implementation

- No clients (no coraline), only data on redis
- Activate stub messages only with a build parameter
  - Spark can ask Realm to create players on redis
  - Spark can ask Realm to create Lobbies and put players in them
  - Spark can ask Realm to put a Lobby in queue (find match)
  - Spark can see the database and give stats to SparkUI on the number of players and lobbies finding matches, the average queue time, etc..
- Make Spark + SparkUI
