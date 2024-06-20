using Microsoft.AspNetCore.Mvc;

// For more information on enabling Web API for empty projects, visit https://go.microsoft.com/fwlink/?LinkID=397860

namespace Spark.Controllers;
[Route("api/[controller]")]
[ApiController]
public class LobbyController : ControllerBase
{

    [HttpPost]
    public void CreateLobby([FromBody] string playerId)
    {
    }

    [HttpPost]
    public void CreateLobbies([FromBody] string[] playerId)
    {
    }


    // PUT api/<LobbyController>/5
    [HttpPut("{id}")]
    public void Put(int id, [FromBody] string value)
    {
    }

    [HttpDelete("{id}")]
    public void DeleteLobby(int id)
    {
    }
}
