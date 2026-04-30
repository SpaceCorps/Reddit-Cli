using Reddit.Console.Commands;
using Spectre.Console.Cli;

var app = new CommandApp();

app.Configure(config =>
{
    config.SetApplicationName("reddit");

    config.AddCommand<SearchCommand>("search")
        .WithDescription("Search Reddit for posts, comments, communities, or users");

    config.AddCommand<ScrapeCommand>("scrape")
        .WithDescription("Scrape a Reddit URL (post, community, or user page)");
});

return app.Run(args);
