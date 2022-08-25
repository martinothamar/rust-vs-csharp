// namespace App
// {
//     public struct Person 
//     {
//         public string FirstName { readonly get; private set; }
//         public string LastName { readonly get; private set; }

// 		public Person(string firstName, string lastName) 
// 		{
// 			FirstName = firstName;
// 			LastName = lastName;
// 		}

//         public readonly string GetFullName() => $"{FirstName} {LastName}";
//     }
// }
	
// static class Program 
// {
// 	static void Main()
// 	{
// 		var person = new App.Person("Martin", "Othamar");
// 		 // Pass by value - 16 bytes copied
// 		LogByValue(person);
// 		 // Pass by value by mutable reference - 8 bytes copied
// 		LogByRef(ref person);
// 		 // Pass by value by readonly reference - 8 bytes copied
// 		LogByReadonlyRef(in person);
// 	}

	
// 	static void LogByValue(App.Person person) => 
// 		Console.WriteLine(person.GetFullName());

	
// 	static void LogByRef(ref App.Person person) => 
// 		Console.WriteLine(person.GetFullName());

	
// 	static void LogByReadonlyRef(in App.Person person) => 
// 		Console.WriteLine(person.GetFullName());
// }
