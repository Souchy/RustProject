
using Redis.OM;
using Spark.Repositories;
using StackExchange.Redis;

namespace Spark;

public class Program
{
    public static void Main(string[] args)
    {
        var builder = WebApplication.CreateBuilder(args);

        // Add services to the container.

        builder.Services.AddControllers();
        // Learn more about configuring Swagger/OpenAPI at https://aka.ms/aspnetcore/swashbuckle
        builder.Services.AddEndpointsApiExplorer();
        builder.Services.AddSwaggerGen();

        // Add custom services
        AddServices(builder.Services);

        var app = builder.Build();

        // Configure the HTTP request pipeline.
        if (app.Environment.IsDevelopment())
        {
            app.UseSwagger();
            app.UseSwaggerUI();
        }

        app.UseHttpsRedirection();

        app.UseAuthorization();


        app.MapControllers();

        app.Run();
    }

    private static void AddServices(IServiceCollection services)
    {
        // Redis Mux
        ConnectionMultiplexer mux = ConnectionMultiplexer.Connect("host.docker.internal:6379");
        services.AddSingleton<ConnectionMultiplexer>(mux);

        // Redis OM
        services.AddSingleton(new RedisConnectionProvider(mux));

        // Repositories
        services.AddSingleton<PlayerRepository>();
        services.AddSingleton<LobbyRepository>();
        services.AddSingleton<MatchRepository>();
        services.AddSingleton<GameRepository>();
    }

}
