using Nullstrap.Models.Attributes;

namespace Nullstrap.Enums;

public enum WindowCornerStyle
{
	[EnumName(StaticName = "Rounded")]
	Rounded,
	[EnumName(StaticName = "Slightly rounded")]
	SlightlyRounded,
	[EnumName(StaticName = "Square")]
	Square
}
