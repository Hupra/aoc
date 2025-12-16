using AdventOfCode.Core;
using System.Reflection;
using System.Diagnostics;

var (year, day, useTestFile) = ParseArgs(args);

// Find all IDay types with [Day] attribute
var days = DiscoverDays();

var match = days.SingleOrDefault(d => d.Attribute.Year == year && d.Attribute.Day == day);

if (match.Type is null)
{
    Console.Error.WriteLine($"No implementation found for {year}-Day{day:00}");
    return 1;
}

// Try to download input if it doesn't exist (only for real input, not test files)
if (!useTestFile)
{
    var downloader = new InputDownloader();
    await downloader.DownloadInputIfNotExists(year, day);
}

// Load input
var input = InputLoader.LoadInput(year, day, useTestFile);

// Instantiate and run
if (Activator.CreateInstance(match.Type) is not IDay solver)
    throw new InvalidOperationException($"Could not create instance of {match.Type.FullName}");

Console.WriteLine($"=== {year} Day {day:00} ===");
Console.WriteLine();

var sw = Stopwatch.StartNew();
var p1 = solver.SolvePart1(input);
sw.Stop();
Console.WriteLine($"Part 1: {p1}");
Console.WriteLine($"Time  : {sw.ElapsedMilliseconds}ms");
Console.WriteLine();

sw.Restart();
var p2 = solver.SolvePart2(input);
sw.Stop();
Console.WriteLine($"Part 2: {p2}");
Console.WriteLine($"Time  : {sw.ElapsedMilliseconds}ms");

return 0;

// ---------- local methods ----------
static (int Year, int Day, bool UseTestFile) ParseArgs(string[] args)
{
    // Usage: dotnet run --project src/AdventOfCode.Runner -- 2025 1 [-t]
    var useTestFile = args.Contains("-t");
    var numericArgs = args.Where(a => a != "-t").ToArray();

    if (numericArgs.Length >= 2 &&
        int.TryParse(numericArgs[0], out var year) &&
        int.TryParse(numericArgs[1], out var day))
    {
        return (year, day, useTestFile);
    }

    // default: latest AoC year, day 1
    return (2025, 1, useTestFile);
}

static (Type Type, DayAttribute Attribute)[] DiscoverDays()
{
    // Load all referenced assemblies from the entry assembly's directory
    var entryAssembly = Assembly.GetEntryAssembly()!;
    var assemblyPath = Path.GetDirectoryName(entryAssembly.Location)!;

    // Load all AdventOfCode.*.dll files in the same directory
    foreach (var dllPath in Directory.GetFiles(assemblyPath, "AdventOfCode.*.dll"))
    {
        try
        {
            Assembly.LoadFrom(dllPath);
        }
        catch
        {
            /* Ignore load failures */
        }
    }

    var assemblies = AppDomain.CurrentDomain
        .GetAssemblies()
        .Where(a => a.GetName().Name?.StartsWith("AdventOfCode") == true);

    return
    [
        .. assemblies
            .SelectMany(a =>
            {
                try
                {
                    return a.GetTypes();
                }
                catch (ReflectionTypeLoadException e)
                {
                    return e.Types.OfType<Type>().ToArray();
                }
            })
            .Where(t => typeof(IDay).IsAssignableFrom(t) && !t.IsAbstract)
            .Select(t => (Type: t, Attribute: t.GetCustomAttribute<DayAttribute>()))
            .Where(x => x.Attribute is not null)
            .Select(x => (x.Type, x.Attribute!))
    ];
}
