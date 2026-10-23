using Nullstrap.Models.Attributes;

namespace Nullstrap.Enums.FlagPresets;

public enum LightingMode
{
	Default,
	Voxel,
	ShadowMap,
	Future,
	[EnumName(StaticName = "Unified (Phase 4)")]
	Unified
}
