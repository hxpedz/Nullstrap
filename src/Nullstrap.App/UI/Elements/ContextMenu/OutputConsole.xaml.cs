using System;
using System.Windows;
using Nullstrap.Integrations;
using Nullstrap.UI.Elements.Base;
using Nullstrap.UI.ViewModels.ContextMenu;

namespace Nullstrap.UI.Elements.ContextMenu;

public partial class OutputConsole : WpfUiWindow
{
	private readonly OutputConsoleViewModel _viewModel;

	public OutputConsole(ActivityWatcher watcher)
	{
		_viewModel = new OutputConsoleViewModel(watcher);
		_viewModel.RequestCloseEvent += OnRequestClose;
		DataContext = _viewModel;
		InitializeComponent();
		Closed += OnClosed;
	}

	private void OnRequestClose(object? sender, EventArgs e)
	{
		Close();
	}

	private void OnClosed(object? sender, EventArgs e)
	{
		Closed -= OnClosed;
		_viewModel.RequestCloseEvent -= OnRequestClose;
		_viewModel.Dispose();
		DataContext = null;
	}
}
