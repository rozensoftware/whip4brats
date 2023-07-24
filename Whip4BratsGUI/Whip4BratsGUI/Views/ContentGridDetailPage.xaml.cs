using CommunityToolkit.WinUI.UI.Animations;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Navigation;

using Whip4BratsGUI.Contracts.Services;
using Whip4BratsGUI.Core.Services;
using Whip4BratsGUI.Helpers;
using Whip4BratsGUI.ViewModels;

namespace Whip4BratsGUI.Views;

public sealed partial class ContentGridDetailPage : Page
{
    public ContentGridDetailViewModel ViewModel
    {
        get;
    }

    public ContentGridDetailPage()
    {
        ViewModel = App.GetService<ContentGridDetailViewModel>();
        InitializeComponent();

        Loaded += ContentGridDetailPage_Loaded;
    }

    private void ContentGridDetailPage_Loaded(object sender, RoutedEventArgs e)
    {       
        if(ViewModel.Item != null)
        {
            if (ViewModel.Item.FeatureID == FeatureListService.FEATURE_PLAY_TIME_ID)
            {
                playTimes.Visibility = Visibility.Visible;
                passwords.Visibility = Visibility.Collapsed;
            }
            else if (ViewModel.Item.FeatureID == FeatureListService.FEATURE_PASSWORD_ID)
            {            
                playTimes.Visibility = Visibility.Collapsed;
                passwords.Visibility = Visibility.Visible;
            }
        }
    }

    protected override void OnNavigatedTo(NavigationEventArgs e)
    {
        base.OnNavigatedTo(e);
        this.RegisterElementForConnectedAnimation("animationKeyContentGrid", itemHero);
    }

    protected override void OnNavigatingFrom(NavigatingCancelEventArgs e)
    {
        base.OnNavigatingFrom(e);
        if (e.NavigationMode == NavigationMode.Back)
        {
            var navigationService = App.GetService<INavigationService>();

            if (ViewModel.Item != null)
            {
                navigationService.SetListDataItemForNextConnectedAnimation(ViewModel.Item);
            }
        }
    }

    private async void ClockStart_TimeChanged(object sender, TimePickerValueChangedEventArgs e)
    {
        await ViewModel.SetNewStartTimeAsync(e.NewTime.ToString());
    }

    private async void ClockEnd_TimeChanged(object sender, TimePickerValueChangedEventArgs e)
    {
        await ViewModel.SetNewEndTimeAsync(e.NewTime.ToString());
    }
}
