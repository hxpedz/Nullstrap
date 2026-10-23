using System.Text.Json;
using System.Text.Json.Serialization;

namespace Nullstrap.Models.NullstrapRPC;

public class Message
{
	[JsonPropertyName("command")]
	public string Command { get; set; } = null!;

	[JsonPropertyName("data")]
	public JsonElement Data { get; set; }
}
