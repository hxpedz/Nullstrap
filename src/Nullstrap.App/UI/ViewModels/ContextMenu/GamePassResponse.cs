using System.Collections.Generic;

namespace Nullstrap.UI.ViewModels.ContextMenu;

internal sealed class GamePassResponse
{
	public List<GamePassData> GamePasses { get; set; } = new List<GamePassData>();
}
