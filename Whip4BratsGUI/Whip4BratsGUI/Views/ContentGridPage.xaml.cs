using Microsoft.UI.Xaml.Controls;

using Whip4BratsGUI.ViewModels;

namespace Whip4BratsGUI.Views;

public sealed partial class ContentGridPage : Page
{
    public ContentGridViewModel ViewModel
    {
        get;
    }

    public ContentGridPage()
    {
        ViewModel = App.GetService<ContentGridViewModel>();
        InitializeComponent();
    }
}
