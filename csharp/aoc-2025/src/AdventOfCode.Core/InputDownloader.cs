namespace AdventOfCode.Core;

public class InputDownloader(string? sessionCookie = null, string? repoRoot = null)
{
    private readonly string _sessionCookie = sessionCookie ?? GetSessionCookie() ?? string.Empty;

    private readonly string _repoRoot =
        repoRoot ?? FindRepoRoot(Directory.GetCurrentDirectory()) ?? Directory.GetCurrentDirectory();

    public async Task<bool> DownloadInputIfNotExists(int year, int day, bool force = false)
    {
        var inputFile = GetInputFilePath(year, day);

        if (File.Exists(inputFile) && !force)
        {
            Console.WriteLine($"Input file already exists: {inputFile}");
            return true;
        }

        if (string.IsNullOrWhiteSpace(_sessionCookie))
        {
            Console.WriteLine("Session cookie not found. Please set AOC_SESSION environment variable.");
            Console.WriteLine(
                "Get your session cookie from adventofcode.com (browser dev tools > Application > Cookies)");
            return false;
        }

        Console.WriteLine($"Downloading input for {year} day {day}...");
        var success = await DownloadInput(year, day, inputFile);

        Console.WriteLine(success ? $"✓ Input saved to: {inputFile}" : "Failed to download input file.");

        return success;
    }

    private async Task<bool> DownloadInput(int year, int day, string outputFile)
    {
        using var client = new HttpClient();
        client.DefaultRequestHeaders.Add("Cookie", $"session={_sessionCookie}");

        try
        {
            var url = $"https://adventofcode.com/{year}/day/{day}/input";
            var response = await client.GetAsync(url);

            if (!response.IsSuccessStatusCode)
            {
                Console.WriteLine($"HTTP Error: {response.StatusCode}");
                if (response.StatusCode == System.Net.HttpStatusCode.BadRequest)
                {
                    Console.WriteLine("Bad session cookie or puzzle not yet available.");
                }

                return false;
            }

            var inputsDir = Path.GetDirectoryName(outputFile);
            if (inputsDir != null && !Directory.Exists(inputsDir))
            {
                Directory.CreateDirectory(inputsDir);
            }

            var content = await response.Content.ReadAsStringAsync();
            await File.WriteAllTextAsync(outputFile, content);
            return true;
        }
        catch (Exception ex)
        {
            Console.WriteLine($"Error downloading: {ex.Message}");
            return false;
        }
    }

    private string GetInputFilePath(int year, int day)
    {
        var dayStr = day.ToString("D2");
        return Path.Combine(_repoRoot, "src", $"AdventOfCode.Y{year}", "Inputs", $"day{dayStr}.txt");
    }

    private static string? FindRepoRoot(string startPath)
    {
        var current = new DirectoryInfo(startPath);
        while (current != null)
        {
            if (Directory.Exists(Path.Combine(current.FullName, "src")))
            {
                return current.FullName;
            }

            current = current.Parent;
        }

        return null;
    }

    private static string? GetSessionCookie()
    {
        var cookie = Environment.GetEnvironmentVariable("AOC_SESSION");
        if (!string.IsNullOrWhiteSpace(cookie))
        {
            return cookie;
        }

        var repoRoot = FindRepoRoot(Directory.GetCurrentDirectory());
        if (repoRoot != null)
        {
            var envFile = Path.Combine(repoRoot, ".env");
            if (File.Exists(envFile))
            {
                var lines = File.ReadAllLines(envFile);
                foreach (var line in lines)
                {
                    if (line.StartsWith("AOC_SESSION="))
                    {
                        return line.Substring("AOC_SESSION=".Length).Trim();
                    }
                }
            }
        }

        return null;
    }
}
