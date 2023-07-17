using System.Resources;
using Whip4BratsGUI.Core.Contracts.Services;

namespace Whip4BratsGUI.Core.Services;
public class AuxiliaryService : IAuxiliaryService
{
    private static readonly ResourceManager _resource = new("Whip4BratsGUI.Core.Localization.Strings", typeof(FeatureListService).Assembly);

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
}
