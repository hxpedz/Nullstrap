using System.Text.Json.Serialization;

namespace NullstrapClient.WebServer.Enums;

[JsonConverter(typeof(JsonStringEnumConverter))]
internal enum SignatureType
{
	None,
	Legacy,
	RbxSig,
	RbxSig2,
	RbxSig4
}
