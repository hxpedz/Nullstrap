using System.Collections.Generic;

namespace Nullstrap.Integrations.Animation;

public sealed class RigDefinition
{
	public string RootPart { get; init; } = "";

	public List<RigPart> Parts { get; } = [];

	public List<RigJoint> Joints { get; } = [];
}
