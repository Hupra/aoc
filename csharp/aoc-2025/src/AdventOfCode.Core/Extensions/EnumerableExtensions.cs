namespace AdventOfCode.Core.Extensions;

public static class EnumerableExtensions
{
    public static IEnumerable<T[]> SplitBy<T>(this IEnumerable<T> source, Func<T, bool> predicate)
    {
        var current = new List<T>();
        
        foreach (var item in source)
        {
            if (predicate(item))
            {
                if (current.Count <= 0) continue;
                yield return current.ToArray();
                current.Clear();
            }
            else
                current.Add(item);
        }
        
        if (current.Count > 0)
            yield return current.ToArray();
    }
}
