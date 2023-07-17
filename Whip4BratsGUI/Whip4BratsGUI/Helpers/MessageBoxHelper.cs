using Microsoft.UI.Xaml;

namespace Whip4BratsGUI.Helpers;
public class MessageBoxHelper
{
    public static async Task ShowErrorAsync(string message, XamlRoot xml)
    {
        var dialog = new Microsoft.UI.Xaml.Controls.ContentDialog
        {
            XamlRoot = xml,
            Title = "Error",
            Content = message,
            PrimaryButtonText = "Ok",
            DefaultButton = Microsoft.UI.Xaml.Controls.ContentDialogButton.Primary
        };

        _ = await dialog.ShowAsync();
    }
}
