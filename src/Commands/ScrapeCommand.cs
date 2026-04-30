using System.ComponentModel;
using Reddit.Console.Infrastructure;
using Spectre.Console;
using Spectre.Console.Cli;

namespace Reddit.Console.Commands;

public sealed class ScrapeCommand : AsyncCommand<ScrapeCommand.Settings>
{
    public sealed class Settings : GlobalSettings
    {
        [CommandArgument(0, "<URL>")]
        [Description("Reddit URL to scrape (post, community, or user page)")]
        public required string Url { get; init; }

        [CommandOption("--skip-comments")]
        [Description("Skip scraping comments")]
        public bool SkipComments { get; init; }

        [CommandOption("--skip-user-posts")]
        [Description("Skip scraping user posts when visiting user pages")]
        public bool SkipUserPosts { get; init; }

        [CommandOption("--skip-community")]
        [Description("Skip community info (still scrapes posts)")]
        public bool SkipCommunity { get; init; }

        [CommandOption("--max-items <N>")]
        [Description("Maximum items to save")]
        [DefaultValue(10)]
        public int MaxItems { get; init; } = 10;

        [CommandOption("--max-posts <N>")]
        [Description("Maximum posts per page")]
        [DefaultValue(10)]
        public int MaxPosts { get; init; } = 10;

        [CommandOption("--max-comments <N>")]
        [Description("Maximum comments per post")]
        [DefaultValue(10)]
        public int MaxComments { get; init; } = 10;

        [CommandOption("--max-communities <N>")]
        [Description("Maximum community pages to scrape")]
        [DefaultValue(2)]
        public int MaxCommunities { get; init; } = 2;

        [CommandOption("--max-users <N>")]
        [Description("Maximum user pages to scrape")]
        [DefaultValue(2)]
        public int MaxUsers { get; init; } = 2;

        [CommandOption("--since <DATE>")]
        [Description("Only include posts after this date (e.g. 2025-01-01)")]
        public string? Since { get; init; }

        [CommandOption("--comments-since <DATE>")]
        [Description("Only include comments after this date")]
        public string? CommentsSince { get; init; }
    }

    protected override async Task<int> ExecuteAsync(CommandContext context, Settings settings, CancellationToken cancellation)
    {
        AnsiConsole.MarkupLine("[grey]Scraping Reddit (this may take 30–60s)...[/]");

        using var client = settings.CreateClient();
        var doc = await client.ScrapeAsync(new ScrapeInput
        {
            StartUrls = [new { url = settings.Url }],
            SkipComments = settings.SkipComments,
            SkipUserPosts = settings.SkipUserPosts,
            SkipCommunity = settings.SkipCommunity,
            MaxItems = settings.MaxItems,
            MaxPostCount = settings.MaxPosts,
            MaxComments = settings.MaxComments,
            MaxCommunitiesCount = settings.MaxCommunities,
            MaxUserCount = settings.MaxUsers,
            PostDateLimit = settings.Since,
            CommentDateLimit = settings.CommentsSince
        });

        YamlOutput.Write(doc);
        return 0;
    }
}
