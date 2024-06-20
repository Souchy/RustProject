using Microsoft.AspNetCore.Mvc;
using Spark.Repositories;

// For more information on enabling Web API for empty projects, visit https://go.microsoft.com/fwlink/?LinkID=397860

namespace Spark.Controllers;
[Route("api/[controller]")]
[ApiController]
public class PlayerController : ControllerBase
{

    private readonly ILogger<PlayerController> _logger;
    private readonly PlayerRepository _playerRepository;

    public PlayerController(
        PlayerRepository playerRepo,
        ILogger<PlayerController> logger
    )
    {
        _logger = logger;
        _playerRepository = playerRepo;
    }

    [HttpGet("count/{playerState}")]
    public int GetPlayerCount(int playerState)
    {
        return _playerRepository.GetPlayerCount(playerState);
    }

    [HttpGet("all")]
    public ICollection<Player> Get()
    {
        return _playerRepository.Get();
    }

    [HttpPost("generate")]
    public async void GeneratePlayers(int count)
    {
        await _playerRepository.Generate(count);
    }

}
