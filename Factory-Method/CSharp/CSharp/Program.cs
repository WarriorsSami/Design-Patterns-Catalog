using System;
using CSharp.Creators;

namespace CSharp
{
    internal static class Program
    {
        private static Dialog DialogBox { get; set; }
        
        public static void Main(string[] args)
        {
            Configure();
            RunBusiness();
        }

        private static void Configure()
        {
            var os = Environment.OSVersion;
            var platform = os.Platform;
            var isWindows = platform is PlatformID.Win32NT or PlatformID.Win32Windows;
            
            if (isWindows)
            {
                DialogBox = new WindowsDialog();
            }
            else
            {
                DialogBox = new HtmlDialog();
            }
        }

        private static void RunBusiness()
        {
            DialogBox.RenderWindow();
        }
    }
}
