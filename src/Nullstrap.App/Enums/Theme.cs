using Nullstrap.Models.Attributes;

namespace Nullstrap.Enums;

public enum Theme
{
	[EnumName(FromTranslation = "Common.SystemDefault")]
	Default,
	Dark,
	Light,
	Nullstrap,
	UltraGray,
	Berry,
	Blue,
	Cyan,
	Green,
	Orange,
	Pink,
	Purple,
	Red,
	Yellow,
	Custom
}
