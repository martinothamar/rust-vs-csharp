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
        PassConcrete(banana);
        PassBoxed(banana);
        PassGeneric(in banana);
	}

    static void PassConcrete(in App.Banana banana) =>
        Console.WriteLine(banana.Calories);

    static void PassBoxed(App.IFruit fruit) =>
        Console.WriteLine(fruit.Calories);

    static void PassGeneric<T>(in T fruit) where T : struct, App.IFruit => 
        Console.WriteLine(fruit.Calories);
}
