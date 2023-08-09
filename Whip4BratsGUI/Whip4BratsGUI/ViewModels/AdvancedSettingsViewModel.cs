using CommunityToolkit.Mvvm.ComponentModel;
using Whip4BratsGUI.Contracts.ViewModels;
using Whip4BratsGUI.Core.Contracts.Services;
using Whip4BratsGUI.Core.Models;
using Whip4BratsGUI.Helpers;

namespace Whip4BratsGUI.ViewModels;
public partial class AdvancedSettingsViewModel : ObservableRecipient, INavigationAware
{
    private readonly IFeatureListService _featureListService;
    private readonly IWindowsRegistryService _registryService;
    private readonly IAuxiliaryService _auxiliaryService;

    private static readonly string WHIP4BRATS_SERVICE_NAME = "BratService";
    private static readonly string WHIP4BRATS_FILENAME = "brat-server.exe";

    [ObservableProperty]
    private Feature? item;

    [ObservableProperty]
    private bool? disabled;

    [ObservableProperty]
    private bool isServiceRunning;

    public AdvancedSettingsViewModel(IFeatureListService featureListService, 
        IWindowsRegistryService registryService, IAuxiliaryService auxiliaryService)
    {
        _featureListService = featureListService;
        _registryService = registryService;
        _auxiliaryService = auxiliaryService;

        IsServiceRunning = !_auxiliaryService.IsServiceRunning(WHIP4BRATS_SERVICE_NAME);
    }

    public void SetDisabled(bool b)
    {
        _registryService.SetDisabled(b);
    }

    public void StartService()
    {
        try
        {
            var currPath = _auxiliaryService.GetProgramPath();
            //remove the last part of the path
            currPath = currPath.Substring(0, currPath.LastIndexOf('\\'));
            var exe = Path.Combine(currPath, WHIP4BRATS_FILENAME);
            _auxiliaryService.RunExternalProgram(exe);
            _auxiliaryService.StartService(WHIP4BRATS_SERVICE_NAME);
        }
        catch (Exception e)
        {
            _ = MessageBoxHelper.ShowErrorAsync(e.Message, App.MainWindow.Content.XamlRoot);
        }
    }

    public async void OnNavigatedTo(object parameter)
    {
        if (parameter is long featureID)
        {
            var data = await _featureListService.GetContentGridAsync();
            Item = data.First(i => i.FeatureID == featureID);
            Disabled = _registryService.IsDisabled();
        }
    }

    public void OnNavigatedFrom()
    {
    }
}
