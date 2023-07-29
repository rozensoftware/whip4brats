using Whip4BratsGUI.Core.Contracts.Services;
using Whip4BratsGUI.Core.Models;
using Microsoft.Win32;
using System.Text.Json;
using System.Resources;

namespace Whip4BratsGUI.Core.Services;
public class WindowsRegistryService : IWindowsRegistryService
{
    private static readonly ResourceManager _resource = new("Whip4BratsGUI.Core.Localization.Strings", typeof(FeatureListService).Assembly);
    private static readonly string PLAY_TIME_REG_KEY = "SOFTWARE\\Rozen Software\\Whip4Brats";
    private static readonly string PLAY_TIME_REG_NAME = "play_time";
    private static readonly string DOMAIN_NAME_REG_NAME = "domain_name";
    private static readonly string PARENTAL_PASSWORD_REG_NAME = "parental_control_password";
    private static readonly string SERVER_ADDRESS_REG_NAME = "server_address";
    private static readonly string USER_NAME_REG_NAME = "user_name";
    private static readonly string USER_PASSWORD_REG_NAME = "user_password";
    private static readonly string LOCAL_IP_ADDRESS = "127.0.0.1";

    public PlayCalendar ReadPlayTime()
    {
        // Open the registry key where the object is stored
#pragma warning disable CA1416 // Validate platform compatibility
        var key = Registry.LocalMachine.OpenSubKey(PLAY_TIME_REG_KEY, false);

        // Read the serialized data
        if (key is null)
        {
            key.Close();
            return new PlayCalendar();
        }

        var data = key.GetValue(PLAY_TIME_REG_NAME).ToString();
        key.Close();
#pragma warning restore CA1416 // Validate platform compatibility

        var playTimes = JsonSerializer.Deserialize<PlayCalendar>(data);
        return playTimes;
    }

    public bool WritePlayTime(PlayCalendar playTimes)
    {
        // Serialize the object
        var data = JsonSerializer.Serialize(playTimes);

        // Open the registry key where the object will be stored
#pragma warning disable CA1416 // Validate platform compatibility
        var key = Registry.LocalMachine.OpenSubKey(PLAY_TIME_REG_KEY, true);

        if (key is null)
        {
            key = Registry.LocalMachine.CreateSubKey(PLAY_TIME_REG_KEY);
            if (key is null)
            {
                return false;
            }
        }

        // Write the serialized data
        key.SetValue(PLAY_TIME_REG_NAME, data, RegistryValueKind.String);

        key.Close();
#pragma warning restore CA1416 // Validate platform compatibility

        return true;
    }

    public bool CheckIfRegistryKeyExists()
    {
        #pragma warning disable CA1416 // Validate platform compatibility
        var key = Registry.LocalMachine.OpenSubKey(PLAY_TIME_REG_KEY, false);
        #pragma warning restore CA1416 // Validate platform compatibility

        return key is not null;
    }

    /// <summary>
    /// Initializes registry settings for the first time.
    /// </summary>
    /// <exception cref="Exception">When could not create or write to registry the exception will be thrown.</exception>
    public void InitializeRegistrySettings()
    {
        var defaultPassword = "1234";

        #pragma warning disable CA1416 // Validate platform compatibility
        var keyUser = Registry.CurrentUser.OpenSubKey(PLAY_TIME_REG_KEY, true);
        if (keyUser is null)
        {
            keyUser = Registry.CurrentUser.CreateSubKey(PLAY_TIME_REG_KEY);
            if(keyUser is null)
            {            
                throw new Exception(_resource.GetString("registry_setting_failed"));
            }
        }

        keyUser.SetValue(PARENTAL_PASSWORD_REG_NAME, string.Empty, RegistryValueKind.String);
        keyUser.Close();

        var key = Registry.LocalMachine.OpenSubKey(PLAY_TIME_REG_KEY, true);

        if (key is null)
        {        
            key = Registry.LocalMachine.CreateSubKey(PLAY_TIME_REG_KEY);
            if (key is null)
            {            
                throw new Exception(_resource.GetString("registry_setting_failed"));
            }
        }

        var playTimes = AuxiliaryService.CreatePlayTimeCalendar();

        var data = JsonSerializer.Serialize(playTimes);

        key.SetValue(PLAY_TIME_REG_NAME, data, RegistryValueKind.String);
        key.SetValue(DOMAIN_NAME_REG_NAME, Environment.MachineName, RegistryValueKind.String);
        key.SetValue(PARENTAL_PASSWORD_REG_NAME, AuxiliaryService.EncodeToBase64(defaultPassword), RegistryValueKind.String);
        key.SetValue(SERVER_ADDRESS_REG_NAME, LOCAL_IP_ADDRESS, RegistryValueKind.String);
        key.SetValue(USER_NAME_REG_NAME, string.Empty, RegistryValueKind.String);
        key.SetValue(USER_PASSWORD_REG_NAME, AuxiliaryService.EncodeToBase64(defaultPassword), RegistryValueKind.String);

        key.Close();
        #pragma warning restore CA1416 // Validate platform compatibility
    }

    public void UpdateCredentials(string parentPassword, string childUserName, string childPassword)
    {
        #pragma warning disable CA1416 // Validate platform compatibility
        var key = Registry.LocalMachine.OpenSubKey(PLAY_TIME_REG_KEY, true);

        if (key is null)
        {        
            key = Registry.LocalMachine.CreateSubKey(PLAY_TIME_REG_KEY);
            if (key is null)
            {            
                throw new Exception(_resource.GetString("registry_setting_failed"));
            }
        }

        key.SetValue(PARENTAL_PASSWORD_REG_NAME, parentPassword, RegistryValueKind.String);
        key.SetValue(USER_NAME_REG_NAME, childUserName, RegistryValueKind.String);
        key.SetValue(USER_PASSWORD_REG_NAME, childPassword, RegistryValueKind.String);

        key.Close();

        key = Registry.CurrentUser.OpenSubKey(PLAY_TIME_REG_KEY, true);

        if (key is null)
        {
            key = Registry.CurrentUser.CreateSubKey(PLAY_TIME_REG_KEY);
            if (key is null)
            {
                throw new Exception(_resource.GetString("registry_setting_failed"));
            }
        }

        key.SetValue(PARENTAL_PASSWORD_REG_NAME, parentPassword, RegistryValueKind.String);
        key.Close();
#pragma warning restore CA1416 // Validate platform compatibility
    }

    public void ReadCredentials(out string parentPassword, out string childUserName, out string childPassword)
    {
        parentPassword = string.Empty;
        childUserName = string.Empty;
        childPassword = string.Empty;

        #pragma warning disable CA1416 // Validate platform compatibility
        var key = Registry.LocalMachine.OpenSubKey(PLAY_TIME_REG_KEY, false);

        if (key is null)
        {
            return;
        }

        parentPassword = key.GetValue(PARENTAL_PASSWORD_REG_NAME).ToString();
        childUserName = key.GetValue(USER_NAME_REG_NAME).ToString();
        childPassword = key.GetValue(USER_PASSWORD_REG_NAME).ToString();

        key.Close();
#pragma warning restore CA1416 // Validate platform compatibility
    }
}
