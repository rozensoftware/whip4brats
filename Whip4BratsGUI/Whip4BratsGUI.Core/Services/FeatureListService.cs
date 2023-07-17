using System.Resources;
using Whip4BratsGUI.Core.Contracts.Services;
using Whip4BratsGUI.Core.Models;

namespace Whip4BratsGUI.Core.Services;

public class FeatureListService : IFeatureListService
{
    private List<Feature> _allFeatures;
    private static readonly ResourceManager _resource = new("Whip4BratsGUI.Core.Localization.Strings", typeof(FeatureListService).Assembly);

    private static IEnumerable<Feature> AllFeatures()
    {

        return new List<Feature>()
        {
            new Feature()
            {
                Description = _resource.GetString("clock_description"),
                FeatureID = 1,
                FeatureName = _resource.GetString("clock_name"),
                SymbolCode = 0xEC92
            }
        };
    }

    public async Task<IEnumerable<Feature>> GetContentGridAsync()
    {
        _allFeatures ??= new List<Feature>(AllFeatures());

        await Task.CompletedTask;
        return _allFeatures;
    }
}
