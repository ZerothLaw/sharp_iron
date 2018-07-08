using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Security.Policy;

namespace RustAppDomainManager
{
    [Guid("B47320A6-6265-4C34-90AC-3FF2A909686C"), ComVisible(true)]
    public interface ICustomAppDomainManager
    {
        AppDomain GetAppDomain(string friendlyName);

        AppDomain CreateAppDomain(string name);
    }

    [Guid("C1AF7E4A-D78A-47DD-A308-356608657037"), ClassInterface(ClassInterfaceType.None), ComVisible(true), ProgId("RustAppDomainManager.RustAppDomainMgr"),
        ComDefaultInterface(typeof(ICustomAppDomainManager))]
    public sealed class RustAppDomainManager: AppDomainManager, ICustomAppDomainManager
    {
        private Dictionary<string, AppDomain> mpAppDomains_m = new Dictionary<string, AppDomain>();
        public RustAppDomainManager()
        {
            //System.Console.Out.WriteLine("RustAppDomainManager ctor");
        }

        public override void InitializeNewDomain(AppDomainSetup appDomainInfo)
        {
            InitializationFlags = AppDomainManagerInitializationOptions.RegisterWithHost;
        }

        public override AppDomain CreateDomain(string friendlyName, Evidence securityInfo, AppDomainSetup appDomainInfo)
        {
            //Console.Out.WriteLine("CreateDomain "  + friendlyName);
            var apd = base.CreateDomain(friendlyName, securityInfo, appDomainInfo);
            //Console.Out.WriteLine("Domain: " + (apd != null ? "created" : "null"));
            mpAppDomains_m[friendlyName] = apd;
            return apd;
        }

        public AppDomain GetAppDomain(string friendlyName)
        {
            if (mpAppDomains_m.ContainsKey(friendlyName))
            {
                return mpAppDomains_m[friendlyName];
            }
            return null;
        }

        public AppDomain CreateAppDomain(string name)
        {
            return CreateDomain(name, null, null);
        }
    }
}
