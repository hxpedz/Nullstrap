using Wpf.Ui.Controls;
using Nullstrap.UI.ViewModels.Settings;

namespace Nullstrap.UI.Elements.Settings.Pages;

public partial class OverlaysPage : UiPage
{
	public OverlaysPage()
	{
		InitializeComponent();
		Loaded += (s, e) =>
		{
			DataContext ??= new ModsViewModel();
			(DataContext as ModsViewModel)?.PopulateKeystrokeThemes();
		};
	}
}
