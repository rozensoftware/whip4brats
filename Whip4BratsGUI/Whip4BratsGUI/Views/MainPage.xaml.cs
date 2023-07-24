using Microsoft.UI.Xaml.Controls;
using Whip4BratsGUI.Core.Contracts.Services;
using Whip4BratsGUI.ViewModels;

namespace Whip4BratsGUI.Views;

public sealed partial class MainPage : Page
{
    public MainViewModel ViewModel
    {
        get;
    }

    public MainPage()
    {
        ViewModel = App.GetService<MainViewModel>();
        InitializeComponent();

        App.GetService<IAuxiliaryService>().Initialize();
    }
}
