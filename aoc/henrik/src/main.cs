private class FastHashSet
{
    private BitArray containerPositive = new BitArray(int.MaxValue, false);
    private BitArray containerNegative = new BitArray(int.MaxValue, false);
    public FastHashSet(int[] initialValues)
    {
        for (int i = 0; i < initialValues.Length; i++)
        {
            var val = initialValues[i];
            if (val > 0)
                containerPositive[val] = true;
            else
                containerNegative[val] = true;
        }
    }

    public bool Contains(int value) => value > 0 ? containerPositive[value] : containerNegative[value];
}

public List<(string, string, string)> FindThreeWordsThatCoversTheMostUsedLetters_Faster(int wordLength)
{
    var sw_init = Stopwatch.StartNew();

    var letterLookup = Enumerable.Range('a', 'z' - 'a').ToArray();
    var desiredLetters = dic.OrderByDescending(kvp => kvp.Value).Take(3 * wordLength).Select(kvp => kvp.Key).ToArray();
    var desiredBitmask = GetBitmask(new string(desiredLetters));

    var wordsOfCorrectLength = _words.Where(w => w.Length == wordLength).ToArray();
    Dictionary<int, List<string>> bitmaskLookup = new();
    foreach (var word in wordsOfCorrectLength)
    {
        var bitmask = GetBitmask(word);
        if ((bitmask & desiredBitmask) == bitmask)
        {
            if (bitmaskLookup.ContainsKey(bitmask) == false)
                bitmaskLookup[bitmask] = new List<string>();

            bitmaskLookup[bitmask].Add(word);
        }
    }

    var wordIdentities = wordsOfCorrectLength.Select(GetBitmask).ToArray();

    //var wordIdentitiesSet = wordIdentities.ToHashSet();
    var test = new FastHashSet(wordIdentities);

    var visited = new HashSet<(int, int, int)>();

    sw_init.Stop();
    var sw_exe = Stopwatch.StartNew();

    var results = new List<(int, int, int)>();
    for (int ia = 0; ia < wordIdentities.Length; ia++)
    {
        var a = wordIdentities[ia];
        for (int ib = ia + 1; ib < wordIdentities.Length; ib++)
        {
            var b = wordIdentities[ib];
            if ((a & b) != 0)
                continue;

            var ab = a | b;
            var c = desiredBitmask & ~ab;
            //if (wordIdentitiesSet.Contains(c) == false)
            //    continue;

            if (test.Contains(c) == false)
                continue;

            var arr = new int[] { a, b, c };
            Array.Sort(arr);
            var visitedItem = (arr[0], arr[1], arr[2]);
            if (visited.Add(visitedItem))
            {
                results.Add(visitedItem);
            }
        }
    }

    sw_exe.Stop();
    var sw_combine = Stopwatch.StartNew();

    var actualResults = new List<(string, string, string)>();
    foreach (var result in results)
    {
        var res = from a in bitmaskLookup[result.Item1]
                    from b in bitmaskLookup[result.Item2]
                    from c in bitmaskLookup[result.Item3]
                    select (a, b, c);
        actualResults.AddRange(res);
    }

    sw_combine.Stop();
    Console.WriteLine($"ini: {sw_init.ElapsedMilliseconds}, exe: {sw_exe.ElapsedMilliseconds}, combine: {sw_combine.ElapsedMilliseconds}");
    return actualResults;
}
[MethodImpl(MethodImplOptions.AggressiveInlining)]
private static int GetBitmask(string word)
{
    int letterBitmask = 0;
    foreach (char c in word)
    {
        letterBitmask |= 1 << (c - 'a');
    }

    return letterBitmask;
}