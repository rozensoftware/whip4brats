using System.Resources;
using Whip4BratsGUI.Core.Contracts.Services;
using Whip4BratsGUI.Core.Models;

namespace Whip4BratsGUI.Core.Services;

public class FeatureListService : IFeatureListService
{
    public static readonly int FEATURE_PLAY_TIME_ID = 1;
    public static readonly int FEATURE_PASSWORD_ID = 2;

    private List<Feature> _allFeatures;
    private static readonly ResourceManager _resource = new("Whip4BratsGUI.Core.Localization.Strings", typeof(FeatureListService).Assembly);

    private static IEnumerable<Feature> AllFeatures()
    {

        return new List<Feature>()
        {
            new Feature()
            {
                Description = _resource.GetString("clock_description"),
                FeatureID = FEATURE_PLAY_TIME_ID,
                FeatureName = _resource.GetString("clock_name"),
                SymbolCode = 0xEC92
            },
            new Feature()
            {
                Description = _resource.GetString("password_description"),
                FeatureID = FEATURE_PASSWORD_ID,
                FeatureName = _resource.GetString("password_name"),
                SymbolCode = 0xE8BD
            },
        };
    }

    public async Task<IEnumerable<Feature>> GetContentGridAsync()
    {
        _allFeatures ??= new List<Feature>(AllFeatures());

        await Task.CompletedTask;
        return _allFeatures;
    }
}
