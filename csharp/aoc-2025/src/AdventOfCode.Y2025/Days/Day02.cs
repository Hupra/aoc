using AdventOfCode.Core;

namespace AdventOfCode.Y2025.Days;

[Day(2025, 2)]
public sealed class Day02 : IDay
{
    public string SolvePart1(string input)
    {
        return input.Split(',').Select(s => s.Split('-').Select(long.Parse)).Select(ranges =>
        {
            var enumerable = ranges.ToList();
            var s = enumerable.First();
            var e = enumerable.Last();
            long sum = 0;
            for (var i = s; i <= e; i++)
            {
                var str = i.ToString();
                var h = str.Length / 2;
                if (str[..h] == str[h..])
                    sum += i;
            }

            return sum;
        }).Sum().ToString();
    }

    public string SolvePart2(string input)
    {
        return input.Split(',').Select(s => s.Split('-').Select(long.Parse)).Select(ranges =>
        {
            var enumerable = ranges as long[] ?? ranges.ToArray();
            var s = enumerable.First();
            var e = enumerable.Last();
            long sum = 0;
            for (var i = s; i <= e; i++)
            {
                var validId = true;
                var str = i.ToString();
                var h = str.Length / 2;

                // 1, 2, 3, 4, 5, len/2
                for (var sub = 1; sub <= h; sub++)
                {
                    // Must be multiple of sub, otherwise can't repeat evenly
                    if (str.Length % sub != 0) continue;

                    var fact = str[..sub];
                    var repeating = true;
                    for (var check = sub; check <= str.Length - sub; check += sub)
                    {
                        if (str.Substring(check, sub) == fact) continue;
                        repeating = false;
                        break;
                    }

                    if (!repeating) continue;
                    validId = false;
                    break;
                }

                if (!validId)
                {
                    // Console.WriteLine($"Invalid ID: {i}");
                    sum += i;
                }
            }

            return sum;
        }).Sum().ToString();
    }
}
