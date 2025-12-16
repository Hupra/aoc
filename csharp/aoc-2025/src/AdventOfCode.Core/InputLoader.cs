namespace AdventOfCode.Core;

public static class InputLoader
{
    public static string LoadInput(int year, int day, bool useTestFile = false)
    {
        // Expect: src/AdventOfCode.Y2025/Inputs/day01.txt etc.
        var baseDir = AppContext.BaseDirectory;

        // Runner runs from its own bin; go up to repo src folder:
        var dir = new DirectoryInfo(baseDir);
        while (dir is not null && !Directory.Exists(Path.Combine(dir.FullName, "src")))
            dir = dir.Parent;

        if (dir is null)
            throw new InvalidOperationException("Cannot locate repository root (no 'src' folder found).");

        var extension = useTestFile ? ".test" : ".txt";
        var inputsPath = Path.Combine(
            dir.FullName,
            "src",
            $"AdventOfCode.Y{year}",
            "Inputs",
            $"day{day:00}{extension}");

        return File.Exists(inputsPath)
            ? File.ReadAllText(inputsPath)
            : throw new FileNotFoundException($"Input file not found: {inputsPath}");
    }
}