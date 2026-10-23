using Nullstrap.Models.Attributes;

namespace Nullstrap.Enums.FlagPresets;

public enum ProfileMode
{
	[EnumName(FromTranslation = "Common.Automatic")]
	Default,
	[EnumName(StaticName = "Nullstraps Official")]
	Nullstrap,
	[EnumName(StaticName = "Stoofs")]
	Stoof
}
