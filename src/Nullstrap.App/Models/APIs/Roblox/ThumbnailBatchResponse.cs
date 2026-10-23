using System;
using System.Text.Json.Serialization;

namespace Nullstrap.Models.APIs.Roblox;

internal sealed class ThumbnailBatchResponse
{
	[JsonPropertyName("data")]
	public ThumbnailResponse[] Data { get; set; } = Array.Empty<ThumbnailResponse>();
}
