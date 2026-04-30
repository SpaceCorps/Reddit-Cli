using System.ComponentModel;
using Reddit.Console.Infrastructure;
using Spectre.Console;
using Spectre.Console.Cli;

namespace Reddit.Console.Commands;

public sealed class SearchCommand : AsyncCommand<SearchCommand.Settings>
{
    public sealed class Settings : GlobalSettings
    {
        [CommandArgument(0, "<QUERY>")]
        [Description("Search query")]
        public required string Query { get; init; }

        [CommandOption("--community <NAME>")]
        [Description("Limit search to a specific community (e.g. 'programming')")]
        public string? Community { get; init; }

        [CommandOption("--comments")]
        [Description("Search for comments instead of posts")]
        public bool Comments { get; init; }

        [CommandOption("--communities")]
        [Description("Search for communities")]
        public bool Communities { get; init; }

        [CommandOption("--users")]
        [Description("Search for users")]
        public bool Users { get; init; }

        [CommandOption("--sort <SORT>")]
        [Description("Sort by: relevance, hot, top, new, rising, comments")]
        [DefaultValue("new")]
        public string Sort { get; init; } = "new";

        [CommandOption("--time <TIME>")]
        [Description("Filter by time: all, hour, day, week, month, year")]
        public string? Time { get; init; }

        [CommandOption("--max-items <N>")]
        [Description("Maximum items to return")]
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

        [CommandOption("--skip-comments")]
        [Description("Skip scraping comments on posts")]
        public bool SkipComments { get; init; }

        [CommandOption("--nsfw")]
        [Description("Include NSFW content")]
        public bool Nsfw { get; init; }

        [CommandOption("--since <DATE>")]
        [Description("Only include posts after this date (e.g. 2025-01-01)")]
        public string? Since { get; init; }

        [CommandOption("--comments-since <DATE>")]
        [Description("Only include comments after this date")]
        public string? CommentsSince { get; init; }
    }

    protected override async Task<int> ExecuteAsync(CommandContext context, Settings settings, CancellationToken cancellation)
    {
        var hasTypeFlag = settings.Comments || settings.Communities || settings.Users;

        AnsiConsole.MarkupLine("[grey]Searching Reddit (this may take 30–60s)...[/]");

        using var client = settings.CreateClient();
        var doc = await client.SearchAsync(new SearchInput
        {
            Searches = [settings.Query],
            SearchCommunityName = settings.Community,
            SearchPosts = !hasTypeFlag || (!settings.Comments && !settings.Communities && !settings.Users),
            SearchComments = settings.Comments,
            SearchCommunities = settings.Communities,
            SearchUsers = settings.Users,
            Sort = settings.Sort,
            Time = settings.Time,
            IncludeNSFW = settings.Nsfw,
            MaxItems = settings.MaxItems,
            MaxPostCount = settings.MaxPosts,
            MaxComments = settings.MaxComments,
            SkipComments = settings.SkipComments,
            PostDateLimit = settings.Since,
            CommentDateLimit = settings.CommentsSince
        });

        YamlOutput.Write(doc);
        return 0;
    }
}
