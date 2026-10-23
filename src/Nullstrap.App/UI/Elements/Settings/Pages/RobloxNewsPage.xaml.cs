using System.Windows;
using System.Windows.Controls;
using Nullstrap.UI.ViewModels.Settings;

namespace Nullstrap.UI.Elements.Settings.Pages
{
    public partial class RobloxNewsPage : Page
    {
        private readonly RobloxNewsViewModel _viewModel;

        public RobloxNewsPage()
        {
            _viewModel = new RobloxNewsViewModel();
            DataContext = _viewModel;
            InitializeComponent();
            Loaded += OnPageLoaded;
        }

        private void OnPageLoaded(object sender, RoutedEventArgs e)
        {
            _ = _viewModel.LoadAsync(false);
        }
    }
}
