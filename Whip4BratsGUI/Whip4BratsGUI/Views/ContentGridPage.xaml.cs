using System.Resources;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Whip4BratsGUI.Contracts.Services;
using Whip4BratsGUI.Core.Contracts.Services;
using Whip4BratsGUI.Core.Services;
using Whip4BratsGUI.ViewModels;

namespace Whip4BratsGUI.Views;

public sealed partial class ContentGridPage : Page
{
    public ContentGridViewModel ViewModel
    {
        get;
    }

    private readonly IAuxiliaryService _auxiliaryService = App.GetService<IAuxiliaryService>();
    private readonly IWindowsRegistryService _windowsRegistryService = App.GetService<IWindowsRegistryService>();

    public ContentGridPage()
    {
        ViewModel = App.GetService<ContentGridViewModel>();
        InitializeComponent();

        Loaded += ContentGridPage_Loaded;
    }

    private async void ContentGridPage_Loaded(object sender, RoutedEventArgs e)
    {
        if (!_auxiliaryService.IsParentLogged())
        {
            //get text from resource file
            var resource = new ResourceManager("Whip4BratsGUI.Core.Localization.Strings", typeof(FeatureListService).Assembly);

            ContentDialog dialog = new ContentDialog
            {
                Title = resource.GetString("EnterPassword"),
                Content = new PasswordPage(),
                PrimaryButtonText = "OK",
                DefaultButton = ContentDialogButton.Primary,
                XamlRoot = XamlRoot
            };

            //add event handler for dialog closing
            dialog.Closing += (s, args) =>
            {
                if (args.Result == ContentDialogResult.Primary)
                {
                    if (dialog.Content is PasswordPage passwordPage)
                    {
                        _windowsRegistryService.ReadCredentials(out var parentPassword, out var childUserName, out var childPassword);
                        var pass = _auxiliaryService.DecodeFromBase64(parentPassword);
                        if (passwordPage.ViewModel.ParentPassword == pass)
                        {
                            _auxiliaryService.SetParentLogged(true);
                        }
                        else
                        {
                            var navigationService = App.GetService<INavigationService>();
                            navigationService.GoBack();
                        }
                    }
                }
                else
                {
                    var navigationService = App.GetService<INavigationService>();
                    navigationService.GoBack();
                }
            };

            await dialog.ShowAsync();
        }
    }
}
