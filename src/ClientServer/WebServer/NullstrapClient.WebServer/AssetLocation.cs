using System.Text.Json.Serialization;

namespace NullstrapClient.WebServer;

internal class AssetLocation
{
	[JsonPropertyName("location")]
	public string? Location { get; set; }
}
