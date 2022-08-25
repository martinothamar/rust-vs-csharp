namespace App
{
    public interface IFruit 
    {
        static abstract uint CaloriesPer100G { get; }

        uint WeightG { get; init; }

        uint Calories { get; }
    }

    public readonly record struct Banana(uint WeightG) : IFruit
    {
        public static uint CaloriesPer100G => 89;

        public uint Calories => CaloriesPer100G * WeightG / 100;
    }
}
	
static class Program 
{
	static void Main()
	{
		var banana = new App.Banana(120);
        var calories = banana.Calories;
        Console.WriteLine(calories);
	}
}
