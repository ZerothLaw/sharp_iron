using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Security.Policy;

namespace RustAppDomainManager
{
    [Guid("B47320A6-6265-4C34-90AC-3FF2A909686C"), ComVisible(true)]
    public interface ICustomAppDomainManager
    {
        AppDomain GetAppDomain([In]byte[] friendlyName);

        AppDomain CreateAppDomain([In]byte[] name);

        _Assembly LoadAssembly([In]byte[] name);
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

        public AppDomain GetAppDomain([In]byte[] friendlyName)
        {
            string sfriendName = System.Text.Encoding.UTF8.GetString(friendlyName);
            if (mpAppDomains_m.ContainsKey(sfriendName))
            {
                return mpAppDomains_m[sfriendName];
            }
            return null;
        }

        public AppDomain CreateAppDomain([In]byte[] name)
        {
            string sName = System.Text.Encoding.UTF8.GetString(name);
            return CreateDomain(sName, null, null);
        }

        public _Assembly LoadAssembly([In] byte[] name)
        {
            string sName = System.Text.Encoding.UTF8.GetString(name);
            //string name = "System, Version=4.0.0.0, Culture=neutral, PublicKeyToken=b77a5c561934e089, processorArchitecture=MSIL";
            Console.Out.WriteLine("loading name: " + sName);
            var assembly = AppDomain.CurrentDomain.Load(sName);
            return assembly;
        }
    }
}
