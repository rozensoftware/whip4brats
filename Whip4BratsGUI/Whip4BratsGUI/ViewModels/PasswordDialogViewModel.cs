using CommunityToolkit.Mvvm.ComponentModel;

namespace Whip4BratsGUI.ViewModels;
public partial class PasswordDialogViewModel : ObservableRecipient
{
    [ObservableProperty]
    private string? parentPassword;

    public PasswordDialogViewModel()
    {
    }    
}
