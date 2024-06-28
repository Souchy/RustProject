using IdGen;
using Redis.OM;
using Redis.OM.Searching;
using StackExchange.Redis;
using static Player.Types;

namespace Spark.Repositories;

public class PlayerRepository
{

    private readonly RedisCollection<Player> _collection;
    private readonly RedisConnectionProvider _provider;
    private readonly IDatabase _db;
    private readonly IdGenerator _idGenerator;

    public PlayerRepository(RedisConnectionProvider provider, ConnectionMultiplexer mux, IdGenerator idGen)
    {
        _db = mux.GetDatabase();
        _provider = provider;
        _collection = (RedisCollection<Player>) provider.RedisCollection<Player>();
        _idGenerator = idGen;
    }

    public int GetPlayerCount(int playerState)
    {
        PlayerState state = (PlayerState) playerState;
        return _collection.Count(p => p.State == state);
    }

    public IRedisCollection<Player> GetPlayers(int playerState)
    {
        PlayerState state = (PlayerState) playerState;
        return _collection.Where(p => p.State == state);
    }

    public async Task<Player?> GetById(string id)
    {
        return await _collection.FindByIdAsync(id);
    }

    public IEnumerable<Player> Get()
    {
        return _collection.Where(p => true);
    }

    public async Task Generate(int count)
    {
        var ids = _idGenerator.Take(count);
        foreach (var id in ids)
        {
            var player = new Player();
            player.Id = id;
            player.Lobby = 0;
            player.Mmr = 1000;
            player.State = Player.Types.PlayerState.Idle;
            await _collection.InsertAsync(player);
        }
    }

}
