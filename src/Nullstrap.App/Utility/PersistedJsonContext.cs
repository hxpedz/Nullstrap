using System.Collections.Generic;
using System.Text.Json.Serialization;
using Nullstrap.Models.Persistable;

namespace Nullstrap.Utility;

[JsonSerializable(typeof(AppSettings))]
[JsonSerializable(typeof(State))]
[JsonSerializable(typeof(RobloxState))]
[JsonSerializable(typeof(DownloadStats))]
[JsonSerializable(typeof(Dictionary<string, object>))]
internal sealed partial class PersistedJsonContext : JsonSerializerContext
{
}
