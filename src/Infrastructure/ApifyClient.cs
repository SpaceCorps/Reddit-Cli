using System.Net.Http.Json;
using System.Text.Json;
using System.Text.Json.Serialization;

namespace Reddit.Console.Infrastructure;

public sealed class ApifyClient : IDisposable
{
    private static readonly JsonSerializerOptions JsonOptions = new()
    {
        PropertyNamingPolicy = JsonNamingPolicy.CamelCase,
        PropertyNameCaseInsensitive = true,
        DefaultIgnoreCondition = JsonIgnoreCondition.WhenWritingNull
    };

    private readonly HttpClient _http;
    private readonly string _token;

    public ApifyClient(string token)
    {
        _token = token;
        _http = new HttpClient
        {
            BaseAddress = new Uri("https://api.apify.com/v2/"),
            Timeout = TimeSpan.FromMinutes(5)
        };
    }

    public async Task<JsonDocument> SearchAsync(SearchInput input)
    {
        var endpoint = $"acts/trudax~reddit-scraper-lite/run-sync-get-dataset-items?token={_token}";
        return await PostAndReadAsync(endpoint, input);
    }

    public async Task<JsonDocument> ScrapeAsync(ScrapeInput input)
    {
        var endpoint = $"acts/trudax~reddit-scraper-lite/run-sync-get-dataset-items?token={_token}";
        return await PostAndReadAsync(endpoint, input);
    }

    private async Task<JsonDocument> PostAndReadAsync(string endpoint, object body)
    {
        var response = await _http.PostAsJsonAsync(endpoint, body, JsonOptions);

        if (!response.IsSuccessStatusCode)
        {
            var error = await response.Content.ReadAsStringAsync();
            throw new HttpRequestException($"Apify API error: {response.StatusCode} — {error}");
        }

        var stream = await response.Content.ReadAsStreamAsync();
        return await JsonDocument.ParseAsync(stream);
    }

    public void Dispose() => _http.Dispose();
}

public sealed class SearchInput
{
    public string[]? Searches { get; init; }
    public string? SearchCommunityName { get; init; }
    public bool SearchPosts { get; init; } = true;
    public bool SearchComments { get; init; }
    public bool SearchCommunities { get; init; }
    public bool SearchUsers { get; init; }
    public string? Sort { get; init; }
    public string? Time { get; init; }
    public bool IncludeNSFW { get; init; } = true;
    public int MaxItems { get; init; } = 10;
    public int MaxPostCount { get; init; } = 10;
    public int MaxComments { get; init; } = 10;
    public bool SkipComments { get; init; }
    public string? PostDateLimit { get; init; }
    public string? CommentDateLimit { get; init; }
    public object Proxy { get; init; } = new { UseApifyProxy = true, ApifyProxyGroups = new[] { "RESIDENTIAL" } };
}

public sealed class ScrapeInput
{
    public required object[] StartUrls { get; init; }
    public bool SkipComments { get; init; }
    public bool SkipUserPosts { get; init; }
    public bool SkipCommunity { get; init; }
    public int MaxItems { get; init; } = 10;
    public int MaxPostCount { get; init; } = 10;
    public int MaxComments { get; init; } = 10;
    public int MaxCommunitiesCount { get; init; } = 2;
    public int MaxUserCount { get; init; } = 2;
    public string? PostDateLimit { get; init; }
    public string? CommentDateLimit { get; init; }
    public object Proxy { get; init; } = new { UseApifyProxy = true, ApifyProxyGroups = new[] { "RESIDENTIAL" } };
}
