using System.Collections.ObjectModel;

using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;

using Whip4BratsGUI.Contracts.Services;
using Whip4BratsGUI.Contracts.ViewModels;
using Whip4BratsGUI.Core.Contracts.Services;
using Whip4BratsGUI.Core.Models;

namespace Whip4BratsGUI.ViewModels;

public partial class ContentGridViewModel : ObservableRecipient, INavigationAware
{
    private readonly INavigationService _navigationService;
    private readonly IFeatureListService _featureService;

    public ObservableCollection<Feature> Source { get; } = new ObservableCollection<Feature>();

    public ContentGridViewModel(INavigationService navigationService, IFeatureListService featureService)
    {
        _navigationService = navigationService;
        _featureService = featureService;
    }

    public async void OnNavigatedTo(object parameter)
    {
        Source.Clear();

        var data = await _featureService.GetContentGridAsync();
        foreach (var item in data)
        {
            Source.Add(item);
        }
    }

    public void OnNavigatedFrom()
    {
    }

    [RelayCommand]
    private void OnItemClick(Feature? clickedItem)
    {
        if (clickedItem != null)
        {
            _navigationService.SetListDataItemForNextConnectedAnimation(clickedItem);
            _navigationService.NavigateTo(typeof(ContentGridDetailViewModel).FullName!, clickedItem.FeatureID);
        }
    }
}
