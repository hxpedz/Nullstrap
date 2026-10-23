using Nullstrap.UI.ViewModels.Settings;
using Wpf.Ui.Controls;

namespace Nullstrap.UI.Elements.Settings.Pages;

public partial class RosemaryPage : UiPage
{
	public RosemaryPage()
	{
		DataContext = new RosemaryViewModel();
		InitializeComponent();
	}
}
