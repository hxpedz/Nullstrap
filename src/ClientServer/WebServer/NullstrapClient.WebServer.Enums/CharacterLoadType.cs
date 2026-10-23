using System.Text.Json.Serialization;

namespace NullstrapClient.WebServer.Enums;

[JsonConverter(typeof(JsonStringEnumConverter))]
internal enum CharacterLoadType
{
	Fetch,
	Whole
}
