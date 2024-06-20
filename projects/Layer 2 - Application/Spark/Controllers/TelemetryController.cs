using Microsoft.AspNetCore.Mvc;

// For more information on enabling Web API for empty projects, visit https://go.microsoft.com/fwlink/?LinkID=397860

namespace Spark.Controllers;
[Route("api/[controller]")]
[ApiController]
public class TelemetryController : ControllerBase
{

    [HttpGet("players/count/{playerState}")]
    public string GetPlayerCount(int playerState)
    {
        return "value";
    }

    [HttpGet("matches/count/{queue}")]
    public string GetMatchCount(int queue)
    {
        return "value";
    }

}
