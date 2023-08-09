using System.Diagnostics;
using System.Resources;
using System.ServiceProcess;
using System.Text;
using Whip4BratsGUI.Core.Contracts.Services;
using Whip4BratsGUI.Core.Models;

namespace Whip4BratsGUI.Core.Services;
public class AuxiliaryService : IAuxiliaryService
{
    private static readonly ResourceManager _resource = new("Whip4BratsGUI.Core.Localization.Strings", typeof(FeatureListService).Assembly);

    private readonly IWindowsRegistryService _windowsRegistryService;

    private bool _isParentLogged;

    public AuxiliaryService(IWindowsRegistryService windowsRegistryService)
    {    
        _windowsRegistryService = windowsRegistryService;
        _isParentLogged = false;
    }
    
    public IList<string> GetWeekDays()
    {
        return new List<string>
        {
            _resource.GetString("Sunday"),
            _resource.GetString("Monday"),
            _resource.GetString("Tuesday"),
            _resource.GetString("Wednesday"),
            _resource.GetString("Thursday"),
            _resource.GetString("Friday"),
            _resource.GetString("Saturday")
        };
    }

    /// <summary>
    /// Initializes system to use for the first time.
    /// </summary>
    /// <exception cref="Exception">Thrown when registry key cannot be created.</exception></exception>
    public void Initialize()
    {
        if (!_windowsRegistryService.CheckIfRegistryKeyExists())
        {
            _windowsRegistryService.InitializeRegistrySettings();
        }
    }
    
    public static string EncodeToBase64(string toEncode)
    {    
        var toEncodeAsBytes = Encoding.ASCII.GetBytes(toEncode);
        var returnValue = Convert.ToBase64String(toEncodeAsBytes);
        return returnValue;
    }

    public string DecodeFromBase64(string encodedData)
    {    
        var encodedDataAsBytes = Convert.FromBase64String(encodedData);
        var returnValue = Encoding.ASCII.GetString(encodedDataAsBytes);
        return returnValue;
    }

    public static PlayCalendar CreatePlayTimeCalendar()
    {
        var calendar = new PlayCalendar
        {
            days = new List<PlayTime>()
        };

        for (var i = 0; i < 7; i++)
        {        
            var playTime = new PlayTime()
            {
                day = i,
                start_time_hour = 9,
                start_time_minutes = 0,
                end_time_hour = 21,
                end_time_minutes = 0
            };

            calendar.days.Add(playTime);
        }

        return calendar;
    }

    public bool IsParentLogged()
    {
        return _isParentLogged;
    }

    public void SetParentLogged(bool isLogged)
    {
        _isParentLogged = isLogged; 
    }

    //Get path where this program is running in
    public string GetProgramPath()
    {       
        var path = System.Reflection.Assembly.GetExecutingAssembly().Location;
        var directory = System.IO.Path.GetDirectoryName(path);
        return directory;
    }

    public void RunExternalProgram(string programPath)
    {    
        var process = new Process();
        process.StartInfo.FileName = programPath;
        process.StartInfo.UseShellExecute = false;
        process.StartInfo.Arguments = "--register";
        process.Start();
    }

    public bool IsServiceRunning(string serviceName)
    {
        #pragma warning disable CA1416 // Validate platform compatibility
        
        var services = ServiceController.GetServices();
        var service = services.FirstOrDefault(s => s.ServiceName == serviceName);
        return service != null && service.Status == ServiceControllerStatus.Running;
        
        #pragma warning restore CA1416 // Validate platform compatibility
    }

    public void StartService(string serviceName)
    {    
        #pragma warning disable CA1416 // Validate platform compatibility
           
        var services = ServiceController.GetServices();
        var service = services.FirstOrDefault(s => s.ServiceName == serviceName);
        if (service != null && service.Status != ServiceControllerStatus.Running)
        {        
            service.Start();
        }
        
        #pragma warning restore CA1416 // Validate platform compatibility
    }
}
