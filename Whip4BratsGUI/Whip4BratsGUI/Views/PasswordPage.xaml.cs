using Microsoft.UI.Xaml.Controls;
using Whip4BratsGUI.ViewModels;

namespace Whip4BratsGUI.Views;

public sealed partial class PasswordPage : Page
{
    public PasswordDialogViewModel ViewModel
    {
        get;
    }

    public PasswordPage()
    {
        ViewModel = App.GetService<PasswordDialogViewModel>();
        InitializeComponent();
    }
}
