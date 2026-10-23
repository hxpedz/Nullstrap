using System;
using System.Diagnostics;
using System.Windows;
using System.Windows.Controls;
using Nullstrap.UI.ViewModels.Settings;

namespace Nullstrap.UI.Elements.Dialogs
{
    public partial class NewsItemDialog : UserControl
    {
        private readonly NewsItem _item;

        public event EventHandler? CloseRequested;

        public NewsItemDialog(NewsItem item)
        {
            _item = item ?? throw new ArgumentNullException(nameof(item));
            DataContext = _item;
            InitializeComponent();
        }

        private void CopyText_Click(object sender, RoutedEventArgs e)
        {
            try
            {
                var text = $"{_item.Title}\n{_item.Date:G}\n\n{_item.Content}";
                Nullstrap.Utility.ClipboardService.SetText(text);
            }
            catch (Exception ex)
            {
                Debug.WriteLine($"[NewsItemDialog::CopyText] {ex.Message}");
            }
        }

        private void Close_Click(object sender, RoutedEventArgs e)
        {
            CloseRequested?.Invoke(this, EventArgs.Empty);
        }

        private void TagButton_Click(object sender, RoutedEventArgs e)
        {
            if (sender is Button { Content: string url } && Utilities.IsWebLink(url))
                Utilities.OpenWebLink(url);
        }
    }
}
