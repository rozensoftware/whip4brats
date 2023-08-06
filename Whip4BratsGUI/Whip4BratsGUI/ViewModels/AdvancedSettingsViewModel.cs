using CommunityToolkit.Mvvm.ComponentModel;
using Whip4BratsGUI.Contracts.ViewModels;
using Whip4BratsGUI.Core.Contracts.Services;
using Whip4BratsGUI.Core.Models;

namespace Whip4BratsGUI.ViewModels;
public partial class AdvancedSettingsViewModel : ObservableRecipient, INavigationAware
{
    private readonly IFeatureListService _featureListService;
    private readonly IWindowsRegistryService _registryService;

    [ObservableProperty]
    private Feature? item;

    [ObservableProperty]
    private bool? disabled;

    public AdvancedSettingsViewModel(IFeatureListService featureListService, IWindowsRegistryService registryService)
    {
        _featureListService = featureListService;
        _registryService = registryService;
    }

    public void SetDisabled(bool b)
    {
        _registryService.SetDisabled(b);
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
