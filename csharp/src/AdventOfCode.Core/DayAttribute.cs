namespace AdventOfCode.Core;

[AttributeUsage(AttributeTargets.Class)]
public class DayAttribute(int year, int day) : Attribute
{
    public int Year { get; } = year;
    public int Day { get; } = day;
}