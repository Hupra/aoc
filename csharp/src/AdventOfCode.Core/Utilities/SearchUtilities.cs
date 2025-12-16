namespace AdventOfCode.Core.Utilities;

public static class SearchUtilities
{
    /// <summary>
    /// Performs a binary search to find the first index where the predicate returns true.
    /// </summary>
    /// <param name="left">The left bound (inclusive)</param>
    /// <param name="right">The right bound (inclusive)</param>
    /// <param name="predicate">The predicate to test</param>
    /// <returns>The first index where predicate is true, or right+1 if never true</returns>
    public static int BinarySearchFirst(int left, int right, Func<int, bool> predicate)
    {
        var result = right + 1;
        
        while (left <= right)
        {
            var mid = left + (right - left) / 2;
            
            if (predicate(mid))
            {
                result = mid;
                right = mid - 1;
            }
            else
            {
                left = mid + 1;
            }
        }
        
        return result;
    }
}