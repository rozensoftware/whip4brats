using System.Collections.ObjectModel;
using CommunityToolkit.Mvvm.ComponentModel;
using Microsoft.UI.Xaml.Controls;
using Whip4BratsGUI.Contracts.ViewModels;
using Whip4BratsGUI.Core.Contracts.Services;
using Whip4BratsGUI.Core.Models;
using Whip4BratsGUI.Core.Services;
using Whip4BratsGUI.Helpers;

namespace Whip4BratsGUI.ViewModels;

public partial class ContentGridDetailViewModel : ObservableRecipient, INavigationAware
{
    private readonly IFeatureListService _featureListService;
    private readonly IWindowsRegistryService _windowsRegistryService;
    private readonly IAuxiliaryService _auxiliaryService;

    [ObservableProperty]
    private Feature? item;

    [ObservableProperty]
    private PlayCalendar? playTime;

    [ObservableProperty]
    private string? startTime;

    [ObservableProperty]
    private string? endTime;

    [ObservableProperty]
    private string? parentPassword;

    [ObservableProperty]
    private string? childPassword;

    [ObservableProperty]
    private string? childUserName;

    private bool _internalTimeChange = false;
    private int _selectedDayIdx = 0;
    private int _currentFeatureId = -1;
    private readonly List<string> _daysList;

    private string? prevParentPassword;
    private string? prevChildPassword;
    private string? prevChildUserName;

    public ObservableCollection<string> Days { get; } = new ObservableCollection<string>();

    public ContentGridDetailViewModel(IFeatureListService featureListService, 
        IWindowsRegistryService windowsRegistryService, IAuxiliaryService auxiliaryService)
    {
        _featureListService = featureListService;
        _windowsRegistryService = windowsRegistryService;
        _auxiliaryService = auxiliaryService;

        _daysList = auxiliaryService.GetWeekDays().ToList();
    }

    public void UpdateSelectedDay(object sender, SelectionChangedEventArgs e)
    {
        var item = sender as ListView;
        if (item is not null)
        {
            var day = item.SelectedValue as string;
            if (day is not null && PlayTime is not null)
            {
                _internalTimeChange = true;

                //find index of selected day
                var index = _daysList.IndexOf(day);
                _selectedDayIdx = index;

                var t = new TimeSpan(PlayTime.days[index].start_time_hour, PlayTime.days[index].start_time_minutes, 0);
                var d = DateTime.Today.Add(t);
                StartTime = d.ToString("HH:mm");
            
                t = new TimeSpan(PlayTime.days[index].end_time_hour, PlayTime.days[index].end_time_minutes, 0);
                d = DateTime.Today.Add(t);
                EndTime = d.ToString("HH:mm");

                _internalTimeChange = false;
            }
        }
    }

    public async Task SetNewStartTimeAsync(string time)
    {    
        if(_internalTimeChange || PlayTime is null)
        {        
            return;
        }

        var save_time = false;

        var val = int.Parse(time[..2]);
        if (PlayTime.days[_selectedDayIdx].start_time_hour != val)
        {
            PlayTime.days[_selectedDayIdx].start_time_hour = int.Parse(time[..2]);
            save_time = true;
        }

        val = int.Parse(time[3..5]);
        if (PlayTime.days[_selectedDayIdx].start_time_minutes != val)
        {
            PlayTime.days[_selectedDayIdx].start_time_minutes = int.Parse(time[3..5]);
            save_time = true;
        }

        if (save_time)
        {
            try
            {
                _windowsRegistryService.WritePlayTime(PlayTime);
            }
            catch (Exception e)
            {
                await MessageBoxHelper.ShowErrorAsync(e.Message, App.MainWindow.Content.XamlRoot);
            }
        }
    }

    public async Task SetNewEndTimeAsync(string time)
    {
        if (_internalTimeChange || PlayTime is null)
        {
            return;
        }

        var save_time = false;

        var val = int.Parse(time[..2]);
        if (PlayTime.days[_selectedDayIdx].end_time_hour != val)
        {
            PlayTime.days[_selectedDayIdx].end_time_hour = int.Parse(time[..2]);
            save_time = true;
        }
        
        val = int.Parse(time[3..5]);
        if (PlayTime.days[_selectedDayIdx].end_time_minutes != val)
        {
            PlayTime.days[_selectedDayIdx].end_time_minutes = int.Parse(time[3..5]);
            save_time = true;
        }

        if (save_time)
        {
            try
            {
                _windowsRegistryService.WritePlayTime(PlayTime);
            }
            catch (Exception e)
            {
                await MessageBoxHelper.ShowErrorAsync(e.Message, App.MainWindow.Content.XamlRoot);
            }
        }
    }

    public async void OnNavigatedTo(object parameter)
    {
        if (parameter is long featureID)
        {
            _currentFeatureId = (int)featureID;
            var data = await _featureListService.GetContentGridAsync();
            Item = data.First(i => i.FeatureID == featureID);

            PlayTime = _windowsRegistryService.ReadPlayTime();
            
            Days.Clear();

            foreach (var day in _daysList)
            {            
                Days.Add(day);
            }

            _windowsRegistryService.ReadCredentials(out var pp, out var cu, out var cp);

            ParentPassword = _auxiliaryService.DecodeFromBase64(pp);
            ChildUserName = cu;
            ChildPassword = _auxiliaryService.DecodeFromBase64(cp);

            prevParentPassword = ParentPassword;
            prevChildPassword = ChildPassword;
            prevChildUserName = ChildUserName;
        }
    }

    public void OnNavigatedFrom()
    {
        if(_currentFeatureId == FeatureListService.FEATURE_PASSWORD_ID)
        {
            if (string.IsNullOrEmpty(ParentPassword) || string.IsNullOrEmpty(ChildPassword) || string.IsNullOrEmpty(ChildUserName))
            {
                var resource = new System.Resources.ResourceManager("Whip4BratsGUI.Core.Localization.Strings", typeof(FeatureListService).Assembly);

                _ = MessageBoxHelper.ShowErrorAsync(resource.GetString("AllFieldsWarning")!, App.MainWindow.Content.XamlRoot);
                return;
            }

            if (prevParentPassword != ParentPassword || prevChildPassword != ChildPassword || prevChildUserName != ChildUserName)
            {
                try
                {
                    _windowsRegistryService.UpdateCredentials(AuxiliaryService.EncodeToBase64(ParentPassword), ChildUserName,
                        AuxiliaryService.EncodeToBase64(ChildPassword));
                    _auxiliaryService.SetParentLogged(false);
                }
                catch (Exception e)
                {                
                    _ = MessageBoxHelper.ShowErrorAsync(e.Message, App.MainWindow.Content.XamlRoot);
                }
            }
        }
    }
}
