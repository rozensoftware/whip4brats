using Whip4BratsGUI.Core.Contracts.Services;
using Whip4BratsGUI.Core.Models;
using Microsoft.Win32;
using System.Text.Json;

namespace Whip4BratsGUI.Core.Services;
public class WindowsRegistryService : IWindowsRegistryService
{
    private static readonly string PLAY_TIME_REG_KEY = "SOFTWARE\\Rozen Software\\Whip4Brats";
    private static readonly string PLAY_TIME_REG_NAME = "play_time";

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
}
