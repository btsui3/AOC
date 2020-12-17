using System;
using System.Net.Http;

namespace Csharp
{
    public class Day1
    {
        public static async System.Threading.Tasks.Task<string> GetInputAsync()
        {
            var baseAddress = new Uri("https://adventofcode.com/2020/day");
            using (var handler = new HttpClientHandler { UseCookies = false })
            using (var client = new HttpClient(handler) { BaseAddress = baseAddress })
            {
                var message = new HttpRequestMessage(HttpMethod.Get, "1/input");
                message.Headers.Add("Cookie", "53616c7465645f5fdba1ca082a2a18c88311d76b8744021d54793ca0fe09ce7a361fc8ab754b4ddb6d0ea24a424a76b6");
                var result = await client.SendAsync(message);
                result.EnsureSuccessStatusCode();
                Console.WriteLine(result.ToString());
            }


            return null;
        }
    }
}
