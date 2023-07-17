namespace Whip4BratsGUI.Core.Models;
public class Feature
{
    public long FeatureID
    {    
        get; set;
    }

    public string FeatureName
    {    
        get; set;
    }

    public string Description
    {       
        get; set;
    }

    public string SymbolName
    {
        get; set;
    }

    public int SymbolCode
    {
        get; set;
    }

    public char Symbol => (char)SymbolCode;
}
