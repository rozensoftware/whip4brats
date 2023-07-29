using CommunityToolkit.Mvvm.ComponentModel;
using Whip4BratsGUI.Contracts.ViewModels;
using Whip4BratsGUI.Core.Contracts.Services;
using Whip4BratsGUI.Core.Models;

namespace Whip4BratsGUI.ViewModels;
public partial class AdvancedSettingsViewModel : ObservableRecipient, INavigationAware
{
    private readonly IFeatureListService _featureListService;

    [ObservableProperty]
    private Feature? item;

    public AdvancedSettingsViewModel(IFeatureListService featureListService)
    {
        _featureListService = featureListService;
    }


    public async void OnNavigatedTo(object parameter)
    {
        if (parameter is long featureID)
        {
            var data = await _featureListService.GetContentGridAsync();
            Item = data.First(i => i.FeatureID == featureID);
        }
    }

    public void OnNavigatedFrom()
    {
    }
}
